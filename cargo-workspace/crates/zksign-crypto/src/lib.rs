//! zksign-crypto
//!
//! Minimal crypto helpers (SHA256 / Blake2s wrappers and a tiny Merkle helper)

use sha2::{Digest, Sha256};
use blake2::Blake2s256;

/// Compute SHA256 digest of bytes
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let res = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&res);
    out
}

/// Compute Blake2s256 digest of bytes
pub fn blake2s(data: &[u8]) -> [u8; 32] {
    let mut hasher = Blake2s256::new();
    hasher.update(data);
    let res = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&res);
    out
}

/// Simple Merkle root for a slice of leaves using SHA256
pub fn merkle_root(leaves: &[Vec<u8>]) -> Option<[u8; 32]> {
    if leaves.is_empty() {
        return None;
    }
    let mut nodes: Vec<[u8; 32]> = leaves.iter().map(|l| sha256(l)).collect();
    while nodes.len() > 1 {
        let mut next = vec![];
        for pair in nodes.chunks(2) {
            if pair.len() == 2 {
                let mut concat = vec![];
                concat.extend_from_slice(&pair[0]);
                concat.extend_from_slice(&pair[1]);
                next.push(sha256(&concat));
            } else {
                // duplicate last
                let mut concat = vec![];
                concat.extend_from_slice(&pair[0]);
                concat.extend_from_slice(&pair[0]);
                next.push(sha256(&concat));
            }
        }
        nodes = next;
    }
    nodes.into_iter().next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha_and_merkle() {
        let a = b"hello".to_vec();
        let b = b"world".to_vec();
        let root = merkle_root(&[a, b]).unwrap();
        assert_eq!(root.len(), 32);
    }
}
