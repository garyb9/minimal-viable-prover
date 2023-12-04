// mod consts;
use bls12_381::{Scalar, G1Affine, G1Projective};
use rand::thread_rng;
use ff::Field;

pub const N: u64 = 1 << 17; // 2**17

pub fn generate_srs() {
    let random_tau: Scalar = generate_random_scalar();
    let mut tau_group: Vec<Scalar> = Vec::new();

    // Iterate from 0 to 2*N - 1
    let size = 2*N-1;
    for i in 0..(2 * N) {
        let curr = random_tau.pow(&[i, 0, 0, 0]);
        tau_group.push(curr);
        println!("{:?}/{:?}: Adding (tau**{:?})={:?} to tau group vector", i, size, i, curr);
    }
    println!("Done generating Tau Group");
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