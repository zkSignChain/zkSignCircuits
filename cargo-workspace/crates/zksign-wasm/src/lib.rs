use wasm_bindgen::prelude::*;
use serde_json;
use zksign_circuits::{mock_prove, SampleInput};

#[wasm_bindgen]
pub fn wasm_generate_proof(input_json: &str) -> Vec<u8> {
    // Accept JSON input for portability between TS and Rust
    let input: SampleInput = serde_json::from_str(input_json).unwrap_or(SampleInput { value: 0 });
    mock_prove(&input)
}

#[wasm_bindgen]
pub fn wasm_verify_proof(_proof: &[u8]) -> bool {
    // Placeholder verification, always true for scaffold
    true
}

// Optional: tests using wasm-bindgen-test can be added
