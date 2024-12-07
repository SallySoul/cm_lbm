use cm_lbm_generated::rust_ops::*;

fn abs_density(f: [f32; 27]) -> f32 {
    f.iter().map(|x| x.abs()).sum()
}

fn abs_density_f64(f: [f64; 27]) -> f64 {
    f.iter().map(|x| x.abs()).sum()
}

fn main() {
    let riv = 0.5;
    let omega = 0.5;
    let density = 1.0;
    let mut f = eq(0.0, 0.0, 0.2, density);
    let mut m = moments(f);
    let mut da = abs_density(f);

    let mut f_f64 = eq_f64(0.0, 0.0, 0.2, density as f64);
    let mut m_f64 = moments_f64(f_f64);
    let mut da_f64 = abs_density_f64(f_f64);

    let mut f_bgk = eq(0.0, 0.0, 0.2, density);
    let mut m_bgk = moments(f_bgk);
    let mut da_bgk = abs_density(f_bgk);

    println!("0, f32 - {} - {:?}", da, m);
    println!("0, bgk - {} - {:?}", da_bgk, m_bgk);
    println!("0, f64 + {} - {:?}\n", da_f64, m_f64);

    for i in 0..1000 {
        f = cm_mrt(f, m.1[0], m.1[1], m.1[2], m.0, riv);
        m = moments(f);
        da = abs_density(f);

        f_bgk = bgk(f_bgk, m_bgk.1[0], m_bgk.1[1], m_bgk.1[2], m_bgk.0, omega);
        m_bgk = moments(f_bgk);
        da_bgk = abs_density(f_bgk);

        f_f64 = cm_mrt_f64(
            f_f64, m_f64.1[0], m_f64.1[1], m_f64.1[2], m_f64.0, riv as f64,
        );
        m_f64 = moments_f64(f_f64);
        da_f64 = abs_density_f64(f_f64);
        println!("{}, f32 - {} - {:?}", i, da, m);
        println!("{}, bgk - {} - {:?}", i, da_bgk, m_bgk);
        println!("{}, f64 + {} - {:?}\n", i, da_f64, m_f64);
    }
}
