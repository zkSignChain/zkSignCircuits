//! zksign-circuits
//!
//! Example circuit harness and test inputs. This crate is intentionally minimal —
//! real circuits should add `halo2` or `arkworks` as needed behind feature flags.

use serde::{Deserialize, Serialize};

/// A sample public input for an example circuit
#[derive(Debug, Serialize, Deserialize)]
pub struct SampleInput {
    pub value: u64,
}

/// Mock `prove` function for local testing. Replace with real prover integration.
pub fn mock_prove(input: &SampleInput) -> Vec<u8> {
    // In a real implementation, you'd call into halo2 or arkworks
    // to construct a proof for `input`. This returns a deterministic placeholder.
    format!("proof_for:{}", input.value).into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_prove() {
        let inp = SampleInput { value: 42 };
        let p = mock_prove(&inp);
        assert!(p.len() > 0);
    }
}
