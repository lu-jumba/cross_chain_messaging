use ethers::prelude::*;
use tiny_keccak::{Hasher, Keccak};

struct MerkleTree {
    leaves: Vec<[u8; 32]>,
}

impl MerkleTree {
    // Create a new Merkle tree from message hashes
    pub fn new(messages: Vec<String>) -> Self {
        let leaves = messages
            .iter()
            .map(|msg| {
                let mut hasher = Keccak::v256();
                let mut output = [0u8; 32];
                hasher.update(msg.as_bytes());
                hasher.finalize(&mut output);
                output
            })
            .collect();
        Self { leaves }
    }

    // Compute the Merkle root
    pub fn compute_root(&self) -> [u8; 32] {
        // Implement Merkle root computation (simplified example)
        self.leaves[0] // For demonstration, return the first leaf (replace with actual root calculation)
    }

    // Generate a proof for a specific message
    pub fn generate_proof(&self, index: usize) -> Vec<[u8; 32]> {
        // Generate Merkle proof for the leaf at the given index (simplified)
        vec![self.leaves[index]]
    }

   

}
