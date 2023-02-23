use risc0_zkvm::{sha::Digest, Prover, Receipt as Risc0Receipt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Receipt {
    journal: Box<[u8]>,
    seal: Box<[u32]>,
}

#[wasm_bindgen]
impl Receipt {
    #[wasm_bindgen(constructor)]
    pub fn new(journal: Box<[u8]>, seal: Box<[u32]>) -> Self {
        Self {
            journal: journal.into(),
            seal: seal.into(),
        }
    }
    #[wasm_bindgen(getter)]
    pub fn journal(&self) -> Box<[u8]> {
        self.journal.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn seal(&self) -> Box<[u32]> {
        self.seal.clone()
    }
}

#[wasm_bindgen]
pub fn validate_proof(receipt: Receipt, image_id: &[u8]) -> Result<(), JsError> {
    let receipt = Risc0Receipt::new(&receipt.journal, &receipt.seal);
    let image_id: [u8; 32] = image_id.try_into()?;
    let digest = Digest::from(image_id);
    Ok(receipt
        .verify(&digest)
        .map_err(|e| JsError::new(&format!("Failed to validate proof: {}", e)))?)
}

// TODO this is currently blocked by getrandom crate not working in wasm. Investigate r0 lib
// #[wasm_bindgen]
// pub fn generate_proof(code: &[u8], image_id: &[u8], input: &[u32]) -> Result<Receipt, JsError> {
//     let image_id: [u8; 32] = image_id.try_into()?;
//     // TODO this doesn't impl error
//     let mut prover = Prover::new(code, image_id)
//         .map_err(|e| JsError::new(&format!("Failed to validate proof: {}", e)))?;
//     prover.add_input_u32_slice(input);
//     let receipt = prover
//         .run()
//         // TODO this one too
//         .map_err(|e| JsError::new(&format!("Failed to validate proof: {}", e)))?;
//     Ok(Receipt::new(receipt.journal.into(), receipt.seal.into()))
// }
