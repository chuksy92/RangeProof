
// extern crate rand;
use rand::thread_rng;

// extern crate curve25519_dalek_ng;
use curve25519_dalek_ng::scalar::Scalar;

extern crate merlin;
use merlin::Transcript;

extern crate bulletproofs;
use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};


fn main() {
    
    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 1);
    let secret_value = 1037578891u64;
    let blinding = Scalar::random(&mut thread_rng());
 


    let mut prover_transcript = Transcript::new(b"doctest example");

    // Create a 32-bit rangeproof.
    let (proof, committed_value) = RangeProof::prove_single(
        &bp_gens,
        &pc_gens,
        &mut prover_transcript,
        secret_value,
        curve25519_dalek_ng::scalar::Scalar::random(&mut thread_rng()),
        32,
    ).expect("A real program could handle errors");

    // Verification requires a transcript with identical initial state:
    let mut verifier_transcript = Transcript::new(b"doctest example");
    assert!(
        proof
            .verify_single(&bp_gens, &pc_gens, &mut verifier_transcript, &committed_value, 32)
            .is_ok()
    );
}


// use rand::{thread_rng, Rng};
// use rand::rngs::ThreadRng;
// use curve25519_dalek_ng::{ristretto::RistrettoPoint, scalar::Scalar};
// use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};


