use cm_lbm_generated::rust_ops::*;

fn abs_density(f: [f32; 27]) -> f32 {
    f.iter().map(|x| x.abs()).sum()
}

fn main() {
    let mut f = eq(0.0, 0.0, 0.1, 1.0);
    let mut m = moments(f);
    let mut da = abs_density(f);
   println!("0 - {} - {:?} -- {:?}", da, m, f);


    for i in 0..20 {
        f = cm_mrt(f, m.1[0], m.1[1], m.1[2], m.0, 0.6);
        m = moments(f);
        da = abs_density(f);
        println!("{} - {} - {:?} -- {:?}",i, da, m, f);
    }
}
