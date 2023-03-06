use risc0_zkvm::serde::{from_slice, to_vec};
use serde::Deserialize;
use std::sync::LazyLock;
use std::sync::Mutex;

pub type ZkContext = LazyLock<Mutex<crate::prover_context::ProverContext>>;

pub static ZK_CONTEXT: ZkContext =
    LazyLock::new(|| Mutex::new(crate::prover_context::ProverContext::new()));

pub type SendData = Vec<u32>;

use serde::Serialize;

pub struct ProverContext {
    pub stack: Vec<SendData>,
    pop_index: usize,
}

impl ProverContext {
    pub fn new() -> Self {
        println!("ProverContext new");
        ProverContext {
            stack: vec![],
            pop_index: 0,
        }
    }

    pub fn read<T: for<'a> Deserialize<'a>>() -> T {
        let mut context = ZK_CONTEXT.lock().unwrap();
        if context.pop_index >= context.stack.len() {
            panic!("ProverContext try to read where there's no value.");
        }
        context.pop_index += 1;
        let ser = context.stack.get(context.pop_index - 1).unwrap();
        from_slice(&ser).unwrap()
    }

    pub fn write<'a, T>(&mut self, value: &'a T)
    where
        T: Serialize,
    {
        let d: SendData = to_vec(value).unwrap();
        self.stack.push(d);
        println!("write len:{}", self.stack.len());
    }

    pub fn write_data(&mut self, data: SendData) {
        self.stack.push(data);
        println!("write_data len:{}", self.stack.len());
    }

    pub fn write_data_ref(&mut self, data: &[u32]) {
        self.stack.push(data.to_vec());
    }
}
