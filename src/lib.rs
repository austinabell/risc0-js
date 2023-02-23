use risc0_zkvm::{sha::Digest, Receipt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn validate_proof(journal: &[u8], seal: &[u32], image_id: &[u8]) -> Result<(), JsError> {
    let receipt: Receipt = Receipt::new(journal, seal);
    let image_id: [u8; 32] = image_id.try_into()?;
    let digest = Digest::from(image_id);
    Ok(receipt
        .verify(&digest)
        .map_err(|e| JsError::new(&format!("Failed to validate proof: {}", e)))?)
}
