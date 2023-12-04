use bls12_381::{G1Affine, G1Projective, Scalar};
use ff::Field;
use rand::thread_rng;

pub const N: u64 = 1 << 17; // 2**17

pub fn setup_for_prover() -> (Vec<G1Projective>, Vec<Scalar>) {
    let tau_group: Vec<bls12_381::Scalar> = generate_tau_srs();
    let g_srs: Vec<bls12_381::G1Projective> = generate_srs(&tau_group);
    let lagrange_basis: Vec<bls12_381::G1Projective> =
        convert_to_lagrange_basis(&tau_group, &g_srs);
    let polynomial: Vec<bls12_381::Scalar> = generate_random_polynomial();
    (lagrange_basis, polynomial)
}

pub fn generate_random_polynomial() -> Vec<Scalar> {
    let mut polynomial: Vec<Scalar> = Vec::new();

    // Iterate from 0 to 2*N - 1
    let size: u64 = 2 * N - 1;
    for i in 0..(2 * N) {
        let curr: Scalar = generate_random_scalar();
        polynomial.push(curr);
        println!(
            "{:?}/{:?}: Adding (C_{:?})={:?} coefficient to polynomial in Fr",
            i, size, i, curr
        );
    }
    println!("Done generating Polynomial");
    polynomial
}

pub fn convert_to_lagrange_basis(
    tau_group: &Vec<Scalar>,
    g_srs: &Vec<G1Projective>,
) -> Vec<G1Projective> {
    let mut lagrange_basis: Vec<G1Projective> = Vec::new();

    for (i, &x) in tau_group.iter().enumerate() {
        let mut numerator: G1Projective = G1Projective::identity();
        let mut denominator: Scalar = Scalar::one();

        for (j, &x_j) in tau_group.iter().enumerate() {
            if i != j {
                let delta: Scalar = x - x_j;
                numerator += g_srs[j] * delta;
                denominator *= x - x_j;
            }
        }

        let lagrange_coefficient: G1Projective = numerator * denominator.invert().unwrap();
        lagrange_basis.push(lagrange_coefficient);
        println!(
            "Lagrange Coefficient {:?} computed as: {:?}",
            i, lagrange_coefficient
        );
    }

    lagrange_basis
}

pub fn generate_srs(tau_srs: &Vec<Scalar>) -> Vec<G1Projective> {
    // let tau_group: Vec<Scalar> = generate_tau_srs();
    let random_group_element: G1Projective = generate_random_point_on_curve();
    let mut g_srs: Vec<G1Projective> = Vec::new();

    // Iterate from 0 to 2*N - 1
    let size: u64 = 2 * N - 1;
    for (i, &scalar) in tau_srs.iter().enumerate() {
        let curr: G1Projective = random_group_element * scalar;
        g_srs.push(curr);
        println!(
            "{:?}/{:?}: Adding G_{:?} = {:?} to G_SRS vector",
            i, size, i, curr
        );
    }
    println!("Done generating G_SRS");
    g_srs
}

pub fn generate_tau_srs() -> Vec<Scalar> {
    let tau: Scalar = generate_random_scalar();
    let mut tau_srs: Vec<Scalar> = Vec::new();

    // Iterate from 0 to 2*N - 1
    let size: u64 = 2 * N - 1;
    for i in 0..(size + 1) {
        let curr: Scalar = tau.pow(&[i, 0, 0, 0]);
        tau_srs.push(curr);
        println!(
            "{:?}/{:?}: Adding tau^{:?} = {:?} to tau srs vector",
            i, size, i, curr
        );
    }
    println!("Done generating Tau SRS");
    tau_srs
}

pub fn generate_random_scalar() -> Scalar {
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    Scalar::random(&mut rng)
}

pub fn generate_random_point_on_curve() -> G1Projective {
    let scalar: Scalar = generate_random_scalar();
    let base_point: G1Affine = G1Affine::generator();
    base_point * scalar
}
