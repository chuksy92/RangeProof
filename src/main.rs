use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use rand::thread_rng;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::CompressedRistretto;

use merlin::Transcript;
use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // Get the account for the prover's secret keypair
    let prover_keypair_account = next_account_info(accounts_iter)?;

    // Generate a random secret value and binding for the range proof
    let secret_value = 1037578891u64;
    let binding = Scalar::random(&mut thread_rng());

    // Create the Pedersen and Bulletproof generators
    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 1);

    // Create a transcript for the range proof
    let mut prover_transcript = Transcript::new(b"doctest example");

    // Create a range proof for the secret value and binding
    let (proof, committed_value) = RangeProof::prove_single(
        &bp_gens,
        &pc_gens,
        &mut prover_transcript,
        secret_value,
        &binding,
        32,
    ).expect("Failed to create range proof");

    // Sign the commitment and the range proof using the prover's keypair
    let prover_keypair = crate::Keypair::from_account(prover_keypair_account)?;
    let prover_pubkey = prover_keypair.pubkey();
    let mut commitment_data = prover_pubkey.to_bytes();
    commitment_data.extend_from_slice(&committed_value.as_bytes());
    let commitment_signature = prover_keypair.sign_message(&commitment_data);
    let proof_signature = prover_keypair.sign_message(&proof.to_bytes());

    // Verify the range proof
    let verifier_key = PedersenGens::default().B.compress();
    let mut verifier_transcript = Transcript::new(b"doctest example");
    let proof_result = proof.verify_single(
        &bp_gens,
        &pc_gens,
        &mut verifier_transcript,
        &verifier_key,
        32,
    );

    if proof_result.is_err() {
        return Err(ProgramError::InvalidArgument);
    }

    Ok(())
}


// fn main() {
//     // Set up the program ID and accounts for testing
//     let program_id = Pubkey::new_unique();
//     let prover_keypair = Keypair::new();

//     let mut accounts = vec![];

//     let prover_keypair_account = AccountInfo::new(
//         &prover_keypair.pubkey(),
//         false,
//         true,
//         &mut vec![],
//         &program_id,
//         false,
//         0,
//     );
//     accounts.push(prover_keypair_account);

//     // Call the program's entrypoint function
//     let result = process_instruction(
//         &program_id,
//         &accounts,
//         &[],
//     );

//     // Handle the program result
//     match result {
//         Ok(_) => println!("Program executed successfully!"),
//         Err(err) => println!("Program failed with error: {:?}", err),
//     }
// }



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


