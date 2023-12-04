
#![allow(dead_code)]
#![allow(unused_variables)]
mod setup;

fn main() {
    // Prover gets the setup
    let for_prover: (Vec<bls12_381::G1Projective>, Vec<bls12_381::Scalar>) = setup::setup_for_prover();
    let lagrange_basis: Vec<bls12_381::G1Projective> = for_prover.0;
    let polynomial: Vec<bls12_381::Scalar> = for_prover.1;

    // // Prover generates commitment and response
    // let (commitment, response) = prover(secret);

    // // Verifier checks the proof
    // let proof_verified = verifier(commitment, response);

    // // Print the result
    // if proof_verified {
    //     println!("Proof verified: Prover knows the secret.");
    // } else {
    //     println!("Proof not verified: Prover does not know the secret.");
    // }    
    
}
