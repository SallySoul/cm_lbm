pub fn wgsl_moments() -> &'static str {
    &"
fn moments(index: i32) {
    let base = index * 27; 
    let f0 = distributions[base + 0];
    let f1 = distributions[base + 1];
    let f2 = distributions[base + 2];
    let f3 = distributions[base + 3];
    let f4 = distributions[base + 4];
    let f5 = distributions[base + 5];
    let f6 = distributions[base + 6];
    let f7 = distributions[base + 7];
    let f8 = distributions[base + 8];
    let f9 = distributions[base + 9];
    let f10 = distributions[base + 10];
    let f11 = distributions[base + 11];
    let f12 = distributions[base + 12];
    let f13 = distributions[base + 13];
    let f14 = distributions[base + 14];
    let f15 = distributions[base + 15];
    let f16 = distributions[base + 16];
    let f17 = distributions[base + 17];
    let f18 = distributions[base + 18];
    let f19 = distributions[base + 19];
    let f20 = distributions[base + 20];
    let f21 = distributions[base + 21];
    let f22 = distributions[base + 22];
    let f23 = distributions[base + 23];
    let f24 = distributions[base + 24];
    let f25 = distributions[base + 25];
    let f26 = distributions[base + 26];
let density = f0 + f1 + f10 + f11 + f12 + f13 + f14 + f15 + f16 + f17 + f18 + f19 + f2 + f20 + f21 + f22 + f23 + f24 + f25 + f26 + f3 + f4 + f5 + f6 + f7 + f8 + f9;

let ux = 1.0*f1 - 1.0*f10 + 1.0*f11 + 1.0*f12 - 1.0*f13 - 1.0*f14 + 1.0*f19 - 1.0*f2 + 1.0*f20 + 1.0*f21 - 1.0*f22 + 1.0*f23 - 1.0*f24 - 1.0*f25 - 1.0*f26 + 1.0*f7 + 1.0*f8 - 1.0*f9;

let uy = -1.0*f10 + 1.0*f15 + 1.0*f16 - 1.0*f17 - 1.0*f18 + 1.0*f19 + 1.0*f20 - 1.0*f21 + 1.0*f22 - 1.0*f23 - 1.0*f24 + 1.0*f25 - 1.0*f26 + 1.0*f3 - 1.0*f4 + 1.0*f7 - 1.0*f8 + 1.0*f9;

let uz = 1.0*f11 - 1.0*f12 + 1.0*f13 - 1.0*f14 + 1.0*f15 - 1.0*f16 + 1.0*f17 - 1.0*f18 + 1.0*f19 - 1.0*f20 + 1.0*f21 + 1.0*f22 - 1.0*f23 + 1.0*f24 - 1.0*f25 - 1.0*f26 + 1.0*f5 - 1.0*f6;

    densities[index] = density;
    set_velocity(index, vec3(ux, uy, uz)); 
}
"
}
