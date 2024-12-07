pub fn moments_f64(f: [f64; 27]) -> (f64, [f64; 3]) {
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
    let density = q0
        + q1
        + q10
        + q11
        + q12
        + q13
        + q14
        + q15
        + q16
        + q17
        + q18
        + q19
        + q2
        + q20
        + q21
        + q22
        + q23
        + q24
        + q25
        + q26
        + q3
        + q4
        + q5
        + q6
        + q7
        + q8
        + q9;

    let ux =
        1.0 * q1 - 1.0 * q10 + 1.0 * q11 + 1.0 * q12 - 1.0 * q13 - 1.0 * q14
            + 1.0 * q19
            - 1.0 * q2
            + 1.0 * q20
            + 1.0 * q21
            - 1.0 * q22
            + 1.0 * q23
            - 1.0 * q24
            - 1.0 * q25
            - 1.0 * q26
            + 1.0 * q7
            + 1.0 * q8
            - 1.0 * q9;

    let uy = -1.0 * q10 + 1.0 * q15 + 1.0 * q16 - 1.0 * q17 - 1.0 * q18
        + 1.0 * q19
        + 1.0 * q20
        - 1.0 * q21
        + 1.0 * q22
        - 1.0 * q23
        - 1.0 * q24
        + 1.0 * q25
        - 1.0 * q26
        + 1.0 * q3
        - 1.0 * q4
        + 1.0 * q7
        - 1.0 * q8
        + 1.0 * q9;

    let uz = 1.0 * q11 - 1.0 * q12 + 1.0 * q13 - 1.0 * q14 + 1.0 * q15
        - 1.0 * q16
        + 1.0 * q17
        - 1.0 * q18
        + 1.0 * q19
        - 1.0 * q20
        + 1.0 * q21
        + 1.0 * q22
        - 1.0 * q23
        + 1.0 * q24
        - 1.0 * q25
        - 1.0 * q26
        + 1.0 * q5
        - 1.0 * q6;

    (density, [ux / density, uy / density, uz / density])
}
