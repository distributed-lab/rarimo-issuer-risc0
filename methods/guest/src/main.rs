use issuer_core::{CountNullifiersInput, CountNullifiersJournal};
use risc0_zkp::core::hash::sha::Sha256HashSuite;
use risc0_zkp::core::hash::HashSuite;
use risc0_zkp::field::baby_bear::BabyBear;
use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};

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
    let blinder_commitment = Impl::hash_bytes(input.blinder.as_slice())
        .as_bytes()
        .to_vec();

    // Compute document commitment.
    let document_commitment = {
        let mut bytes = input.document_hash.clone();
        bytes.extend(input.blinder.iter());

        Impl::hash_bytes(bytes.as_slice()).as_bytes().to_vec()
    };

    // Since nullifier formula = hash(salt[i] || blinder || document hash)
    // we are preparing 2 and 3 operands.
    let mut nullifier_base = input.blinder;
    nullifier_base.extend(input.document_hash.iter());

    let mut total_duplicates = 0;

    let suite: HashSuite<BabyBear> = Sha256HashSuite::new_suite();
    let hashfn = suite.hashfn.as_ref();

    for (salt, proof) in input.salts.iter().zip(input.merkle_proofs) {
        let nullifier = {
            let mut bytes = salt.clone();
            bytes.extend(nullifier_base.as_slice());

           *Impl::hash_bytes(&bytes)
        };

        let is_verified = proof.verify(&nullifier, &input.merkle_root, hashfn);
        total_duplicates += is_verified as u64;
    }

    let journal = CountNullifiersJournal {
        blinder_commitment,
        document_commitment,
        total_duplicates,
    };

    env::commit(&journal);
}
