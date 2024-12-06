pub fn mrt(
    f: [f32; 27],
    ux: f32,
    uy: f32,
    uz: f32,
    density: f32,
    riv: f32,
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
result[0] = 0.871800647655485*density*ux * ux + 5.55111512312578e-17*density*ux + 0.871800647655485*density*uy * uy + 5.55111512312578e-17*density*uy + 0.871800647655485*density*uz * uz + 5.55111512312578e-17*density*uz + 1.37183350925717*density + 1.0*q0 - 1.94174757281553*q1 - 1.96410934908596*q10 - 1.96410934908596*q11 - 1.96410934908596*q12 - 1.96410934908596*q13 - 1.96410934908596*q14 - 1.96410934908596*q15 - 1.96410934908596*q16 - 1.96410934908596*q17 - 1.96410934908596*q18 - 1.95387778164147*q19 - 1.94174757281553*q2 - 1.95387778164147*q20 - 1.95387778164147*q21 - 1.95387778164147*q22 - 1.95387778164147*q23 - 1.95387778164147*q24 - 1.95387778164147*q25 - 1.95387778164147*q26 - 1.94174757281553*q3 - 1.94174757281553*q4 - 1.94174757281553*q5 - 1.94174757281553*q6 - 1.96410934908596*q7 - 1.96410934908596*q8 - 1.96410934908596*q9;

result[1] = -0.333333333333333*density*riv*ux * ux + 0.166666666666667*density*riv*uy * uy + 0.166666666666667*density*riv*uz * uz + 0.211348867110769*density*ux * ux + 0.541830679447893*density*ux - 0.108548765646749*density*uy * uy + 1.38777878078145e-17*density*uy - 0.108548765646749*density*uz * uz + 6.93889390390723e-18*density*uz - 0.145300107942581*density + 0.333333333333333*q1*riv + 1.32362459546926*q1 + 0.166666666666667*q10*riv + 0.658430079073725*q10 + 0.166666666666667*q11*riv - 1.28331749374181*q11 + 0.166666666666667*q12*riv - 1.28331749374181*q12 + 0.166666666666667*q13*riv + 0.658430079073725*q13 + 0.166666666666667*q14*riv + 0.658430079073725*q14 - 0.333333333333333*q15*riv + 0.647249190938511*q15 - 0.333333333333333*q16*riv + 0.647249190938511*q16 - 0.333333333333333*q17*riv + 0.647249190938511*q17 - 0.333333333333333*q18*riv + 0.647249190938511*q18 - 0.998096753122219*q19 + 0.333333333333333*q2*riv + 0.323624595469256*q2 - 0.998096753122219*q20 - 0.998096753122219*q21 + 0.98786518567773*q22 - 0.998096753122219*q23 + 0.98786518567773*q24 + 0.98786518567773*q25 + 0.98786518567773*q26 - 0.166666666666667*q3*riv + 0.323624595469256*q3 - 0.166666666666667*q4*riv + 0.323624595469256*q4 - 0.166666666666667*q5*riv + 0.323624595469256*q5 - 0.166666666666667*q6*riv + 0.323624595469256*q6 + 0.166666666666667*q7*riv - 1.28331749374181*q7 + 0.166666666666667*q8*riv - 1.28331749374181*q8 + 0.166666666666667*q9*riv + 0.658430079073725*q9;

result[2] = -0.333333333333333*density*riv*ux * ux + 0.166666666666667*density*riv*uy * uy + 0.166666666666667*density*riv*uz * uz + 0.211348867110769*density*ux * ux - 0.541830679447893*density*ux - 0.108548765646749*density*uy * uy + 1.38777878078145e-17*density*uy - 0.108548765646749*density*uz * uz + 6.93889390390723e-18*density*uz - 0.145300107942581*density + 0.333333333333333*q1*riv + 0.323624595469256*q1 + 0.166666666666667*q10*riv - 1.28331749374181*q10 + 0.166666666666667*q11*riv + 0.658430079073725*q11 + 0.166666666666667*q12*riv + 0.658430079073725*q12 + 0.166666666666667*q13*riv - 1.28331749374181*q13 + 0.166666666666667*q14*riv - 1.28331749374181*q14 - 0.333333333333333*q15*riv + 0.647249190938511*q15 - 0.333333333333333*q16*riv + 0.647249190938511*q16 - 0.333333333333333*q17*riv + 0.647249190938511*q17 - 0.333333333333333*q18*riv + 0.647249190938511*q18 + 0.98786518567773*q19 + 0.333333333333333*q2*riv + 1.32362459546926*q2 + 0.98786518567773*q20 + 0.98786518567773*q21 - 0.998096753122219*q22 + 0.98786518567773*q23 - 0.998096753122219*q24 - 0.998096753122219*q25 - 0.998096753122219*q26 - 0.166666666666667*q3*riv + 0.323624595469256*q3 - 0.166666666666667*q4*riv + 0.323624595469256*q4 - 0.166666666666667*q5*riv + 0.323624595469256*q5 - 0.166666666666667*q6*riv + 0.323624595469256*q6 + 0.166666666666667*q7*riv + 0.658430079073725*q7 + 0.166666666666667*q8*riv + 0.658430079073725*q8 + 0.166666666666667*q9*riv - 1.28331749374181*q9;

result[3] = 0.166666666666667*density*riv*ux * ux - 0.333333333333333*density*riv*uy * uy + 0.166666666666667*density*riv*uz * uz - 0.108548765646748*density*ux * ux - 1.38777878078145e-17*density*ux + 0.211348867110769*density*uy * uy + 0.541830679447893*density*uy - 0.108548765646748*density*uz * uz - 6.93889390390723e-18*density*uz - 0.145300107942581*density - 0.166666666666667*q1*riv + 0.323624595469256*q1 + 0.166666666666667*q10*riv + 0.658430079073725*q10 - 0.333333333333333*q11*riv + 0.647249190938511*q11 - 0.333333333333333*q12*riv + 0.647249190938511*q12 - 0.333333333333333*q13*riv + 0.647249190938511*q13 - 0.333333333333333*q14*riv + 0.647249190938511*q14 + 0.166666666666667*q15*riv - 1.28331749374181*q15 + 0.166666666666667*q16*riv - 1.28331749374181*q16 + 0.166666666666667*q17*riv + 0.658430079073725*q17 + 0.166666666666667*q18*riv + 0.658430079073725*q18 - 0.998096753122219*q19 - 0.166666666666667*q2*riv + 0.323624595469256*q2 - 0.998096753122219*q20 + 0.98786518567773*q21 - 0.998096753122219*q22 + 0.98786518567773*q23 + 0.98786518567773*q24 - 0.998096753122219*q25 + 0.98786518567773*q26 + 0.333333333333333*q3*riv + 1.32362459546926*q3 + 0.333333333333333*q4*riv + 0.323624595469256*q4 - 0.166666666666667*q5*riv + 0.323624595469256*q5 - 0.166666666666667*q6*riv + 0.323624595469256*q6 + 0.166666666666667*q7*riv - 1.28331749374181*q7 + 0.166666666666667*q8*riv + 0.658430079073725*q8 + 0.166666666666667*q9*riv - 1.28331749374181*q9;

result[4] = 0.166666666666667*density*riv*ux * ux - 0.333333333333333*density*riv*uy * uy + 0.166666666666667*density*riv*uz * uz - 0.108548765646748*density*ux * ux - 2.77555756156289e-17*density*ux*uy + 1.38777878078145e-17*density*ux + 0.211348867110769*density*uy * uy - 0.541830679447893*density*uy - 0.108548765646749*density*uz * uz - 2.08166817117217e-17*density*uz - 0.145300107942581*density - 0.166666666666667*q1*riv + 0.323624595469256*q1 + 0.166666666666667*q10*riv - 1.28331749374181*q10 - 0.333333333333333*q11*riv + 0.647249190938511*q11 - 0.333333333333333*q12*riv + 0.647249190938511*q12 - 0.333333333333333*q13*riv + 0.647249190938511*q13 - 0.333333333333333*q14*riv + 0.647249190938511*q14 + 0.166666666666667*q15*riv + 0.658430079073725*q15 + 0.166666666666667*q16*riv + 0.658430079073725*q16 + 0.166666666666667*q17*riv - 1.28331749374181*q17 + 0.166666666666667*q18*riv - 1.28331749374181*q18 + 0.98786518567773*q19 - 0.166666666666667*q2*riv + 0.323624595469256*q2 + 0.98786518567773*q20 - 0.998096753122219*q21 + 0.98786518567773*q22 - 0.998096753122219*q23 - 0.998096753122219*q24 + 0.98786518567773*q25 - 0.998096753122219*q26 + 0.333333333333333*q3*riv + 0.323624595469256*q3 + 0.333333333333333*q4*riv + 1.32362459546926*q4 - 0.166666666666667*q5*riv + 0.323624595469256*q5 - 0.166666666666667*q6*riv + 0.323624595469256*q6 + 0.166666666666667*q7*riv + 0.658430079073725*q7 + 0.166666666666667*q8*riv - 1.28331749374181*q8 + 0.166666666666667*q9*riv + 0.658430079073725*q9;

result[5] = 0.166666666666667*density*riv*ux * ux + 0.166666666666667*density*riv*uy * uy - 0.333333333333333*density*riv*uz * uz - 0.108548765646748*density*ux * ux - 6.93889390390723e-18*density*ux - 0.108548765646748*density*uy * uy - 6.93889390390723e-18*density*uy + 0.211348867110769*density*uz * uz + 0.541830679447893*density*uz - 0.145300107942581*density - 0.166666666666667*q1*riv + 0.323624595469256*q1 - 0.333333333333333*q10*riv + 0.647249190938511*q10 + 0.166666666666667*q11*riv - 1.28331749374181*q11 + 0.166666666666667*q12*riv + 0.658430079073725*q12 + 0.166666666666667*q13*riv - 1.28331749374181*q13 + 0.166666666666667*q14*riv + 0.658430079073725*q14 + 0.166666666666667*q15*riv - 1.28331749374181*q15 + 0.166666666666667*q16*riv + 0.658430079073725*q16 + 0.166666666666667*q17*riv - 1.28331749374181*q17 + 0.166666666666667*q18*riv + 0.658430079073725*q18 - 0.998096753122219*q19 - 0.166666666666667*q2*riv + 0.323624595469256*q2 + 0.98786518567773*q20 - 0.998096753122219*q21 - 0.998096753122219*q22 + 0.98786518567773*q23 - 0.998096753122219*q24 + 0.98786518567773*q25 + 0.98786518567773*q26 - 0.166666666666667*q3*riv + 0.323624595469256*q3 - 0.166666666666667*q4*riv + 0.323624595469256*q4 + 0.333333333333333*q5*riv + 1.32362459546926*q5 + 0.333333333333333*q6*riv + 0.323624595469256*q6 - 0.333333333333333*q7*riv + 0.647249190938511*q7 - 0.333333333333333*q8*riv + 0.647249190938511*q8 - 0.333333333333333*q9*riv + 0.647249190938511*q9;

result[6] = 0.166666666666667*density*riv*ux * ux + 0.166666666666667*density*riv*uy * uy - 0.333333333333333*density*riv*uz * uz - 0.108548765646748*density*ux * ux - 2.08166817117217e-17*density*ux - 0.108548765646749*density*uy * uy - 2.08166817117217e-17*density*uy + 0.211348867110769*density*uz * uz - 0.541830679447893*density*uz - 0.145300107942581*density - 0.166666666666667*q1*riv + 0.323624595469256*q1 - 0.333333333333333*q10*riv + 0.647249190938511*q10 + 0.166666666666667*q11*riv + 0.658430079073725*q11 + 0.166666666666667*q12*riv - 1.28331749374181*q12 + 0.166666666666667*q13*riv + 0.658430079073725*q13 + 0.166666666666667*q14*riv - 1.28331749374181*q14 + 0.166666666666667*q15*riv + 0.658430079073725*q15 + 0.166666666666667*q16*riv - 1.28331749374181*q16 + 0.166666666666667*q17*riv + 0.658430079073725*q17 + 0.166666666666667*q18*riv - 1.28331749374181*q18 + 0.98786518567773*q19 - 0.166666666666667*q2*riv + 0.323624595469256*q2 - 0.998096753122219*q20 + 0.98786518567773*q21 + 0.98786518567773*q22 - 0.998096753122219*q23 + 0.98786518567773*q24 - 0.998096753122219*q25 - 0.998096753122219*q26 - 0.166666666666667*q3*riv + 0.323624595469256*q3 - 0.166666666666667*q4*riv + 0.323624595469256*q4 + 0.333333333333333*q5*riv + 0.323624595469256*q5 + 0.333333333333333*q6*riv + 1.32362459546926*q6 - 0.333333333333333*q7*riv + 0.647249190938511*q7 - 0.333333333333333*q8*riv + 0.647249190938511*q8 - 0.333333333333333*q9*riv + 0.647249190938511*q9;

result[7] = -5.20417042793042e-18*density*riv*ux * ux - 0.25*density*riv*ux*uy + 1.73472347597681e-18*density*riv*ux - 5.20417042793042e-18*density*riv*uy * uy + 1.73472347597681e-18*density*riv*uy - 0.107537914911254*density*ux * ux + 0.159948816378759*density*ux*uy - 0.109103041989319*density*ux - 0.107537914911254*density*uy * uy - 0.109103041989319*density*uy + 0.0524109014675052*density*uz * uz - 0.0358459716370845*density + 0.25*q10*riv - 0.491027337271491*q10 + 0.25*q19*riv - 0.44959093021534*q19 + 0.25*q20*riv - 0.44959093021534*q20 - 0.25*q21*riv + 0.487994785065006*q21 - 0.25*q22*riv + 0.487994785065006*q22 - 0.25*q23*riv + 0.487994785065006*q23 + 0.25*q24*riv - 0.493805296199754*q24 - 0.25*q25*riv + 0.487994785065006*q25 + 0.25*q26*riv - 0.493805296199754*q26 + 0.25*q7*riv + 2.45072023554404*q7 - 0.25*q8*riv + 0.479846449136276*q8 - 0.25*q9*riv + 0.479846449136276*q9;

result[8] = -5.20417042793042e-18*density*riv*ux * ux + 0.25*density*riv*ux*uy + 1.73472347597681e-18*density*riv*ux - 5.20417042793042e-18*density*riv*uy * uy + 1.73472347597681e-18*density*riv*uy - 0.107537914911254*density*ux * ux - 0.159948816378759*density*ux*uy - 3.46944695195361e-18*density*ux*uz - 0.109103041989319*density*ux - 0.107537914911254*density*uy * uy + 0.109103041989319*density*uy + 0.0524109014675052*density*uz * uz - 0.0358459716370845*density - 0.25*q10*riv + 0.479846449136276*q10 - 0.25*q19*riv + 0.487994785065006*q19 - 0.25*q20*riv + 0.487994785065006*q20 + 0.25*q21*riv - 0.44959093021534*q21 + 0.25*q22*riv - 0.493805296199754*q22 + 0.25*q23*riv - 0.44959093021534*q23 - 0.25*q24*riv + 0.487994785065006*q24 + 0.25*q25*riv - 0.493805296199754*q25 - 0.25*q26*riv + 0.487994785065006*q26 - 0.25*q7*riv + 0.479846449136276*q7 + 0.25*q8*riv + 2.45072023554404*q8 + 0.25*q9*riv - 0.491027337271491*q9;

result[9] = -5.20417042793042e-18*density*riv*ux * ux + 0.25*density*riv*ux*uy + 1.73472347597681e-18*density*riv*ux - 5.20417042793042e-18*density*riv*uy * uy + 1.73472347597681e-18*density*riv*uy - 0.107537914911254*density*ux * ux - 0.159948816378759*density*ux*uy + 0.109103041989319*density*ux - 0.107537914911254*density*uy * uy - 3.46944695195361e-18*density*uy*uz - 0.109103041989319*density*uy + 0.0524109014675052*density*uz * uz - 0.0358459716370845*density - 0.25*q10*riv + 0.479846449136276*q10 - 0.25*q19*riv + 0.487994785065006*q19 - 0.25*q20*riv + 0.487994785065006*q20 + 0.25*q21*riv - 0.493805296199754*q21 + 0.25*q22*riv - 0.44959093021534*q22 + 0.25*q23*riv - 0.493805296199754*q23 - 0.25*q24*riv + 0.487994785065006*q24 + 0.25*q25*riv - 0.44959093021534*q25 - 0.25*q26*riv + 0.487994785065006*q26 - 0.25*q7*riv + 0.479846449136276*q7 + 0.25*q8*riv - 0.491027337271491*q8 + 0.25*q9*riv + 2.45072023554404*q9;

result[10] = -5.20417042793042e-18*density*riv*ux * ux - 0.25*density*riv*ux*uy + 1.73472347597681e-18*density*riv*ux - 5.20417042793042e-18*density*riv*uy * uy + 1.73472347597681e-18*density*riv*uy - 0.107537914911254*density*ux * ux + 0.159948816378759*density*ux*uy - 3.46944695195361e-18*density*ux*uz + 0.109103041989319*density*ux - 0.107537914911254*density*uy * uy - 3.46944695195361e-18*density*uy*uz + 0.109103041989319*density*uy + 0.0524109014675052*density*uz * uz - 8.67361737988404e-19*density*uz - 0.0358459716370845*density + 0.25*q10*riv + 2.45072023554404*q10 + 0.25*q19*riv - 0.493805296199754*q19 + 0.25*q20*riv - 0.493805296199754*q20 - 0.25*q21*riv + 0.487994785065006*q21 - 0.25*q22*riv + 0.487994785065006*q22 - 0.25*q23*riv + 0.487994785065006*q23 + 0.25*q24*riv - 0.44959093021534*q24 - 0.25*q25*riv + 0.487994785065006*q25 + 0.25*q26*riv - 0.44959093021534*q26 + 0.25*q7*riv - 0.491027337271491*q7 - 0.25*q8*riv + 0.479846449136276*q8 - 0.25*q9*riv + 0.479846449136276*q9;

result[11] = -5.20417042793042e-18*density*riv*ux * ux - 0.25*density*riv*ux*uz + 1.73472347597681e-18*density*riv*ux - 5.20417042793042e-18*density*riv*uz * uz + 1.73472347597681e-18*density*riv*uz - 0.107537914911254*density*ux * ux + 0.159948816378759*density*ux*uz - 0.109103041989319*density*ux + 0.0524109014675052*density*uy * uy - 0.107537914911254*density*uz * uz - 0.109103041989319*density*uz - 0.0358459716370845*density + 0.25*q11*riv + 2.45072023554404*q11 - 0.25*q12*riv + 0.479846449136276*q12 - 0.25*q13*riv + 0.479846449136276*q13 + 0.25*q14*riv - 0.491027337271491*q14 + 0.25*q19*riv - 0.44959093021534*q19 - 0.25*q20*riv + 0.487994785065006*q20 + 0.25*q21*riv - 0.44959093021534*q21 - 0.25*q22*riv + 0.487994785065006*q22 - 0.25*q23*riv + 0.487994785065006*q23 - 0.25*q24*riv + 0.487994785065006*q24 + 0.25*q25*riv - 0.493805296199754*q25 + 0.25*q26*riv - 0.493805296199754*q26;

result[12] = -5.20417042793042e-18*density*riv*ux * ux + 0.25*density*riv*ux*uz + 1.73472347597681e-18*density*riv*ux - 5.20417042793042e-18*density*riv*uz * uz + 1.73472347597681e-18*density*riv*uz - 0.107537914911254*density*ux * ux - 3.46944695195361e-18*density*ux*uy - 0.159948816378759*density*ux*uz - 0.109103041989319*density*ux + 0.0524109014675052*density*uy * uy - 0.107537914911254*density*uz * uz + 0.109103041989319*density*uz - 0.0358459716370845*density - 0.25*q11*riv + 0.479846449136276*q11 + 0.25*q12*riv + 2.45072023554404*q12 + 0.25*q13*riv - 0.491027337271491*q13 - 0.25*q14*riv + 0.479846449136276*q14 - 0.25*q19*riv + 0.487994785065006*q19 + 0.25*q20*riv - 0.44959093021534*q20 - 0.25*q21*riv + 0.487994785065006*q21 + 0.25*q22*riv - 0.493805296199754*q22 + 0.25*q23*riv - 0.44959093021534*q23 + 0.25*q24*riv - 0.493805296199754*q24 - 0.25*q25*riv + 0.487994785065006*q25 - 0.25*q26*riv + 0.487994785065006*q26;

result[13] = -5.20417042793042e-18*density*riv*ux * ux + 0.25*density*riv*ux*uz + 1.73472347597681e-18*density*riv*ux - 5.20417042793042e-18*density*riv*uz * uz + 1.73472347597681e-18*density*riv*uz - 0.107537914911254*density*ux * ux - 0.159948816378759*density*ux*uz + 0.109103041989319*density*ux + 0.0524109014675052*density*uy * uy - 3.46944695195361e-18*density*uy*uz - 0.107537914911254*density*uz * uz - 0.109103041989319*density*uz - 0.0358459716370845*density - 0.25*q11*riv + 0.479846449136276*q11 + 0.25*q12*riv - 0.491027337271491*q12 + 0.25*q13*riv + 2.45072023554404*q13 - 0.25*q14*riv + 0.479846449136276*q14 - 0.25*q19*riv + 0.487994785065006*q19 + 0.25*q20*riv - 0.493805296199754*q20 - 0.25*q21*riv + 0.487994785065006*q21 + 0.25*q22*riv - 0.44959093021534*q22 + 0.25*q23*riv - 0.493805296199754*q23 + 0.25*q24*riv - 0.44959093021534*q24 - 0.25*q25*riv + 0.487994785065006*q25 - 0.25*q26*riv + 0.487994785065006*q26;

result[14] = -5.20417042793042e-18*density*riv*ux * ux - 0.25*density*riv*ux*uz + 1.73472347597681e-18*density*riv*ux - 5.20417042793042e-18*density*riv*uz * uz + 1.73472347597681e-18*density*riv*uz - 0.107537914911254*density*ux * ux - 3.46944695195361e-18*density*ux*uy + 0.159948816378759*density*ux*uz + 0.109103041989319*density*ux + 0.0524109014675052*density*uy * uy - 3.46944695195361e-18*density*uy*uz - 8.67361737988404e-19*density*uy - 0.107537914911254*density*uz * uz + 0.109103041989319*density*uz - 0.0358459716370845*density + 0.25*q11*riv - 0.491027337271491*q11 - 0.25*q12*riv + 0.479846449136276*q12 - 0.25*q13*riv + 0.479846449136276*q13 + 0.25*q14*riv + 2.45072023554404*q14 + 0.25*q19*riv - 0.493805296199754*q19 - 0.25*q20*riv + 0.487994785065006*q20 + 0.25*q21*riv - 0.493805296199754*q21 - 0.25*q22*riv + 0.487994785065006*q22 - 0.25*q23*riv + 0.487994785065006*q23 - 0.25*q24*riv + 0.487994785065006*q24 + 0.25*q25*riv - 0.44959093021534*q25 + 0.25*q26*riv - 0.44959093021534*q26;

result[15] = -5.20417042793042e-18*density*riv*uy * uy - 0.25*density*riv*uy*uz + 1.73472347597681e-18*density*riv*uy - 5.20417042793042e-18*density*riv*uz * uz + 1.73472347597681e-18*density*riv*uz + 0.0524109014675052*density*ux * ux - 0.107537914911254*density*uy * uy + 0.159948816378759*density*uy*uz - 0.109103041989319*density*uy - 0.107537914911254*density*uz * uz - 0.109103041989319*density*uz - 0.0358459716370845*density + 0.25*q15*riv + 2.45072023554404*q15 - 0.25*q16*riv + 0.479846449136276*q16 - 0.25*q17*riv + 0.479846449136276*q17 + 0.25*q18*riv - 0.491027337271491*q18 + 0.25*q19*riv - 0.44959093021534*q19 - 0.25*q20*riv + 0.487994785065006*q20 - 0.25*q21*riv + 0.487994785065006*q21 + 0.25*q22*riv - 0.44959093021534*q22 + 0.25*q23*riv - 0.493805296199754*q23 - 0.25*q24*riv + 0.487994785065006*q24 - 0.25*q25*riv + 0.487994785065006*q25 + 0.25*q26*riv - 0.493805296199754*q26;

result[16] = -5.20417042793042e-18*density*riv*uy * uy + 0.25*density*riv*uy*uz + 1.73472347597681e-18*density*riv*uy - 5.20417042793042e-18*density*riv*uz * uz + 1.73472347597681e-18*density*riv*uz + 0.0524109014675052*density*ux * ux - 3.46944695195361e-18*density*ux*uy - 0.107537914911254*density*uy * uy - 0.159948816378759*density*uy*uz - 0.109103041989319*density*uy - 0.107537914911254*density*uz * uz + 0.109103041989319*density*uz - 0.0358459716370845*density - 0.25*q15*riv + 0.479846449136276*q15 + 0.25*q16*riv + 2.45072023554404*q16 + 0.25*q17*riv - 0.491027337271491*q17 - 0.25*q18*riv + 0.479846449136276*q18 - 0.25*q19*riv + 0.487994785065006*q19 + 0.25*q20*riv - 0.44959093021534*q20 + 0.25*q21*riv - 0.493805296199754*q21 - 0.25*q22*riv + 0.487994785065006*q22 - 0.25*q23*riv + 0.487994785065006*q23 + 0.25*q24*riv - 0.493805296199754*q24 + 0.25*q25*riv - 0.44959093021534*q25 - 0.25*q26*riv + 0.487994785065006*q26;

result[17] = -5.20417042793042e-18*density*riv*uy * uy + 0.25*density*riv*uy*uz + 1.73472347597681e-18*density*riv*uy - 5.20417042793042e-18*density*riv*uz * uz + 1.73472347597681e-18*density*riv*uz + 0.0524109014675052*density*ux * ux - 3.46944695195361e-18*density*ux*uz - 0.107537914911254*density*uy * uy - 0.159948816378759*density*uy*uz + 0.109103041989319*density*uy - 0.107537914911254*density*uz * uz - 0.109103041989319*density*uz - 0.0358459716370845*density - 0.25*q15*riv + 0.479846449136276*q15 + 0.25*q16*riv - 0.491027337271491*q16 + 0.25*q17*riv + 2.45072023554404*q17 - 0.25*q18*riv + 0.479846449136276*q18 - 0.25*q19*riv + 0.487994785065006*q19 + 0.25*q20*riv - 0.493805296199754*q20 + 0.25*q21*riv - 0.44959093021534*q21 - 0.25*q22*riv + 0.487994785065006*q22 - 0.25*q23*riv + 0.487994785065006*q23 + 0.25*q24*riv - 0.44959093021534*q24 + 0.25*q25*riv - 0.493805296199754*q25 - 0.25*q26*riv + 0.487994785065006*q26;

result[18] = -5.20417042793042e-18*density*riv*uy * uy - 0.25*density*riv*uy*uz + 1.73472347597681e-18*density*riv*uy - 5.20417042793042e-18*density*riv*uz * uz + 1.73472347597681e-18*density*riv*uz + 0.0524109014675052*density*ux * ux - 3.46944695195361e-18*density*ux*uy - 8.67361737988404e-19*density*ux - 0.107537914911254*density*uy * uy + 0.159948816378759*density*uy*uz + 0.109103041989319*density*uy - 0.107537914911254*density*uz * uz + 0.109103041989319*density*uz - 0.0358459716370845*density + 0.25*q15*riv - 0.491027337271491*q15 - 0.25*q16*riv + 0.479846449136276*q16 - 0.25*q17*riv + 0.479846449136276*q17 + 0.25*q18*riv + 2.45072023554404*q18 + 0.25*q19*riv - 0.493805296199754*q19 - 0.25*q20*riv + 0.487994785065006*q20 - 0.25*q21*riv + 0.487994785065006*q21 + 0.25*q22*riv - 0.493805296199754*q22 + 0.25*q23*riv - 0.44959093021534*q23 - 0.25*q24*riv + 0.487994785065006*q24 - 0.25*q25*riv + 0.487994785065006*q25 + 0.25*q26*riv - 0.44959093021534*q26;

result[19] = -0.0262054507337526*density*ux * ux - 0.0799744081893794*density*ux*uy - 0.0799744081893794*density*ux*uz - 0.0263546278726544*density*ux - 0.0262054507337526*density*uy * uy - 0.0799744081893794*density*uy*uz - 0.0263546278726544*density*uy - 0.0262054507337526*density*uz * uz - 0.0263546278726544*density*uz - 0.00873515024458421*density + 2.9099121294718*q19 - 0.00960096371241645*q20 - 0.00960096371241645*q21 - 0.00960096371241645*q22 + 0.00145262778368718*q23 + 0.00145262778368718*q24 + 0.00145262778368718*q25 + 0.00132533114457659*q26;

result[20] = -0.0262054507337526*density*ux * ux - 0.0799744081893794*density*ux*uy + 0.0799744081893794*density*ux*uz - 0.0263546278726544*density*ux - 0.0262054507337526*density*uy * uy + 0.0799744081893794*density*uy*uz - 0.0263546278726544*density*uy - 0.0262054507337526*density*uz * uz + 0.0263546278726544*density*uz - 0.00873515024458421*density - 0.00960096371241645*q19 + 2.9099121294718*q20 + 0.00145262778368718*q21 + 0.00145262778368718*q22 - 0.00960096371241645*q23 + 0.00132533114457659*q24 - 0.00960096371241645*q25 + 0.00145262778368718*q26;

result[21] = -0.0262054507337526*density*ux * ux + 0.0799744081893794*density*ux*uy - 0.0799744081893794*density*ux*uz - 0.0263546278726544*density*ux - 0.0262054507337526*density*uy * uy + 0.0799744081893794*density*uy*uz + 0.0263546278726544*density*uy - 0.0262054507337526*density*uz * uz - 0.0263546278726544*density*uz - 0.00873515024458421*density - 0.00960096371241645*q19 + 0.00145262778368718*q20 + 2.9099121294718*q21 + 0.00145262778368718*q22 - 0.00960096371241645*q23 - 0.00960096371241645*q24 + 0.00132533114457659*q25 + 0.00145262778368718*q26;

result[22] = -0.0262054507337526*density*ux * ux + 0.0799744081893794*density*ux*uy + 0.0799744081893794*density*ux*uz + 0.0263546278726544*density*ux - 0.0262054507337526*density*uy * uy - 0.0799744081893794*density*uy*uz - 0.0263546278726544*density*uy - 0.0262054507337526*density*uz * uz - 0.0263546278726544*density*uz - 0.00873515024458421*density - 0.00960096371241645*q19 + 0.00145262778368718*q20 + 0.00145262778368718*q21 + 2.9099121294718*q22 + 0.00132533114457659*q23 - 0.00960096371241645*q24 - 0.00960096371241645*q25 + 0.00145262778368718*q26;

result[23] = -0.0262054507337526*density*ux * ux + 0.0799744081893794*density*ux*uy + 0.0799744081893794*density*ux*uz - 0.0263546278726544*density*ux - 0.0262054507337526*density*uy * uy - 0.0799744081893794*density*uy*uz + 0.0263546278726544*density*uy - 0.0262054507337526*density*uz * uz + 0.0263546278726544*density*uz - 0.00873515024458421*density + 0.00145262778368718*q19 - 0.00960096371241645*q20 - 0.00960096371241645*q21 + 0.00132533114457659*q22 + 2.9099121294718*q23 + 0.00145262778368718*q24 + 0.00145262778368718*q25 - 0.00960096371241645*q26;

result[24] = -0.0262054507337526*density*ux * ux - 0.0799744081893794*density*ux*uy + 0.0799744081893794*density*ux*uz + 0.0263546278726544*density*ux - 0.0262054507337526*density*uy * uy + 0.0799744081893794*density*uy*uz + 0.0263546278726544*density*uy - 0.0262054507337526*density*uz * uz - 0.0263546278726544*density*uz - 0.00873515024458421*density + 0.00145262778368718*q19 + 0.00132533114457659*q20 - 0.00960096371241645*q21 - 0.00960096371241645*q22 + 0.00145262778368718*q23 + 2.9099121294718*q24 + 0.00145262778368718*q25 - 0.00960096371241645*q26;

result[25] = -0.0262054507337526*density*ux * ux + 0.0799744081893794*density*ux*uy - 0.0799744081893794*density*ux*uz + 0.0263546278726544*density*ux - 0.0262054507337526*density*uy * uy + 0.0799744081893794*density*uy*uz - 0.0263546278726544*density*uy - 0.0262054507337526*density*uz * uz + 0.0263546278726544*density*uz - 0.00873515024458421*density + 0.00145262778368718*q19 - 0.00960096371241645*q20 + 0.00132533114457659*q21 - 0.00960096371241645*q22 + 0.00145262778368718*q23 + 0.00145262778368718*q24 + 2.9099121294718*q25 - 0.00960096371241645*q26;

result[26] = -0.0262054507337526*density*ux * ux - 0.0799744081893794*density*ux*uy - 0.0799744081893794*density*ux*uz + 0.0263546278726544*density*ux - 0.0262054507337526*density*uy * uy - 0.0799744081893794*density*uy*uz + 0.0263546278726544*density*uy - 0.0262054507337526*density*uz * uz + 0.0263546278726544*density*uz - 0.00873515024458421*density + 0.00132533114457659*q19 + 0.00145262778368718*q20 + 0.00145262778368718*q21 + 0.00145262778368718*q22 - 0.00960096371241645*q23 - 0.00960096371241645*q24 - 0.00960096371241645*q25 + 2.9099121294718*q26;

  result
}


