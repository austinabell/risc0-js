// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`
use methods::{METHOD_NAME_ELF, METHOD_NAME_ID};
use risc0_zkvm::{
    default_executor_from_elf,
    serde::{from_slice, to_vec},
    ExecutorEnv,
};
use std::fs::File;
use std::io::Write;

fn main() -> anyhow::Result<()> {
    // First, we construct an executor environment
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&88u64).unwrap())
        .add_input(&to_vec(&23u64).unwrap())
        .build()
        .unwrap();

    // TODO: add guest input to the executor environment using
    // ExecutorEnvBuilder::add_input().
    // To access this method, you'll need to use the alternate construction
    // ExecutorEnv::builder(), which creates an ExecutorEnvBuilder. When you're
    // done adding input, call ExecutorEnvBuilder::build().

    // For example:
    // let env = ExecutorEnv::builder().add_input(&vec).build().unwrap();

    // Next, we make an executor, loading the (renamed) ELF binary.
    let mut exec = default_executor_from_elf(env, METHOD_NAME_ELF).unwrap();

    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();

    let c: u64 = from_slice(&receipt.journal).expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );
    assert_eq!(c, 2024);

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(METHOD_NAME_ID).unwrap();

    let receipt_serialized = bincode::serialize(&receipt)?;
    let mut file = File::create("../../public/receipt.bin")?;
    file.write_all(&receipt_serialized)?;

    let mut file = File::create("../../public/method_id.bin")?;
    file.write_all(&bytemuck::cast::<[u32; 8], [u8; 32]>(METHOD_NAME_ID))?;

    Ok(())
}
