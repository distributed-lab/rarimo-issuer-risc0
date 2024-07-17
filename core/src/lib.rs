mod utils;

use crate::utils::*;

use risc0_zkp::core::digest::Digest;
use risc0_zkp::core::hash::sha::{Impl, Sha256};

use hex;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Host input to the guest program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountNullifiersInput {
    /// Passport/Document hash.
    #[serde(
        serialize_with = "serialize_digest",
        deserialize_with = "deserialize_digest"
    )]
    pub document_hash: Digest,
    /// Legacy value - left for compatibility purposes.
    #[serde(
        serialize_with = "serialize_hex_string",
        deserialize_with = "deserialize_hex_string"
    )]
    pub blinder: Vec<u8>,
    /// Vector of salts for nullifiers composition.
    #[serde(
        serialize_with = "serialize_hex_string_vec",
        deserialize_with = "deserialize_hex_string_vec"
    )]
    pub salts: Vec<Vec<u8>>,
    /// Merkle Root of all nullifiers tree.
    #[serde(
        serialize_with = "serialize_digest",
        deserialize_with = "deserialize_digest"
    )]
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
    #[serde(
        serialize_with = "serialize_digest_vec",
        deserialize_with = "deserialize_digest_vec"
    )]
    pub branch: Vec<Digest>,
}

impl MerkleProof {
    /// Verify the Merkle inclusion proof against the given leaf and root.
    pub fn verify(&self, leaf: &Digest, root: &Digest) -> bool {
        self.root(leaf) == *root
    }

    /// Calculate the root of this branch by iteratively hashing, starting from the leaf.
    pub fn root(&self, leaf: &Digest) -> Digest {
        let mut current_digest = *leaf;
        let mut current_index = self.index;

        for sibling in &self.branch {
            let preimage = match current_index & 1 {
                0 => [current_digest.as_bytes(), sibling.as_bytes()].concat(),
                1 => [sibling.as_bytes(), current_digest.as_bytes()].concat(),
                _ => unreachable!(),
            };

            current_digest = *Impl::hash_bytes(&preimage);
            current_index >>= 1;
        }

        current_digest
    }
}

/// Structure for public values in ZKP.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountNullifiersJournal {
    /// Hash of the blinder.
    #[serde(
        serialize_with = "serialize_digest",
        deserialize_with = "deserialize_digest"
    )]
    pub blinder_commitment: Digest,
    /// Hash of the document and blinder.
    #[serde(
        serialize_with = "serialize_digest",
        deserialize_with = "deserialize_digest"
    )]
    pub document_commitment: Digest,
    /// Amount of duplicate nullifiers for particular document hash.
    pub total_duplicates: u64,
}

impl fmt::Display for CountNullifiersJournal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n  blinder commitment: {},\n  document commitment: {},\n  total duplicates: {}\n",
            hex::encode(&self.blinder_commitment.as_bytes()),
            hex::encode(&self.document_commitment.as_bytes()),
            self.total_duplicates
        )
    }
}
