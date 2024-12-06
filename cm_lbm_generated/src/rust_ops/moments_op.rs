pub fn moments_op(f: [f32; 27]) -> (f32, [f32;3]) {
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
let density = f0 + f1 + f10 + f11 + f12 + f13 + f14 + f15 + f16 + f17 + f18 + f19 + f2 + f20 + f21 + f22 + f23 + f24 + f25 + f26 + f3 + f4 + f5 + f6 + f7 + f8 + f9;

let ux = 1.0*f1 - 1.0*f10 + 1.0*f11 + 1.0*f12 - 1.0*f13 - 1.0*f14 + 1.0*f19 - 1.0*f2 + 1.0*f20 + 1.0*f21 - 1.0*f22 + 1.0*f23 - 1.0*f24 - 1.0*f25 - 1.0*f26 + 1.0*f7 + 1.0*f8 - 1.0*f9;

let uy = -1.0*f10 + 1.0*f15 + 1.0*f16 - 1.0*f17 - 1.0*f18 + 1.0*f19 + 1.0*f20 - 1.0*f21 + 1.0*f22 - 1.0*f23 - 1.0*f24 + 1.0*f25 - 1.0*f26 + 1.0*f3 - 1.0*f4 + 1.0*f7 - 1.0*f8 + 1.0*f9;

let uz = 1.0*f11 - 1.0*f12 + 1.0*f13 - 1.0*f14 + 1.0*f15 - 1.0*f16 + 1.0*f17 - 1.0*f18 + 1.0*f19 - 1.0*f20 + 1.0*f21 + 1.0*f22 - 1.0*f23 + 1.0*f24 - 1.0*f25 - 1.0*f26 + 1.0*f5 - 1.0*f6;

    (density, [ux/density, uy/density, uz/density]) 
}


