// mod consts;
use bls12_381::{Scalar, G1Affine, G1Projective};
use rand::thread_rng;
use ff::Field;

pub const N: u64 = 1 << 17; // 2**17

pub fn convert_to_lagrange_basis(tau_group: &Vec<Scalar>, g_srs: &Vec<G1Projective>) -> Vec<G1Projective> {
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
        println!("Lagrange Coefficient {:?} computed as: {:?}", i, lagrange_coefficient);
    }

    lagrange_basis
}

pub fn generate_srs(tau_group: &Vec<Scalar>) -> Vec<G1Projective> {
    // let tau_group: Vec<Scalar> = generate_random_tau_group();
    let random_group_element: G1Projective = generate_random_point_on_curve();
    let mut g_srs: Vec<G1Projective> = Vec::new();

    // Iterate from 0 to 2*N - 1
    let size: u64 = 2*N-1;
    for (i, &scalar) in tau_group.iter().enumerate() {
        let curr: G1Projective = random_group_element * scalar;
        g_srs.push(curr);
        println!("{:?}/{:?}: Adding (G_{:?})={:?} to [G]_SRS vector", i, size, i, curr);
    }
    println!("Done generating [G]_SRS");
    g_srs
}

pub fn generate_random_tau_group() -> Vec<Scalar> {
    let random_tau: Scalar = generate_random_scalar();
    let mut tau_group: Vec<Scalar> = Vec::new();

    // Iterate from 0 to 2*N - 1
    let size = 2*N-1;
    for i in 0..(2 * N) {
        let curr = random_tau.pow(&[i, 0, 0, 0]);
        tau_group.push(curr);
        println!("{:?}/{:?}: Adding (tau^{:?})={:?} to tau group vector", i, size, i, curr);
    }
    println!("Done generating Tau Group");
    tau_group
}


pub fn generate_random_scalar() -> Scalar {
    let mut rng = thread_rng();
    Scalar::random(&mut rng)
}

pub fn generate_random_point_on_curve() -> G1Projective {
    let scalar = generate_random_scalar();
    let base_point = G1Affine::generator();
    base_point * scalar
}