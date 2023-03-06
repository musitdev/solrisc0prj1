#![no_main]
#![no_std] // std support is experimental, but you can remove this to try it

#[cfg(any(target_os = "zkvm", doc))]
risc0_zkvm::guest::entry!(main);

pub fn main() {
    smartcontract::to_execute()
}
