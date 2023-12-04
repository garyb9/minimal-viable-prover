// mod consts;
use bls12_381::{Scalar, G1Affine, G1Projective};
use rand::thread_rng;
use ff::Field;

pub const N: u64 = 1 << 17; // 2**17

pub fn generate_srs() -> Vec<G1Projective> {
    let tau_group: Vec<Scalar> = generate_random_tau_group();
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