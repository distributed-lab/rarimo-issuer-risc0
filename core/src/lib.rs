extern crate risc0_zkp;
extern crate serde;

use risc0_zkp::core::{digest::Digest, hash::HashFn};
use risc0_zkp::field::baby_bear::BabyBear;
use serde::{Deserialize, Serialize};

/// Host input to the guest program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountNullifiersInput {
    /// Passport/Document hash.
    pub document_hash: Vec<u8>,
    /// Legacy value - left for compatibility purposes.
    pub blinder: Vec<u8>,
    /// Vector of salts for nullifiers composition.
    pub salts: Vec<Vec<u8>>,
    /// Merkle Root of all nullifiers tree.
    pub merkle_root: Digest,
    /// Merkle branches with index for nullifiers inclusion proofs.
    pub merkle_proofs: Vec<MerkleProof>,
}

/// An inclusion proof for the Merkle Tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    /// Index of the leaf for which inclusion is being proven.
    pub index: u32,
    /// Sibling digests on the path from the root to the leaf.
    /// Does not include the root and the leaf.
    pub digests: Vec<Digest>,
}

impl MerkleProof {
    /// Verify the Merkle inclusion proof against the given leaf and root.
    pub fn verify(&self, leaf: &Digest, root: &Digest, hashfn: &dyn HashFn<BabyBear>) -> bool {
        self.root(leaf, hashfn) == *root
    }

    /// Calculate the root of this branch by iteratively hashing, starting from the leaf.
    pub fn root(&self, leaf: &Digest, hashfn: &dyn HashFn<BabyBear>) -> Digest {
        let mut cur = *leaf;
        let mut cur_index = self.index;
        for sibling in &self.digests {
            cur = if cur_index & 1 == 0 {
                *hashfn.hash_pair(&cur, sibling)
            } else {
                *hashfn.hash_pair(sibling, &cur)
            };
            cur_index >>= 1;
        }
        cur
    }
}

/// Structure for public values in ZKP.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountNullifiersJournal {
    /// Hash of the blinder.
    pub blinder_commitment: Vec<u8>,
    /// Hash of the document and blinder.
    pub document_commitment: Vec<u8>,
    /// Amount of duplicate nullifiers for particular document hash.
    pub total_duplicates: u64,
}
