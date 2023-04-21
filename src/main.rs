extern crate rand;
use rand::thread_rng;
// use curve25519_dalek_ng::scalar::Scalar;
use curve25519_dalek_ng::ristretto::CompressedRistretto;
use std::time::Instant;


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
    let start_time = Instant::now();
    let (proof, committed_value) = generate_range_proof(1037578891);

    // Verify the range proof.
    verify_range_proof(&proof, &committed_value);
    let end_time = Instant::now();
    let duration = end_time - start_time;
    
    println!("It took {} milliseconds to execute.", duration.as_millis()); 
}


// use solana_program::{
//     account_info::{next_account_info, AccountInfo},
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     program_error::ProgramError,
//     pubkey::Pubkey,
// };

// use spl_token::{
//     state::{Account as TokenAccount},
// };

// extern crate rand;
// use rand::thread_rng;
// use curve25519_dalek_ng::ristretto::CompressedRistretto;

// extern crate merlin;
// use merlin::Transcript;

// extern crate bulletproofs;
// use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};

// entrypoint!(process_instruction);

// fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     instruction_data: &[u8],
// ) -> ProgramResult {

//     // Verify the instruction data
//     let value = instruction_data
//         .get(..8)
//         .and_then(|slice| slice.try_into().ok())
//         .map(u64::from_le_bytes)
//         .ok_or(ProgramError::InvalidInstructionData)?;

//     // Get the accounts
//     let accounts_iter = &mut accounts.iter();
//     let payer_account = next_account_info(accounts_iter)?;
//     let recipient_account = next_account_info(accounts_iter)?;
//     let token_account = next_account_info(accounts_iter)?;

//     // Verify the token account is owned by the program
//     if *token_account.owner != *program_id {
//         return Err(ProgramError::IncorrectProgramId);
//     }

//     // Verify the token account is a spl_token account
//     let token_account_data = token_account.data.borrow();
//     let token_account_state =
//         TokenAccount::unpack_unchecked(&token_account_data)
//             .map_err(|_| ProgramError::InvalidAccountData)?;

//     if token_account_state.mint != *recipient_account.key {
//         return Err(ProgramError::InvalidAccountData);
//     }

//     // Verify the range proof
//     let pc_gens = PedersenGens::default();
//     let bp_gens = BulletproofGens::new(64, 1);
//     let secret_value = value;
//     let secret_value1 = curve25519_dalek_ng::scalar::Scalar::random(&mut thread_rng());
//     let binding: curve25519_dalek_ng::scalar::Scalar = secret_value1;
//     let mut prover_transcript = Transcript::new(b"doctest example");
//     let (proof, committed_value) = RangeProof::prove_single(
//         &bp_gens,
//         &pc_gens,
//         &mut prover_transcript,
//         secret_value,
//         &binding,
//         32,
//     ).expect("A real program could handle errors");

//     let mut verifier_transcript = Transcript::new(b"doctest example");
//     if let Err(_) = proof.verify_single(
//         &bp_gens,
//         &pc_gens,
//         &mut verifier_transcript,
//         &committed_value,
//         32
//     ) {
//         return Err(ProgramError::InvalidArgument);
//     }

//     // Transfer the token to the recipient account
//     spl_token::instruction::transfer(
//         program_id,
//         token_account.key,
//         recipient_account.key,
//         payer_account.key,
//         &[],
//         value,
//     )?;

//     Ok(())
// }

// This code is an example Solana program that can be used to transfer tokens and verify a range proof for a given value. The program takes in an instruction data, which contains the value to be transferred, and three accounts: a payer account, a recipient account, and a token account.

// First, the program verifies that the instruction data is valid and retrieves the accounts from the input. It then checks that the token account is owned by the program and is a valid spl_token account.

// The program then generates a range proof for the secret value using Bulletproofs and Pedersen commitments. It creates a prover transcript, which is used to create the proof, and a verifier transcript, which is used to verify the proof later. The range proof is then verified using the verifier transcript, and if it fails, the program returns an error.

// Finally, the program transfers the given value of tokens from the payer account to the recipient account using the spl_token transfer instruction.

// This program can be deployed to a Solana cluster and called using the Solana CLI tools or other client-side applications that interact with the Solana network. It can be useful for ensuring the validity and security of token transfers on a cryptocurrency blockchain.

// There are several ways to get people to use your code for verifying range proofs on a Solana blockchain:

// Integrate the code into a cryptocurrency wallet: You can integrate the code into a cryptocurrency wallet that supports Solana. This would allow users to verify the range proofs for their transactions within the wallet itself.

// Build a web app: You can build a web app that allows users to upload their range proofs and verify them. This would be useful for users who do not have a cryptocurrency wallet or who want to verify transactions from multiple wallets.

// Build a command-line tool: You can build a command-line tool that allows users to verify range proofs on their local machine. This would be useful for developers who want to integrate the verification process into their own applications.

// Open-source the code: By open-sourcing the code, you can encourage other developers to use and contribute to the project. This can help build a community around the project and increase its visibility.

// Overall, creating a web app is just one option for getting people to use your code. Depending on your target audience and use case, other options such as integrating the code into a wallet or building a command-line tool may be more appropriate.


// use solana_program::{
//     account_info::{next_account_info, AccountInfo},
//     entrypoint,
//     entrypoint::ProgramResult,
//     pubkey::Pubkey,
// };

// use rand::thread_rng;
// use curve25519_dalek::scalar::Scalar;
// use curve25519_dalek::ristretto::CompressedRistretto;

// use merlin::Transcript;
// use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};

// entrypoint!(process_instruction);

// fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     _instruction_data: &[u8],
// ) -> ProgramResult {
//     let accounts_iter = &mut accounts.iter();

//     // Get the account for the prover's secret keypair
//     let prover_keypair_account = next_account_info(accounts_iter)?;

//     // Generate a random secret value and binding for the range proof
//     let secret_value = 1037578891u64;
//     let binding = Scalar::random(&mut thread_rng());

//     // Create the Pedersen and Bulletproof generators
//     let pc_gens = PedersenGens::default();
//     let bp_gens = BulletproofGens::new(64, 1);

//     // Create a transcript for the range proof
//     let mut prover_transcript = Transcript::new(b"doctest example");

//     // Create a range proof for the secret value and binding
//     let (proof, committed_value) = RangeProof::prove_single(
//         &bp_gens,
//         &pc_gens,
//         &mut prover_transcript,
//         secret_value,
//         &binding,
//         32,
//     ).expect("Failed to create range proof");

//     // Sign the commitment and the range proof using the prover's keypair
//     let prover_keypair = crate::Keypair::from_account(prover_keypair_account)?;
//     let prover_pubkey = prover_keypair.pubkey();
//     let mut commitment_data = prover_pubkey.to_bytes();
//     commitment_data.extend_from_slice(&committed_value.as_bytes());
//     let commitment_signature = prover_keypair.sign_message(&commitment_data);
//     let proof_signature = prover_keypair.sign_message(&proof.to_bytes());

//     // Verify the range proof
//     let verifier_key = PedersenGens::default().B.compress();
//     let mut verifier_transcript = Transcript::new(b"doctest example");
//     let proof_result = proof.verify_single(
//         &bp_gens,
//         &pc_gens,
//         &mut verifier_transcript,
//         &verifier_key,
//         32,
//     );

//     if proof_result.is_err() {
//         return Err(ProgramError::InvalidArgument);
//     }

//     Ok(())
// }


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


