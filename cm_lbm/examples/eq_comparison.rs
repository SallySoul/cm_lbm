use cm_lbm_generated::rust_ops::*;

fn abs_density(f: [f32; 27]) -> f32 {
    f.iter().map(|x| x.abs()).sum()
}

fn abs_density_f64(f: [f64; 27]) -> f64 {
    f.iter().map(|x| x.abs()).sum()
}

fn main() {
    let mut f = eq(0.0, 0.0, 0.1, 1.0);
    let mut m = moments(f);
    let mut da = abs_density(f);

    let mut f_f64 = eq_f64(0.0, 0.0, 0.1, 1.0);
    let mut m_f64 = moments_f64(f_f64);
    let mut da_f64 = abs_density_f64(f_f64);

    println!("0, f32 - {} - {:?} -- {:?}", da, m, f);
    println!("0, f64 + {} - {:?} -- {:?}", da_f64, m_f64, f_f64);

    for i in 0..40 {
        f = cm_mrt(f, m.1[0], m.1[1], m.1[2], m.0, 0.6);
        m = moments(f);
        da = abs_density(f);

        f_f64 =
            cm_mrt_f64(f_f64, m_f64.1[0], m_f64.1[1], m_f64.1[2], m_f64.0, 0.6);
        m_f64 = moments_f64(f_f64);
        da_f64 = abs_density_f64(f_f64);
        println!("{}, f32 - {} - {:?} -- {:?}", i, da, m, f);
        println!("{}, f64 + {} - {:?} -- {:?}", i, da_f64, m_f64, f_f64);
    }
}
