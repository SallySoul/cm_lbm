pub fn wgsl_eqhigh() -> &'static str {
    &"
fn f_equilibrium(density: f32, velocity: vec3<f32>) -> array<f32, 27> {
    let ux = velocity[0];
    let uy = velocity[1];
    let uz = velocity[2];
    var result: array<f32, 27>;
result[0] = 0.037037037037037*density*(-27.0*ux * ux*uy * uy*uz * uz + 18.0*ux * ux*uy * uy + 18.0*ux * ux*uz * uz - 12.0*ux * ux + 18.0*uy * uy*uz * uz - 12.0*uy * uy - 12.0*uz * uz + 8.0);

result[1] = 0.0185185185185185*density*(27.0*ux * ux*uy * uy*uz * uz - 18.0*ux * ux*uy * uy - 18.0*ux * ux*uz * uz + 12.0*ux * ux + 27.0*ux*uy * uy*uz * uz - 18.0*ux*uy * uy - 18.0*ux*uz * uz + 12.0*ux + 9.0*uy * uy*uz * uz - 6.0*uy * uy - 6.0*uz * uz + 4.0);

result[2] = 0.0185185185185185*density*(27.0*ux * ux*uy * uy*uz * uz - 18.0*ux * ux*uy * uy - 18.0*ux * ux*uz * uz + 12.0*ux * ux - 27.0*ux*uy * uy*uz * uz + 18.0*ux*uy * uy + 18.0*ux*uz * uz - 12.0*ux + 9.0*uy * uy*uz * uz - 6.0*uy * uy - 6.0*uz * uz + 4.0);

result[3] = 0.0185185185185185*density*(27.0*ux * ux*uy * uy*uz * uz - 18.0*ux * ux*uy * uy + 27.0*ux * ux*uy*uz * uz - 18.0*ux * ux*uy + 9.0*ux * ux*uz * uz - 6.0*ux * ux - 18.0*uy * uy*uz * uz + 12.0*uy * uy - 18.0*uy*uz * uz + 12.0*uy - 6.0*uz * uz + 4.0);

result[4] = 0.0185185185185185*density*(27.0*ux * ux*uy * uy*uz * uz - 18.0*ux * ux*uy * uy - 27.0*ux * ux*uy*uz * uz + 18.0*ux * ux*uy + 9.0*ux * ux*uz * uz - 6.0*ux * ux - 18.0*uy * uy*uz * uz + 12.0*uy * uy + 18.0*uy*uz * uz - 12.0*uy - 6.0*uz * uz + 4.0);

result[5] = 0.0185185185185185*density*(27.0*ux * ux*uy * uy*uz * uz + 27.0*ux * ux*uy * uy*uz + 9.0*ux * ux*uy * uy - 18.0*ux * ux*uz * uz - 18.0*ux * ux*uz - 6.0*ux * ux - 18.0*uy * uy*uz * uz - 18.0*uy * uy*uz - 6.0*uy * uy + 12.0*uz * uz + 12.0*uz + 4.0);

result[6] = 0.0185185185185185*density*(27.0*ux * ux*uy * uy*uz * uz - 27.0*ux * ux*uy * uy*uz + 9.0*ux * ux*uy * uy - 18.0*ux * ux*uz * uz + 18.0*ux * ux*uz - 6.0*ux * ux - 18.0*uy * uy*uz * uz + 18.0*uy * uy*uz - 6.0*uy * uy + 12.0*uz * uz - 12.0*uz + 4.0);

result[7] = 0.00925925925925926*density*(-9.0*ux * ux*uy * uy - 9.0*ux * ux*uy - 9.0*ux * ux*uz * uz + 6.0*ux * ux - 9.0*ux*uy * uy - 27.0*ux*uy*(ux*uy*uz * uz - ux*uy + ux*uz * uz - ux + uy*uz * uz - uy + uz * uz - 1.0) - 9.0*ux*uy - 9.0*ux*uz * uz + 6.0*ux - 9.0*uy * uy*uz * uz + 6.0*uy * uy - 9.0*uy*uz * uz + 6.0*uy - 3.0*uz * uz + 2.0);

result[8] = 0.00925925925925926*density*(-9.0*ux * ux*uy * uy + 9.0*ux * ux*uy - 9.0*ux * ux*uz * uz + 6.0*ux * ux - 9.0*ux*uy * uy - 27.0*ux*uy*(ux*uy*uz * uz - ux*uy - ux*uz * uz + ux + uy*uz * uz - uy - uz * uz + 1.0) + 9.0*ux*uy - 9.0*ux*uz * uz + 6.0*ux - 9.0*uy * uy*uz * uz + 6.0*uy * uy + 9.0*uy*uz * uz - 6.0*uy - 3.0*uz * uz + 2.0);

result[9] = 0.00925925925925926*density*(-9.0*ux * ux*uy * uy - 9.0*ux * ux*uy - 9.0*ux * ux*uz * uz + 6.0*ux * ux + 9.0*ux*uy * uy - 27.0*ux*uy*(ux*uy*uz * uz - ux*uy + ux*uz * uz - ux - uy*uz * uz + uy - uz * uz + 1.0) + 9.0*ux*uy + 9.0*ux*uz * uz - 6.0*ux - 9.0*uy * uy*uz * uz + 6.0*uy * uy - 9.0*uy*uz * uz + 6.0*uy - 3.0*uz * uz + 2.0);

result[10] = 0.00925925925925926*density*(-9.0*ux * ux*uy * uy + 9.0*ux * ux*uy - 9.0*ux * ux*uz * uz + 6.0*ux * ux + 9.0*ux*uy * uy - 27.0*ux*uy*(ux*uy*uz * uz - ux*uy - ux*uz * uz + ux - uy*uz * uz + uy + uz * uz - 1.0) - 9.0*ux*uy + 9.0*ux*uz * uz - 6.0*ux - 9.0*uy * uy*uz * uz + 6.0*uy * uy + 9.0*uy*uz * uz - 6.0*uy - 3.0*uz * uz + 2.0);

result[11] = 0.00925925925925926*density*(-9.0*ux * ux*uy * uy - 9.0*ux * ux*uz * uz - 9.0*ux * ux*uz + 6.0*ux * ux - 9.0*ux*uy * uy - 9.0*ux*uz * uz - 27.0*ux*uz*(ux*uy * uy*uz + ux*uy * uy - ux*uz - ux + uy * uy*uz + uy * uy - uz - 1.0) - 9.0*ux*uz + 6.0*ux - 9.0*uy * uy*uz * uz - 9.0*uy * uy*uz - 3.0*uy * uy + 6.0*uz * uz + 6.0*uz + 2.0);

result[12] = 0.00925925925925926*density*(-9.0*ux * ux*uy * uy - 9.0*ux * ux*uz * uz + 9.0*ux * ux*uz + 6.0*ux * ux - 9.0*ux*uy * uy - 9.0*ux*uz * uz - 27.0*ux*uz*(ux*uy * uy*uz - ux*uy * uy - ux*uz + ux + uy * uy*uz - uy * uy - uz + 1.0) + 9.0*ux*uz + 6.0*ux - 9.0*uy * uy*uz * uz + 9.0*uy * uy*uz - 3.0*uy * uy + 6.0*uz * uz - 6.0*uz + 2.0);

result[13] = 0.00925925925925926*density*(-9.0*ux * ux*uy * uy - 9.0*ux * ux*uz * uz - 9.0*ux * ux*uz + 6.0*ux * ux + 9.0*ux*uy * uy + 9.0*ux*uz * uz - 27.0*ux*uz*(ux*uy * uy*uz + ux*uy * uy - ux*uz - ux - uy * uy*uz - uy * uy + uz + 1.0) + 9.0*ux*uz - 6.0*ux - 9.0*uy * uy*uz * uz - 9.0*uy * uy*uz - 3.0*uy * uy + 6.0*uz * uz + 6.0*uz + 2.0);

result[14] = 0.00925925925925926*density*(-9.0*ux * ux*uy * uy - 9.0*ux * ux*uz * uz + 9.0*ux * ux*uz + 6.0*ux * ux + 9.0*ux*uy * uy + 9.0*ux*uz * uz - 27.0*ux*uz*(ux*uy * uy*uz - ux*uy * uy - ux*uz + ux - uy * uy*uz + uy * uy + uz - 1.0) - 9.0*ux*uz - 6.0*ux - 9.0*uy * uy*uz * uz + 9.0*uy * uy*uz - 3.0*uy * uy + 6.0*uz * uz - 6.0*uz + 2.0);

result[15] = 0.00925925925925926*density*(-9.0*ux * ux*uy * uy - 9.0*ux * ux*uy - 9.0*ux * ux*uz * uz - 9.0*ux * ux*uz - 3.0*ux * ux - 9.0*uy * uy*uz * uz - 9.0*uy * uy*uz + 6.0*uy * uy - 9.0*uy*uz * uz - 27.0*uy*uz*(ux * ux*uy*uz + ux * ux*uy + ux * ux*uz + ux * ux - uy*uz - uy - uz - 1.0) - 9.0*uy*uz + 6.0*uy + 6.0*uz * uz + 6.0*uz + 2.0);

result[16] = 0.00925925925925926*density*(-9.0*ux * ux*uy * uy - 9.0*ux * ux*uy - 9.0*ux * ux*uz * uz + 9.0*ux * ux*uz - 3.0*ux * ux - 9.0*uy * uy*uz * uz + 9.0*uy * uy*uz + 6.0*uy * uy - 9.0*uy*uz * uz - 27.0*uy*uz*(ux * ux*uy*uz - ux * ux*uy + ux * ux*uz - ux * ux - uy*uz + uy - uz + 1.0) + 9.0*uy*uz + 6.0*uy + 6.0*uz * uz - 6.0*uz + 2.0);

result[17] = 0.00925925925925926*density*(-9.0*ux * ux*uy * uy + 9.0*ux * ux*uy - 9.0*ux * ux*uz * uz - 9.0*ux * ux*uz - 3.0*ux * ux - 9.0*uy * uy*uz * uz - 9.0*uy * uy*uz + 6.0*uy * uy + 9.0*uy*uz * uz - 27.0*uy*uz*(ux * ux*uy*uz + ux * ux*uy - ux * ux*uz - ux * ux - uy*uz - uy + uz + 1.0) + 9.0*uy*uz - 6.0*uy + 6.0*uz * uz + 6.0*uz + 2.0);

result[18] = 0.00925925925925926*density*(-9.0*ux * ux*uy * uy + 9.0*ux * ux*uy - 9.0*ux * ux*uz * uz + 9.0*ux * ux*uz - 3.0*ux * ux - 9.0*uy * uy*uz * uz + 9.0*uy * uy*uz + 6.0*uy * uy + 9.0*uy*uz * uz - 27.0*uy*uz*(ux * ux*uy*uz - ux * ux*uy - ux * ux*uz + ux * ux - uy*uz + uy + uz - 1.0) - 9.0*uy*uz - 6.0*uy + 6.0*uz * uz - 6.0*uz + 2.0);

result[19] = 0.00462962962962963*density*(9.0*ux * ux*uy * uy + 9.0*ux * ux*uy + 9.0*ux * ux*uz * uz + 9.0*ux * ux*uz + 3.0*ux * ux + 9.0*ux*uy * uy + 27.0*ux*uy*uz*(ux*uy*uz + ux*uy + ux*uz + ux + uy*uz + uy + uz + 1.0) + 9.0*ux*uy + 9.0*ux*uz * uz + 9.0*ux*uz + 3.0*ux + 9.0*uy * uy*uz * uz + 9.0*uy * uy*uz + 3.0*uy * uy + 9.0*uy*uz * uz + 9.0*uy*uz + 3.0*uy + 3.0*uz * uz + 3.0*uz + 1.0);

result[20] = 0.00462962962962963*density*(9.0*ux * ux*uy * uy + 9.0*ux * ux*uy + 9.0*ux * ux*uz * uz - 9.0*ux * ux*uz + 3.0*ux * ux + 9.0*ux*uy * uy + 27.0*ux*uy*uz*(ux*uy*uz - ux*uy + ux*uz - ux + uy*uz - uy + uz - 1.0) + 9.0*ux*uy + 9.0*ux*uz * uz - 9.0*ux*uz + 3.0*ux + 9.0*uy * uy*uz * uz - 9.0*uy * uy*uz + 3.0*uy * uy + 9.0*uy*uz * uz - 9.0*uy*uz + 3.0*uy + 3.0*uz * uz - 3.0*uz + 1.0);

result[21] = 0.00462962962962963*density*(9.0*ux * ux*uy * uy - 9.0*ux * ux*uy + 9.0*ux * ux*uz * uz + 9.0*ux * ux*uz + 3.0*ux * ux + 9.0*ux*uy * uy + 27.0*ux*uy*uz*(ux*uy*uz + ux*uy - ux*uz - ux + uy*uz + uy - uz - 1.0) - 9.0*ux*uy + 9.0*ux*uz * uz + 9.0*ux*uz + 3.0*ux + 9.0*uy * uy*uz * uz + 9.0*uy * uy*uz + 3.0*uy * uy - 9.0*uy*uz * uz - 9.0*uy*uz - 3.0*uy + 3.0*uz * uz + 3.0*uz + 1.0);

result[22] = 0.00462962962962963*density*(9.0*ux * ux*uy * uy + 9.0*ux * ux*uy + 9.0*ux * ux*uz * uz + 9.0*ux * ux*uz + 3.0*ux * ux - 9.0*ux*uy * uy + 27.0*ux*uy*uz*(ux*uy*uz + ux*uy + ux*uz + ux - uy*uz - uy - uz - 1.0) - 9.0*ux*uy - 9.0*ux*uz * uz - 9.0*ux*uz - 3.0*ux + 9.0*uy * uy*uz * uz + 9.0*uy * uy*uz + 3.0*uy * uy + 9.0*uy*uz * uz + 9.0*uy*uz + 3.0*uy + 3.0*uz * uz + 3.0*uz + 1.0);

result[23] = 0.00462962962962963*density*(9.0*ux * ux*uy * uy - 9.0*ux * ux*uy + 9.0*ux * ux*uz * uz - 9.0*ux * ux*uz + 3.0*ux * ux + 9.0*ux*uy * uy + 27.0*ux*uy*uz*(ux*uy*uz - ux*uy - ux*uz + ux + uy*uz - uy - uz + 1.0) - 9.0*ux*uy + 9.0*ux*uz * uz - 9.0*ux*uz + 3.0*ux + 9.0*uy * uy*uz * uz - 9.0*uy * uy*uz + 3.0*uy * uy - 9.0*uy*uz * uz + 9.0*uy*uz - 3.0*uy + 3.0*uz * uz - 3.0*uz + 1.0);

result[24] = 0.00462962962962963*density*(9.0*ux * ux*uy * uy - 9.0*ux * ux*uy + 9.0*ux * ux*uz * uz + 9.0*ux * ux*uz + 3.0*ux * ux - 9.0*ux*uy * uy + 27.0*ux*uy*uz*(ux*uy*uz + ux*uy - ux*uz - ux - uy*uz - uy + uz + 1.0) + 9.0*ux*uy - 9.0*ux*uz * uz - 9.0*ux*uz - 3.0*ux + 9.0*uy * uy*uz * uz + 9.0*uy * uy*uz + 3.0*uy * uy - 9.0*uy*uz * uz - 9.0*uy*uz - 3.0*uy + 3.0*uz * uz + 3.0*uz + 1.0);

result[25] = 0.00462962962962963*density*(9.0*ux * ux*uy * uy + 9.0*ux * ux*uy + 9.0*ux * ux*uz * uz - 9.0*ux * ux*uz + 3.0*ux * ux - 9.0*ux*uy * uy + 27.0*ux*uy*uz*(ux*uy*uz - ux*uy + ux*uz - ux - uy*uz + uy - uz + 1.0) - 9.0*ux*uy - 9.0*ux*uz * uz + 9.0*ux*uz - 3.0*ux + 9.0*uy * uy*uz * uz - 9.0*uy * uy*uz + 3.0*uy * uy + 9.0*uy*uz * uz - 9.0*uy*uz + 3.0*uy + 3.0*uz * uz - 3.0*uz + 1.0);

result[26] = 0.00462962962962963*density*(9.0*ux * ux*uy * uy - 9.0*ux * ux*uy + 9.0*ux * ux*uz * uz - 9.0*ux * ux*uz + 3.0*ux * ux - 9.0*ux*uy * uy + 27.0*ux*uy*uz*(ux*uy*uz - ux*uy - ux*uz + ux - uy*uz + uy + uz - 1.0) + 9.0*ux*uy - 9.0*ux*uz * uz + 9.0*ux*uz - 3.0*ux + 9.0*uy * uy*uz * uz - 9.0*uy * uy*uz + 3.0*uy * uy - 9.0*uy*uz * uz + 9.0*uy*uz - 3.0*uy + 3.0*uz * uz - 3.0*uz + 1.0);

    return result;
}
"
}
