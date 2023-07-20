use risc0_zkvm::receipt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SessionReceipt(receipt::SessionReceipt);

#[wasm_bindgen]
impl SessionReceipt {
    pub fn bincode_deserialize(buffer: &[u8]) -> Result<SessionReceipt, JsError> {
        let receipt = bincode::deserialize(buffer)
            .map_err(|e| JsError::new(&format!("Failed to deserialize receipt: {e}")))?;
        Ok(SessionReceipt(receipt))
    }

    #[wasm_bindgen(getter)]
    pub fn journal(&self) -> Vec<u8> {
        self.0.journal.clone()
    }

    pub fn validate(&self, image_id: &[u8]) -> Result<(), JsError> {
        let image_id: [u8; 32] = image_id.try_into()?;
        self.0
            .verify(image_id)
            .map_err(|e| JsError::new(&format!("Failed to validate proof: {e}")))
    }
}
