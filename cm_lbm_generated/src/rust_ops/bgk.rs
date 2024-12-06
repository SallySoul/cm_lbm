pub fn bgk(
    f: [f32; 27],
    ux: f32,
    uy: f32,
    uz: f32,
    density: f32,
    omega: f32,
) -> [f32; 27] {
    let mut result = [0.0; 27];
    let f0 = f[0];
    let f1 = f[1];
    let f2 = f[2];
    let f3 = f[3];
    let f4 = f[4];
    let f5 = f[5];
    let f6 = f[6];
    let f7 = f[7];
    let f8 = f[8];
    let f9 = f[9];
    let f10 = f[10];
    let f11 = f[11];
    let f12 = f[12];
    let f13 = f[13];
    let f14 = f[14];
    let f15 = f[15];
    let f16 = f[16];
    let f17 = f[17];
    let f18 = f[18];
    let f19 = f[19];
    let f20 = f[20];
    let f21 = f[21];
    let f22 = f[22];
    let f23 = f[23];
    let f24 = f[24];
    let f25 = f[25];
    let f26 = f[26];
result[0] = f0 + omega*(-0.296296296296296*density*(-1.5*ux * ux - 1.5*uy * uy - 1.5*uz * uz + 1.0) + f0);

result[1] = f1 + omega*(-0.0740740740740741*density*(3.0*ux * ux + 3.0*ux - 1.5*uy * uy - 1.5*uz * uz + 1.0) + f1);

result[2] = f2 + omega*(-0.0740740740740741*density*(3.0*ux * ux - 3.0*ux - 1.5*uy * uy - 1.5*uz * uz + 1.0) + f2);

result[3] = f3 + omega*(-0.0740740740740741*density*(-1.5*ux * ux + 3.0*uy * uy + 3.0*uy - 1.5*uz * uz + 1.0) + f3);

result[4] = f4 + omega*(-0.0740740740740741*density*(-1.5*ux * ux + 3.0*uy * uy - 3.0*uy - 1.5*uz * uz + 1.0) + f4);

result[5] = f5 + omega*(-0.0740740740740741*density*(-1.5*ux * ux - 1.5*uy * uy + 3.0*uz * uz + 3.0*uz + 1.0) + f5);

result[6] = f6 + omega*(-0.0740740740740741*density*(-1.5*ux * ux - 1.5*uy * uy + 3.0*uz * uz - 3.0*uz + 1.0) + f6);

result[7] = f7 + omega*(-0.0185185185185185*density*(-1.5*ux * ux + 3.0*ux - 1.5*uy * uy + 3.0*uy - 1.5*uz * uz + 4.5*(1.0*ux + 1.0*uy) * (1.0*ux + 1.0*uy) + 1.0) + f7);

result[8] = f8 + omega*(-0.0185185185185185*density*(-1.5*ux * ux + 3.0*ux - 1.5*uy * uy - 3.0*uy - 1.5*uz * uz + 4.5*(1.0*ux - 1.0*uy) * (1.0*ux - 1.0*uy) + 1.0) + f8);

result[9] = f9 + omega*(-0.0185185185185185*density*(-1.5*ux * ux - 3.0*ux - 1.5*uy * uy + 3.0*uy - 1.5*uz * uz + 4.5*(-1.0*ux + 1.0*uy) * (-1.0*ux + 1.0*uy) + 1.0) + f9);

result[10] = f10 + omega*(-0.0185185185185185*density*(-1.5*ux * ux - 3.0*ux - 1.5*uy * uy - 3.0*uy - 1.5*uz * uz + 4.5*(-1.0*ux - 1.0*uy) * (-1.0*ux - 1.0*uy) + 1.0) + f10);

result[11] = f11 + omega*(-0.0185185185185185*density*(-1.5*ux * ux + 3.0*ux - 1.5*uy * uy - 1.5*uz * uz + 3.0*uz + 4.5*(1.0*ux + 1.0*uz) * (1.0*ux + 1.0*uz) + 1.0) + f11);

result[12] = f12 + omega*(-0.0185185185185185*density*(-1.5*ux * ux + 3.0*ux - 1.5*uy * uy - 1.5*uz * uz - 3.0*uz + 4.5*(1.0*ux - 1.0*uz) * (1.0*ux - 1.0*uz) + 1.0) + f12);

result[13] = f13 + omega*(-0.0185185185185185*density*(-1.5*ux * ux - 3.0*ux - 1.5*uy * uy - 1.5*uz * uz + 3.0*uz + 4.5*(-1.0*ux + 1.0*uz) * (-1.0*ux + 1.0*uz) + 1.0) + f13);

result[14] = f14 + omega*(-0.0185185185185185*density*(-1.5*ux * ux - 3.0*ux - 1.5*uy * uy - 1.5*uz * uz - 3.0*uz + 4.5*(-1.0*ux - 1.0*uz) * (-1.0*ux - 1.0*uz) + 1.0) + f14);

result[15] = f15 + omega*(-0.0185185185185185*density*(-1.5*ux * ux - 1.5*uy * uy + 3.0*uy - 1.5*uz * uz + 3.0*uz + 4.5*(1.0*uy + 1.0*uz) * (1.0*uy + 1.0*uz) + 1.0) + f15);

result[16] = f16 + omega*(-0.0185185185185185*density*(-1.5*ux * ux - 1.5*uy * uy + 3.0*uy - 1.5*uz * uz - 3.0*uz + 4.5*(1.0*uy - 1.0*uz) * (1.0*uy - 1.0*uz) + 1.0) + f16);

result[17] = f17 + omega*(-0.0185185185185185*density*(-1.5*ux * ux - 1.5*uy * uy - 3.0*uy - 1.5*uz * uz + 3.0*uz + 4.5*(-1.0*uy + 1.0*uz) * (-1.0*uy + 1.0*uz) + 1.0) + f17);

result[18] = f18 + omega*(-0.0185185185185185*density*(-1.5*ux * ux - 1.5*uy * uy - 3.0*uy - 1.5*uz * uz - 3.0*uz + 4.5*(-1.0*uy - 1.0*uz) * (-1.0*uy - 1.0*uz) + 1.0) + f18);

result[19] = f19 + omega*(-0.00462962962962963*density*(-1.5*ux * ux + 3.0*ux - 1.5*uy * uy + 3.0*uy - 1.5*uz * uz + 3.0*uz + 4.5*(1.0*ux + 1.0*uy + 1.0*uz) * (1.0*ux + 1.0*uy + 1.0*uz) + 1.0) + f19);

result[20] = f20 + omega*(-0.00462962962962963*density*(-1.5*ux * ux + 3.0*ux - 1.5*uy * uy + 3.0*uy - 1.5*uz * uz - 3.0*uz + 4.5*(1.0*ux + 1.0*uy - 1.0*uz) * (1.0*ux + 1.0*uy - 1.0*uz) + 1.0) + f20);

result[21] = f21 + omega*(-0.00462962962962963*density*(-1.5*ux * ux + 3.0*ux - 1.5*uy * uy - 3.0*uy - 1.5*uz * uz + 3.0*uz + 4.5*(1.0*ux - 1.0*uy + 1.0*uz) * (1.0*ux - 1.0*uy + 1.0*uz) + 1.0) + f21);

result[22] = f22 + omega*(-0.00462962962962963*density*(-1.5*ux * ux - 3.0*ux - 1.5*uy * uy + 3.0*uy - 1.5*uz * uz + 3.0*uz + 4.5*(-1.0*ux + 1.0*uy + 1.0*uz) * (-1.0*ux + 1.0*uy + 1.0*uz) + 1.0) + f22);

result[23] = f23 + omega*(-0.00462962962962963*density*(-1.5*ux * ux + 3.0*ux - 1.5*uy * uy - 3.0*uy - 1.5*uz * uz - 3.0*uz + 4.5*(1.0*ux - 1.0*uy - 1.0*uz) * (1.0*ux - 1.0*uy - 1.0*uz) + 1.0) + f23);

result[24] = f24 + omega*(-0.00462962962962963*density*(-1.5*ux * ux - 3.0*ux - 1.5*uy * uy - 3.0*uy - 1.5*uz * uz + 3.0*uz + 4.5*(-1.0*ux - 1.0*uy + 1.0*uz) * (-1.0*ux - 1.0*uy + 1.0*uz) + 1.0) + f24);

result[25] = f25 + omega*(-0.00462962962962963*density*(-1.5*ux * ux - 3.0*ux - 1.5*uy * uy + 3.0*uy - 1.5*uz * uz - 3.0*uz + 4.5*(-1.0*ux + 1.0*uy - 1.0*uz) * (-1.0*ux + 1.0*uy - 1.0*uz) + 1.0) + f25);

result[26] = f26 + omega*(-0.00462962962962963*density*(-1.5*ux * ux - 3.0*ux - 1.5*uy * uy - 3.0*uy - 1.5*uz * uz - 3.0*uz + 4.5*(-1.0*ux - 1.0*uy - 1.0*uz) * (-1.0*ux - 1.0*uy - 1.0*uz) + 1.0) + f26);

  result
}


