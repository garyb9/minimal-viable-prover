
#![allow(dead_code)]
#![allow(unused_variables)]
mod setup;

fn main() {
    // // Prover knows the secret
    // let secret = 7;

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

    println!("N = {:?}", setup::N);

    let tau_group: Vec<bls12_381::Scalar> = setup::generate_random_tau_group();
    let g_srs: Vec<bls12_381::G1Projective> = setup::generate_srs(&tau_group);
    let lagrange_basis: Vec<bls12_381::G1Projective> = setup::convert_to_lagrange_basis(&tau_group, &g_srs);
}
