
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

    // Generate a random scalar
    let random_scalar = setup::generate_random_scalar();
    println!("Random Scalar: {:?}", random_scalar);

    // Generate a random point on the BLS12-381 curve
    let random_point = setup::generate_random_point_on_curve();
    println!("Random Point on Curve: {:?}", random_point);

    setup::generate_srs();
}
