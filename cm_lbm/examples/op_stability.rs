use cm_lbm_generated::rust_ops::*;

fn main() {
    let mut f_bgk = eq(0.1, 0.2, 0.3, 2.0);
    let mut m_bgk = moments(f_bgk);
    let mut f_mrt = eq(0.1, 0.2, 0.3, 2.0);
    let mut m_mrt = moments(f_mrt);
    let mut f_cm_mrt = eq(0.1, 0.2, 0.3, 2.0);
    let mut m_cm_mrt = moments(f_cm_mrt);

    println!("i: 0");
    println!("  - bgk: {:?} -- {:?}", m_bgk, f_bgk);
    println!("  - mrt: {:?} -- {:?}", m_mrt, f_mrt);
    println!("  - cm_mrt: {:?} -- {:?}", m_cm_mrt, f_cm_mrt);

    let omega = 0.5;
    let riv = 0.7;
    for i in 0..10 {
        f_bgk = bgk(f_bgk, m_bgk.1[0], m_bgk.1[1], m_bgk.1[2], m_bgk.0, omega);
        f_mrt = mrt(f_mrt, m_mrt.1[0], m_mrt.1[1], m_mrt.1[2], m_mrt.0, riv);
        f_cm_mrt = cm_mrt(
            f_cm_mrt,
            m_cm_mrt.1[0],
            m_cm_mrt.1[1],
            m_cm_mrt.1[2],
            m_cm_mrt.0,
            riv,
        );
        m_bgk = moments(f_bgk);
        m_mrt = moments(f_mrt);
        m_cm_mrt = moments(f_cm_mrt);
        println!("i: {}", i);
        println!("  - bgk: {:?} -- {:?}", m_bgk, f_bgk);
        println!("  - mrt: {:?} -- {:?}", m_mrt, f_mrt);
        println!("  - cm_mrt: {:?} -- {:?}", m_cm_mrt, f_cm_mrt);
    }
}
