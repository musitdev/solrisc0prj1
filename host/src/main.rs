// TODO: Update the name of the method loaded by the prover. E.g., if the method is `multiply`, replace `METHOD_NAME_ID` with `MULTIPLY_ID` and replace `METHOD_NAME_PATH` with `MULTIPLY_PATH`
use methods::{MYPRG_ID, MYPRG_PATH};
use risc0_zkvm::Prover;
use sovcore::prover_context::ZK_CONTEXT;
// use risc0_zkvm::serde::{from_slice, to_vec};

//link to the methode source code to be executed in the zk VM.
#[path = "../../methods/smartcontract/src/lib.rs"]
mod method;

fn main() {
    // Make the prover.
    let method_code = std::fs::read(MYPRG_PATH)
        .expect("Method code should be present at the specified path; did you use the correct *_PATH constant?");
    let mut prover = Prover::new(&method_code, MYPRG_ID).expect(
        "Prover should be constructed from valid method source code and corresponding method ID",
    );

    ZK_CONTEXT.lock().unwrap().write(&1);
    ZK_CONTEXT.lock().unwrap().write(&2);
    method::to_execute();

    // Run pre execution for hints context creation.
    //prover.add_input_u32_slice(vec.as_slice());

    // Run prover & generate receipt
    let receipt = prover.run()
        .expect("Code should be provable unless it 1) had an error or 2) overflowed the cycle limit. See `embed_methods_with_options` for information on adjusting maximum cycle count.");

    // Optional: Verify receipt to confirm that recipients will also be able to verify your receipt
    receipt.verify(MYPRG_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );

    // TODO: Implement code for transmitting or serializing the receipt for other parties to verify here
}
