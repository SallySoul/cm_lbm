pub fn wgsl_bgk(omega: f32) -> String {
    format!("
fn bgk(index: i32) {{
    let velocity = get_velocity(index);
    let ux = velocity[0];
    let uy = velocity[1];
    let uz = velocity[2];
    let density = densities[index];
    let omega = {};
    var result: array<f32, 27>;
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
result[0] = omega*(0.296296296296296*density*(-3.0*ux * ux - 3.0*uy * uy - 3.0*uz * uz + 1.0) - q0) + q0;

result[1] = omega*(0.0740740740740741*density*(6.0*ux * ux + 3.0*ux - 3.0*uy * uy - 3.0*uz * uz + 1.0) - q1) + q1;

result[2] = omega*(0.0740740740740741*density*(6.0*ux * ux - 3.0*ux - 3.0*uy * uy - 3.0*uz * uz + 1.0) - q2) + q2;

result[3] = omega*(0.0740740740740741*density*(-3.0*ux * ux + 6.0*uy * uy + 3.0*uy - 3.0*uz * uz + 1.0) - q3) + q3;

result[4] = omega*(0.0740740740740741*density*(-3.0*ux * ux + 6.0*uy * uy - 3.0*uy - 3.0*uz * uz + 1.0) - q4) + q4;

result[5] = omega*(0.0740740740740741*density*(-3.0*ux * ux - 3.0*uy * uy + 6.0*uz * uz + 3.0*uz + 1.0) - q5) + q5;

result[6] = omega*(0.0740740740740741*density*(-3.0*ux * ux - 3.0*uy * uy + 6.0*uz * uz - 3.0*uz + 1.0) - q6) + q6;

result[7] = omega*(0.0185185185185185*density*(-3.0*ux * ux + 3.0*ux - 3.0*uy * uy + 3.0*uy - 3.0*uz * uz + 9.0*(ux + uy) * (ux + uy) + 1.0) - q7) + q7;

result[8] = omega*(0.0185185185185185*density*(-3.0*ux * ux + 3.0*ux - 3.0*uy * uy - 3.0*uy - 3.0*uz * uz + 9.0*(ux - uy) * (ux - uy) + 1.0) - q8) + q8;

result[9] = omega*(0.0185185185185185*density*(-3.0*ux * ux - 3.0*ux - 3.0*uy * uy + 3.0*uy - 3.0*uz * uz + 9.0*(-ux + uy) * (-ux + uy) + 1.0) - q9) + q9;

result[10] = omega*(0.0185185185185185*density*(-3.0*ux * ux - 3.0*ux - 3.0*uy * uy - 3.0*uy - 3.0*uz * uz + 9.0*(-ux - uy) * (-ux - uy) + 1.0) - q10) + q10;

result[11] = omega*(0.0185185185185185*density*(-3.0*ux * ux + 3.0*ux - 3.0*uy * uy - 3.0*uz * uz + 3.0*uz + 9.0*(ux + uz) * (ux + uz) + 1.0) - q11) + q11;

result[12] = omega*(0.0185185185185185*density*(-3.0*ux * ux + 3.0*ux - 3.0*uy * uy - 3.0*uz * uz - 3.0*uz + 9.0*(ux - uz) * (ux - uz) + 1.0) - q12) + q12;

result[13] = omega*(0.0185185185185185*density*(-3.0*ux * ux - 3.0*ux - 3.0*uy * uy - 3.0*uz * uz + 3.0*uz + 9.0*(-ux + uz) * (-ux + uz) + 1.0) - q13) + q13;

result[14] = omega*(0.0185185185185185*density*(-3.0*ux * ux - 3.0*ux - 3.0*uy * uy - 3.0*uz * uz - 3.0*uz + 9.0*(-ux - uz) * (-ux - uz) + 1.0) - q14) + q14;

result[15] = omega*(0.0185185185185185*density*(-3.0*ux * ux - 3.0*uy * uy + 3.0*uy - 3.0*uz * uz + 3.0*uz + 9.0*(uy + uz) * (uy + uz) + 1.0) - q15) + q15;

result[16] = omega*(0.0185185185185185*density*(-3.0*ux * ux - 3.0*uy * uy + 3.0*uy - 3.0*uz * uz - 3.0*uz + 9.0*(uy - uz) * (uy - uz) + 1.0) - q16) + q16;

result[17] = omega*(0.0185185185185185*density*(-3.0*ux * ux - 3.0*uy * uy - 3.0*uy - 3.0*uz * uz + 3.0*uz + 9.0*(-uy + uz) * (-uy + uz) + 1.0) - q17) + q17;

result[18] = omega*(0.0185185185185185*density*(-3.0*ux * ux - 3.0*uy * uy - 3.0*uy - 3.0*uz * uz - 3.0*uz + 9.0*(-uy - uz) * (-uy - uz) + 1.0) - q18) + q18;

result[19] = omega*(0.00462962962962963*density*(-3.0*ux * ux + 3.0*ux - 3.0*uy * uy + 3.0*uy - 3.0*uz * uz + 3.0*uz + 9.0*(ux + uy + uz) * (ux + uy + uz) + 1.0) - q19) + q19;

result[20] = omega*(0.00462962962962963*density*(-3.0*ux * ux + 3.0*ux - 3.0*uy * uy + 3.0*uy - 3.0*uz * uz - 3.0*uz + 9.0*(ux + uy - uz) * (ux + uy - uz) + 1.0) - q20) + q20;

result[21] = omega*(0.00462962962962963*density*(-3.0*ux * ux + 3.0*ux - 3.0*uy * uy - 3.0*uy - 3.0*uz * uz + 3.0*uz + 9.0*(ux - uy + uz) * (ux - uy + uz) + 1.0) - q21) + q21;

result[22] = omega*(0.00462962962962963*density*(-3.0*ux * ux - 3.0*ux - 3.0*uy * uy + 3.0*uy - 3.0*uz * uz + 3.0*uz + 9.0*(-ux + uy + uz) * (-ux + uy + uz) + 1.0) - q22) + q22;

result[23] = omega*(0.00462962962962963*density*(-3.0*ux * ux + 3.0*ux - 3.0*uy * uy - 3.0*uy - 3.0*uz * uz - 3.0*uz + 9.0*(ux - uy - uz) * (ux - uy - uz) + 1.0) - q23) + q23;

result[24] = omega*(0.00462962962962963*density*(-3.0*ux * ux - 3.0*ux - 3.0*uy * uy - 3.0*uy - 3.0*uz * uz + 3.0*uz + 9.0*(-ux - uy + uz) * (-ux - uy + uz) + 1.0) - q24) + q24;

result[25] = omega*(0.00462962962962963*density*(-3.0*ux * ux - 3.0*ux - 3.0*uy * uy + 3.0*uy - 3.0*uz * uz - 3.0*uz + 9.0*(-ux + uy - uz) * (-ux + uy - uz) + 1.0) - q25) + q25;

result[26] = omega*(0.00462962962962963*density*(-3.0*ux * ux - 3.0*ux - 3.0*uy * uy - 3.0*uy - 3.0*uz * uz - 3.0*uz + 9.0*(-ux - uy - uz) * (-ux - uy - uz) + 1.0) - q26) + q26;

    add_qi_to_distributions(index, result);
}}
", omega)
}
