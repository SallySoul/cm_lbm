pub fn wgsl_moments() -> &'static str {
    &"
fn moments(index: i32) {
    let base = index * 27; 
    let q0 = distributions[base + 0];
    let q1 = distributions[base + 1];
    let q2 = distributions[base + 2];
    let q3 = distributions[base + 3];
    let q4 = distributions[base + 4];
    let q5 = distributions[base + 5];
    let q6 = distributions[base + 6];
    let q7 = distributions[base + 7];
    let q8 = distributions[base + 8];
    let q9 = distributions[base + 9];
    let q10 = distributions[base + 10];
    let q11 = distributions[base + 11];
    let q12 = distributions[base + 12];
    let q13 = distributions[base + 13];
    let q14 = distributions[base + 14];
    let q15 = distributions[base + 15];
    let q16 = distributions[base + 16];
    let q17 = distributions[base + 17];
    let q18 = distributions[base + 18];
    let q19 = distributions[base + 19];
    let q20 = distributions[base + 20];
    let q21 = distributions[base + 21];
    let q22 = distributions[base + 22];
    let q23 = distributions[base + 23];
    let q24 = distributions[base + 24];
    let q25 = distributions[base + 25];
    let q26 = distributions[base + 26];
let density = q0 + q1 + q10 + q11 + q12 + q13 + q14 + q15 + q16 + q17 + q18 + q19 + q2 + q20 + q21 + q22 + q23 + q24 + q25 + q26 + q3 + q4 + q5 + q6 + q7 + q8 + q9;

let ux = 1.0*q1 - 1.0*q10 + 1.0*q11 + 1.0*q12 - 1.0*q13 - 1.0*q14 + 1.0*q19 - 1.0*q2 + 1.0*q20 + 1.0*q21 - 1.0*q22 + 1.0*q23 - 1.0*q24 - 1.0*q25 - 1.0*q26 + 1.0*q7 + 1.0*q8 - 1.0*q9;

let uy = -1.0*q10 + 1.0*q15 + 1.0*q16 - 1.0*q17 - 1.0*q18 + 1.0*q19 + 1.0*q20 - 1.0*q21 + 1.0*q22 - 1.0*q23 - 1.0*q24 + 1.0*q25 - 1.0*q26 + 1.0*q3 - 1.0*q4 + 1.0*q7 - 1.0*q8 + 1.0*q9;

let uz = 1.0*q11 - 1.0*q12 + 1.0*q13 - 1.0*q14 + 1.0*q15 - 1.0*q16 + 1.0*q17 - 1.0*q18 + 1.0*q19 - 1.0*q20 + 1.0*q21 + 1.0*q22 - 1.0*q23 + 1.0*q24 - 1.0*q25 - 1.0*q26 + 1.0*q5 - 1.0*q6;

    densities[index] = density;
    set_velocity(index, vec3(ux, uy, uz)); 
}
"
}
