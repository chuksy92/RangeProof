
extern crate rand;
use rand::thread_rng;
// use curve25519_dalek_ng::scalar::Scalar;
use curve25519_dalek_ng::ristretto::CompressedRistretto;


extern crate merlin;
use merlin::Transcript;

extern crate bulletproofs;
use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};


pub fn generate_range_proof(
    secret_value: u64
) -> (RangeProof, CompressedRistretto) {
    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 1);
    // let secret_value1 = 1037578891u64;
    let secret_value1 = curve25519_dalek_ng::scalar::Scalar::random(&mut thread_rng());
    let binding: curve25519_dalek_ng::scalar::Scalar = secret_value1;
    let mut prover_transcript = Transcript::new(b"doctest example");

     // Create a 32-bit rangeproof.
    let (proof, committed_value) = RangeProof::prove_single(
        &bp_gens,
        &pc_gens,
        &mut prover_transcript,
        secret_value,
        &binding,
        32,
    ).expect("A real program could handle errors");

    (proof, committed_value)
}

pub fn verify_range_proof(
    proof: &RangeProof,
    committed_value: &CompressedRistretto
){
    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 1);
   // Verification requires a transcript with identical initial state:
    let mut verifier_transcript = Transcript::new(b"doctest example");
   if let Err(error) = proof.verify_single(&bp_gens, &pc_gens, &mut verifier_transcript, &committed_value, 32) {
        println!("Proof verification failed: {}", error);
    } else {
        println!("Proof verified successfully");
    }
}


fn main() {
    // Generate a range proof for the secret value 1037578891, with 32 bits of precision.
    let (proof, committed_value) = generate_range_proof(1037578891);

    // Verify the range proof.
    verify_range_proof(&proof, &committed_value); 
}




// fn main() {

    
//     let pc_gens = PedersenGens::default();
//     let bp_gens = BulletproofGens::new(64, 1);
//     let secret_value1 = 1037578891u64;
//     let secret_value = curve25519_dalek_ng::scalar::Scalar::random(&mut thread_rng());
//     let secret_value_ng: curve25519_dalek_ng::scalar::Scalar = secret_value;

//     let _blinding = Scalar::random(&mut thread_rng());
 


//     let mut prover_transcript = Transcript::new(b"doctest exampl");

//     // Create a 32-bit rangeproof.
//     let (proof, committed_value) = RangeProof::prove_single(
//         &bp_gens,.p-0
//         &pc_gens,
//         &mut prover_transcript,
//         secret_value1,
//         &secret_value_ng,
//         32,
//     ).expect("A real program could handle errors");

//     // Verification requires a transcript with identical initial state:
//     let mut verifier_transcript = Transcript::new(b"doctest example");
//    if let Err(error) = proof.verify_single(&bp_gens, &pc_gens, &mut verifier_transcript, &committed_value, 32) {
//         println!("Proof verification failed: {}", error);
//     } else {
//         println!("Proof verified successfully");
//     }

// }


