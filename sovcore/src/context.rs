#[cfg(feature = "prove")]
use crate::prover_context::ProverContext;
use core::fmt::Debug;
use serde::{Deserialize, Serialize};

/// Read private data from the host and deserializes it.
#[cfg(feature = "prove")]
pub fn read<T: for<'a> Deserialize<'a>>() -> T {
    ProverContext::read()
}

#[cfg(feature = "prove")]
pub fn commit<T: Serialize + Debug>(data: &T) {
    println!("Commit data:{:?}", data);
}

#[cfg(any(target_os = "zkvm", doc))]
use risc0_zkvm::guest::env;
#[cfg(any(target_os = "zkvm", doc))]
pub fn read<T: for<'a> Deserialize<'a>>() -> T {
    env::read()
}
#[cfg(any(target_os = "zkvm", doc))]
pub fn write_out<T: Serialize>(data: &T) {
    env::write(data)
}
#[cfg(any(target_os = "zkvm", doc))]
pub fn log(msg: &str) {
    env::log(msg)
}

#[cfg(any(target_os = "zkvm", doc))]
pub fn commit<T: Serialize>(data: &T) {
    env::commit(data)
}
