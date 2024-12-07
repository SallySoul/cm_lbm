pub fn bgk(
    f: [f32; 27],
    ux: f32,
    uy: f32,
    uz: f32,
    density: f32,
    omega: f32,
) -> [f32; 27] {
    let mut result = [0.0; 27];
    let q0 = f[0];
    let q1 = f[1];
    let q2 = f[2];
    let q3 = f[3];
    let q4 = f[4];
    let q5 = f[5];
    let q6 = f[6];
    let q7 = f[7];
    let q8 = f[8];
    let q9 = f[9];
    let q10 = f[10];
    let q11 = f[11];
    let q12 = f[12];
    let q13 = f[13];
    let q14 = f[14];
    let q15 = f[15];
    let q16 = f[16];
    let q17 = f[17];
    let q18 = f[18];
    let q19 = f[19];
    let q20 = f[20];
    let q21 = f[21];
    let q22 = f[22];
    let q23 = f[23];
    let q24 = f[24];
    let q25 = f[25];
    let q26 = f[26];
    result[0] = omega
        * (0.296296296296296
            * density
            * (-1.5 * ux * ux - 1.5 * uy * uy - 1.5 * uz * uz + 1.0)
            - q0)
        + q0;

    result[1] = omega
        * (0.0740740740740741
            * density
            * (3.0 * ux * ux + 3.0 * ux - 1.5 * uy * uy - 1.5 * uz * uz + 1.0)
            - q1)
        + q1;

    result[2] = omega
        * (0.0740740740740741
            * density
            * (3.0 * ux * ux - 3.0 * ux - 1.5 * uy * uy - 1.5 * uz * uz + 1.0)
            - q2)
        + q2;

    result[3] = omega
        * (0.0740740740740741
            * density
            * (-1.5 * ux * ux + 3.0 * uy * uy + 3.0 * uy - 1.5 * uz * uz
                + 1.0)
            - q3)
        + q3;

    result[4] = omega
        * (0.0740740740740741
            * density
            * (-1.5 * ux * ux + 3.0 * uy * uy - 3.0 * uy - 1.5 * uz * uz
                + 1.0)
            - q4)
        + q4;

    result[5] = omega
        * (0.0740740740740741
            * density
            * (-1.5 * ux * ux - 1.5 * uy * uy
                + 3.0 * uz * uz
                + 3.0 * uz
                + 1.0)
            - q5)
        + q5;

    result[6] = omega
        * (0.0740740740740741
            * density
            * (-1.5 * ux * ux - 1.5 * uy * uy + 3.0 * uz * uz - 3.0 * uz
                + 1.0)
            - q6)
        + q6;

    result[7] = omega
        * (0.0185185185185185
            * density
            * (-1.5 * ux * ux + 3.0 * ux - 1.5 * uy * uy + 3.0 * uy
                - 1.5 * uz * uz
                + 4.5 * (ux + uy) * (ux + uy)
                + 1.0)
            - q7)
        + q7;

    result[8] = omega
        * (0.0185185185185185
            * density
            * (-1.5 * ux * ux + 3.0 * ux
                - 1.5 * uy * uy
                - 3.0 * uy
                - 1.5 * uz * uz
                + 4.5 * (ux - uy) * (ux - uy)
                + 1.0)
            - q8)
        + q8;

    result[9] = omega
        * (0.0185185185185185
            * density
            * (-1.5 * ux * ux - 3.0 * ux - 1.5 * uy * uy + 3.0 * uy
                - 1.5 * uz * uz
                + 4.5 * (-ux + uy) * (-ux + uy)
                + 1.0)
            - q9)
        + q9;

    result[10] = omega
        * (0.0185185185185185
            * density
            * (-1.5 * ux * ux
                - 3.0 * ux
                - 1.5 * uy * uy
                - 3.0 * uy
                - 1.5 * uz * uz
                + 4.5 * (-ux - uy) * (-ux - uy)
                + 1.0)
            - q10)
        + q10;

    result[11] = omega
        * (0.0185185185185185
            * density
            * (-1.5 * ux * ux + 3.0 * ux - 1.5 * uy * uy - 1.5 * uz * uz
                + 3.0 * uz
                + 4.5 * (ux + uz) * (ux + uz)
                + 1.0)
            - q11)
        + q11;

    result[12] = omega
        * (0.0185185185185185
            * density
            * (-1.5 * ux * ux + 3.0 * ux
                - 1.5 * uy * uy
                - 1.5 * uz * uz
                - 3.0 * uz
                + 4.5 * (ux - uz) * (ux - uz)
                + 1.0)
            - q12)
        + q12;

    result[13] = omega
        * (0.0185185185185185
            * density
            * (-1.5 * ux * ux - 3.0 * ux - 1.5 * uy * uy - 1.5 * uz * uz
                + 3.0 * uz
                + 4.5 * (-ux + uz) * (-ux + uz)
                + 1.0)
            - q13)
        + q13;

    result[14] = omega
        * (0.0185185185185185
            * density
            * (-1.5 * ux * ux
                - 3.0 * ux
                - 1.5 * uy * uy
                - 1.5 * uz * uz
                - 3.0 * uz
                + 4.5 * (-ux - uz) * (-ux - uz)
                + 1.0)
            - q14)
        + q14;

    result[15] = omega
        * (0.0185185185185185
            * density
            * (-1.5 * ux * ux - 1.5 * uy * uy + 3.0 * uy - 1.5 * uz * uz
                + 3.0 * uz
                + 4.5 * (uy + uz) * (uy + uz)
                + 1.0)
            - q15)
        + q15;

    result[16] = omega
        * (0.0185185185185185
            * density
            * (-1.5 * ux * ux - 1.5 * uy * uy + 3.0 * uy
                - 1.5 * uz * uz
                - 3.0 * uz
                + 4.5 * (uy - uz) * (uy - uz)
                + 1.0)
            - q16)
        + q16;

    result[17] = omega
        * (0.0185185185185185
            * density
            * (-1.5 * ux * ux - 1.5 * uy * uy - 3.0 * uy - 1.5 * uz * uz
                + 3.0 * uz
                + 4.5 * (-uy + uz) * (-uy + uz)
                + 1.0)
            - q17)
        + q17;

    result[18] = omega
        * (0.0185185185185185
            * density
            * (-1.5 * ux * ux
                - 1.5 * uy * uy
                - 3.0 * uy
                - 1.5 * uz * uz
                - 3.0 * uz
                + 4.5 * (-uy - uz) * (-uy - uz)
                + 1.0)
            - q18)
        + q18;

    result[19] = omega
        * (0.00462962962962963
            * density
            * (-1.5 * ux * ux + 3.0 * ux - 1.5 * uy * uy + 3.0 * uy
                - 1.5 * uz * uz
                + 3.0 * uz
                + 4.5 * (ux + uy + uz) * (ux + uy + uz)
                + 1.0)
            - q19)
        + q19;

    result[20] = omega
        * (0.00462962962962963
            * density
            * (-1.5 * ux * ux + 3.0 * ux - 1.5 * uy * uy + 3.0 * uy
                - 1.5 * uz * uz
                - 3.0 * uz
                + 4.5 * (ux + uy - uz) * (ux + uy - uz)
                + 1.0)
            - q20)
        + q20;

    result[21] = omega
        * (0.00462962962962963
            * density
            * (-1.5 * ux * ux + 3.0 * ux
                - 1.5 * uy * uy
                - 3.0 * uy
                - 1.5 * uz * uz
                + 3.0 * uz
                + 4.5 * (ux - uy + uz) * (ux - uy + uz)
                + 1.0)
            - q21)
        + q21;

    result[22] = omega
        * (0.00462962962962963
            * density
            * (-1.5 * ux * ux - 3.0 * ux - 1.5 * uy * uy + 3.0 * uy
                - 1.5 * uz * uz
                + 3.0 * uz
                + 4.5 * (-ux + uy + uz) * (-ux + uy + uz)
                + 1.0)
            - q22)
        + q22;

    result[23] = omega
        * (0.00462962962962963
            * density
            * (-1.5 * ux * ux + 3.0 * ux
                - 1.5 * uy * uy
                - 3.0 * uy
                - 1.5 * uz * uz
                - 3.0 * uz
                + 4.5 * (ux - uy - uz) * (ux - uy - uz)
                + 1.0)
            - q23)
        + q23;

    result[24] = omega
        * (0.00462962962962963
            * density
            * (-1.5 * ux * ux
                - 3.0 * ux
                - 1.5 * uy * uy
                - 3.0 * uy
                - 1.5 * uz * uz
                + 3.0 * uz
                + 4.5 * (-ux - uy + uz) * (-ux - uy + uz)
                + 1.0)
            - q24)
        + q24;

    result[25] = omega
        * (0.00462962962962963
            * density
            * (-1.5 * ux * ux - 3.0 * ux - 1.5 * uy * uy + 3.0 * uy
                - 1.5 * uz * uz
                - 3.0 * uz
                + 4.5 * (-ux + uy - uz) * (-ux + uy - uz)
                + 1.0)
            - q25)
        + q25;

    result[26] = omega
        * (0.00462962962962963
            * density
            * (-1.5 * ux * ux
                - 3.0 * ux
                - 1.5 * uy * uy
                - 3.0 * uy
                - 1.5 * uz * uz
                - 3.0 * uz
                + 4.5 * (-ux - uy - uz) * (-ux - uy - uz)
                + 1.0)
            - q26)
        + q26;

    result
}
