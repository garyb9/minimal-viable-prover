#![allow(dead_code)]
#![allow(unused_variables)]
mod setup;
mod utils;

use std::path::Path;

fn main() {
    let path: &Path = Path::new("../tau_srs.json");

    if utils::file_exists(path.to_str().unwrap()) {
    } else {
        let tau_srs: Vec<bls12_381::Scalar> = setup::generate_tau_srs();
        let g_srs: Vec<bls12_381::G1Projective> = setup::generate_srs(&tau_srs);
        // // Prover gets the setup
        // let trusted_setup: (Vec<bls12_381::G1Projective>, Vec<bls12_381::Scalar>) =
        //     setup::setup_for_prover();
        // let lagrange_basis: Vec<bls12_381::G1Projective> = trusted_setup.0;
        // let polynomial: Vec<bls12_381::Scalar> = trusted_setup.1;
    }
}
