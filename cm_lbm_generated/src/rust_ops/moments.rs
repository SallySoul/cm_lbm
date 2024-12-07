pub fn moments(f: [f32; 27]) -> (f32, [f32; 3]) {
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

    let ux = q1 - q10 + q11 + q12 - q13 - q14 + q19 - q2 + q20 + q21 - q22
        + q23
        - q24
        - q25
        - q26
        + q7
        + q8
        - q9;

    let uy = -q10 + q15 + q16 - q17 - q18 + q19 + q20 - q21 + q22 - q23 - q24
        + q25
        - q26
        + q3
        - q4
        + q7
        - q8
        + q9;

    let uz =
        q11 - q12 + q13 - q14 + q15 - q16 + q17 - q18 + q19 - q20 + q21 + q22
            - q23
            + q24
            - q25
            - q26
            + q5
            - q6;

    (density, [ux / density, uy / density, uz / density])
}
