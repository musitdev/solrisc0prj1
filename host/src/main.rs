use methods::{MYPRG_ID, MYPRG_PATH};
use risc0_zkvm::serde::from_slice;
use risc0_zkvm::Prover;
use serde::{Deserialize, Serialize};
use sovcore::prover_context::ZK_CONTEXT;

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

    ZK_CONTEXT.lock().unwrap().write(&3);
    ZK_CONTEXT.lock().unwrap().write(&2);
    ZK_CONTEXT.lock().unwrap().write(&4);

    method::to_execute();

    // Run pre execution for hints context creation.
    {
        let context = ZK_CONTEXT.lock().unwrap();
        context
            .stack
            .iter()
            .for_each(|d| prover.add_input_u32_slice(d.as_slice()));
    }

    // Run prover & generate receipt
    let receipt = prover.run()
        .expect("Code should be provable unless it 1) had an error or 2) overflowed the cycle limit. See `embed_methods_with_options` for information on adjusting maximum cycle count.");

    // Extract journal of receipt (i.e. output c, where c = a * b)
    let c: Commit = from_slice(&receipt.journal).expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );
    println!("Commited journal:{c:?}",);

    // Optional: Verify receipt to confirm that recipients will also be able to verify your receipt
    receipt.verify(MYPRG_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );

    // TODO: Implement code for transmitting or serializing the receipt for other parties to verify here
}

#[derive(Debug, Serialize, Deserialize)]
struct Commit {
    sorted_vec: [u32; 3],
    hashtable: [u32; 10], //none is 0.
}
