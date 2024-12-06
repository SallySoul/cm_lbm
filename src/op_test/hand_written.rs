use crate::*;

pub fn f_equilibrium(
    directions: &[Vec3; 27],
    density: f32,
    velocity: Vec3,
) -> [f32; 27] {
    let mut result = [0.0; 27];
    for q_i in 0..27 {
        // Calculate equilibrium
        let dir = directions[q_i as usize];
        let dir_u = dir.dot(&velocity);
        let w_i = D3Q27_W[q_i as usize];

        let t1 = 3.0 * dir_u;
        let t2 = 9.0 * dir_u * dir_u;
        let t3 = -(3.0 * velocity.dot(&velocity));

        result[q_i as usize] = w_i * density * (1.0 + t1 + t2 + t3);
    }

    result
}

pub fn moments(directions: &[Vec3; 27], f: &[f32; 27]) -> (f32, Vec3) {
    let mut density = 0.0;
    let mut velocity = Vec3::zero();
    for q_i in 0..27 {
        let q = f[q_i];
        density += q;
        velocity += directions[q_i] * q;
    }
    if density.abs() > 0.00001 {
        velocity /= density;
    } else {
        println!("ERROR");
    }
    (density, velocity)
}
