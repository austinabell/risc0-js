use methods::{METHOD_NAME_ELF, METHOD_NAME_ID};
use risc0_zkvm::{
    default_prover,
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

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, METHOD_NAME_ELF).unwrap();
   
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
