use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};

use issuer_core::{CountNullifiersInput, CountNullifiersJournal};

fn main() {
    // Read the host input from execution environment.
    let input: CountNullifiersInput = env::read();

    // Input validation.
    assert_eq!(
        input.merkle_proofs.len(),
        input.salts.len(),
        "For each Merkle Proof shall be separate salt"
    );

    // Compute blinder commitment.
    let blinder_commitment = *Impl::hash_bytes(&input.blinder);

    // Compute document commitment.
    let document_commitment = {
        let preimage = [input.document_hash.as_bytes(), &input.blinder].concat();
        *Impl::hash_bytes(&preimage)
    };

    // Since nullifier formula is `hash(salt[i] || blinder || document hash)`
    // we are preparing 2 and 3 operands.
    let nullifier_base = [&input.blinder, input.document_hash.as_bytes()].concat();

    // Compute total duplicates.
    let total_duplicates = input
        .salts
        .iter()
        .zip(&input.merkle_proofs)
        .map(|(salt, proof)| {
            let nullifier = {
                let preimage = [&salt[..], &nullifier_base].concat();
                *Impl::hash_bytes(&preimage)
            };

            proof.verify(&nullifier, &input.merkle_root) as u64
        })
        .sum();

    let journal = CountNullifiersJournal {
        blinder_commitment,
        document_commitment,
        total_duplicates,
    };

    env::commit(&journal);
}
