use risc0_zkvm::serde::{from_slice, to_vec};
use serde::Deserialize;
use std::sync::LazyLock;
use std::sync::Mutex;

pub type ZkContext = LazyLock<Mutex<crate::prover_context::ProverContext>>;

pub static ZK_CONTEXT: ZkContext =
    LazyLock::new(|| Mutex::new(crate::prover_context::ProverContext { stack: vec![] }));

pub type SendData = Vec<u32>;

use serde::Serialize;

pub struct ProverContext {
    pub(crate) stack: Vec<SendData>,
}

impl ProverContext {
    pub fn read<T: for<'a> Deserialize<'a>>() -> T {
        let ser = ZK_CONTEXT.lock().unwrap().stack.pop().unwrap();
        from_slice(&ser).unwrap()
    }

    pub fn write<'a, T>(&mut self, value: &'a T)
    where
        T: Serialize,
    {
        let d: SendData = to_vec(value).unwrap();
        self.stack.push(d);
    }

    pub fn write_data(&mut self, data: SendData) {
        self.stack.push(data);
    }

    pub fn write_data_ref(&mut self, data: &[u32]) {
        self.stack.push(data.to_vec());
    }
}
