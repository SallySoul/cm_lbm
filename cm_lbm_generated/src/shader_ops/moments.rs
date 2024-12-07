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

let ux = q1 - q10 + q11 + q12 - q13 - q14 + q19 - q2 + q20 + q21 - q22 + q23 - q24 - q25 - q26 + q7 + q8 - q9;

let uy = -q10 + q15 + q16 - q17 - q18 + q19 + q20 - q21 + q22 - q23 - q24 + q25 - q26 + q3 - q4 + q7 - q8 + q9;

let uz = q11 - q12 + q13 - q14 + q15 - q16 + q17 - q18 + q19 - q20 + q21 + q22 - q23 + q24 - q25 - q26 + q5 - q6;

    densities[index] = density;
    set_velocity(index, vec3(ux/density, uy/density, uz/density)); 
}
"
}
