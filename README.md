# RangeProof

This is Rust code that generates and verifies a range proof using the Bulletproofs library. 
Range proofs are a way to prove that a secret value falls within a certain range, without revealing the actual value.

The generate_range_proof function takes in a secret value as a 64-bit unsigned integer, creates a Pedersen commitment to that value, and then generates a range proof for that commitment using Bulletproofs. The range proof has 32 bits of precision, meaning it can prove that the value falls within a range of $[0, 2^{32})$. 
The function returns the range proof and the compressed commitment.

The verify_range_proof function takes in the range proof and the compressed commitment, and verifies that the proof is valid for the given commitment using the Bulletproofs library. 
If the proof is valid, it prints "Proof verified successfully". 
If the proof is invalid, it prints an error message.

In the main function, generate_range_proof is called with the secret value 1037578891, and the resulting range proof and commitment are passed to verify_range_proof for verification.
