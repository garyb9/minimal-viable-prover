#![allow(dead_code)]
#![allow(unused_variables)]
mod setup;
mod utils;
use std::{fs, path::Path, u64};

use bls12_381::Scalar;

fn main() {
    let tau_srs_path: &Path = Path::new("tau_srs.txt");
    let tau_srs_filename: &str = tau_srs_path.to_str().unwrap();

    if utils::file_exists(tau_srs_filename) {
        // Read the contents of the file into a string
        let contents = fs::read_to_string(tau_srs_filename).expect("Unable to read from file");

        // Create a vector of Scalars from the file contents
        let tau_srs: Vec<Scalar> = contents
            .lines()
            .map(|line| {
                let hex_string: &str = line.trim_start_matches("0x");
                let u64hex = u64::from_str_radix(&hex_string[0..16], 16).unwrap();
                Scalar::from(u64hex)
            })
            .collect();

        // Print the vector of Scalars
        println!("Scalar vector: {:?}", tau_srs);
    } else {
        // let mut file = File::create(tau_srs_filename).unwrap();
        let tau_srs: Vec<bls12_381::Scalar> = setup::generate_tau_srs();

        let data: String = tau_srs
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n");
        fs::write(tau_srs_filename, data).expect("Unable to write file");

        // let g_srs: Vec<bls12_381::G1Projective> = setup::generate_srs(&tau_srs);

        // // Prover gets the setup
        // let trusted_setup: (Vec<bls12_381::G1Projective>, Vec<bls12_381::Scalar>) =
        //     setup::setup_for_prover();
        // let lagrange_basis: Vec<bls12_381::G1Projective> = trusted_setup.0;
        // let polynomial: Vec<bls12_381::Scalar> = trusted_setup.1;
    }
}
