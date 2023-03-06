#[cfg(feature = "prove")]
use crate::prover_context::ProverContext;
use serde::Deserialize;

/// Read private data from the host and deserializes it.
#[cfg(feature = "prove")]
pub fn read<T: for<'a> Deserialize<'a>>() -> T {
    ProverContext::read()
}

#[cfg(any(target_os = "zkvm", doc))]
use risc0_zkvm::guest::env;
#[cfg(any(target_os = "zkvm", doc))]
pub fn read<T: for<'a> Deserialize<'a>>() -> T {
    env::read()
}

/// Read private data from the host and deserializes it.
#[cfg(feature = "zk")]
pub fn readzk<T: for<'a> Deserialize<'a>>() -> T {
    todo!()
}
