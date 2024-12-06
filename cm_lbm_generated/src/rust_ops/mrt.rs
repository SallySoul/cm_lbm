pub fn mrt(
    f: [f32; 27],
    ux: f32,
    uy: f32,
    uz: f32,
    density: f32,
    riv: f32,
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
    result[0] = 0.871800647655485 * density * ux * ux
        + 5.55111512312578e-17 * density * ux
        + 0.871800647655485 * density * uy * uy
        + 5.55111512312578e-17 * density * uy
        + 0.871800647655485 * density * uz * uz
        + 5.55111512312578e-17 * density * uz
        + 1.37183350925717 * density
        + 1.0 * f0
        - 1.94174757281553 * f1
        - 1.96410934908596 * f10
        - 1.96410934908596 * f11
        - 1.96410934908596 * f12
        - 1.96410934908596 * f13
        - 1.96410934908596 * f14
        - 1.96410934908596 * f15
        - 1.96410934908596 * f16
        - 1.96410934908596 * f17
        - 1.96410934908596 * f18
        - 1.95387778164147 * f19
        - 1.94174757281553 * f2
        - 1.95387778164147 * f20
        - 1.95387778164147 * f21
        - 1.95387778164147 * f22
        - 1.95387778164147 * f23
        - 1.95387778164147 * f24
        - 1.95387778164147 * f25
        - 1.95387778164147 * f26
        - 1.94174757281553 * f3
        - 1.94174757281553 * f4
        - 1.94174757281553 * f5
        - 1.94174757281553 * f6
        - 1.96410934908596 * f7
        - 1.96410934908596 * f8
        - 1.96410934908596 * f9;

    result[1] = -0.333333333333333 * density * riv * ux * ux
        + 0.166666666666667 * density * riv * uy * uy
        + 0.166666666666667 * density * riv * uz * uz
        + 0.211348867110769 * density * ux * ux
        + 0.541830679447893 * density * ux
        - 0.108548765646749 * density * uy * uy
        + 1.38777878078145e-17 * density * uy
        - 0.108548765646749 * density * uz * uz
        + 6.93889390390723e-18 * density * uz
        - 0.145300107942581 * density
        + 0.333333333333333 * f1 * riv
        + 1.32362459546926 * f1
        + 0.166666666666667 * f10 * riv
        + 0.658430079073725 * f10
        + 0.166666666666667 * f11 * riv
        - 1.28331749374181 * f11
        + 0.166666666666667 * f12 * riv
        - 1.28331749374181 * f12
        + 0.166666666666667 * f13 * riv
        + 0.658430079073725 * f13
        + 0.166666666666667 * f14 * riv
        + 0.658430079073725 * f14
        - 0.333333333333333 * f15 * riv
        + 0.647249190938511 * f15
        - 0.333333333333333 * f16 * riv
        + 0.647249190938511 * f16
        - 0.333333333333333 * f17 * riv
        + 0.647249190938511 * f17
        - 0.333333333333333 * f18 * riv
        + 0.647249190938511 * f18
        - 0.998096753122219 * f19
        + 0.333333333333333 * f2 * riv
        + 0.323624595469256 * f2
        - 0.998096753122219 * f20
        - 0.998096753122219 * f21
        + 0.98786518567773 * f22
        - 0.998096753122219 * f23
        + 0.98786518567773 * f24
        + 0.98786518567773 * f25
        + 0.98786518567773 * f26
        - 0.166666666666667 * f3 * riv
        + 0.323624595469256 * f3
        - 0.166666666666667 * f4 * riv
        + 0.323624595469256 * f4
        - 0.166666666666667 * f5 * riv
        + 0.323624595469256 * f5
        - 0.166666666666667 * f6 * riv
        + 0.323624595469256 * f6
        + 0.166666666666667 * f7 * riv
        - 1.28331749374181 * f7
        + 0.166666666666667 * f8 * riv
        - 1.28331749374181 * f8
        + 0.166666666666667 * f9 * riv
        + 0.658430079073725 * f9;

    result[2] = -0.333333333333333 * density * riv * ux * ux
        + 0.166666666666667 * density * riv * uy * uy
        + 0.166666666666667 * density * riv * uz * uz
        + 0.211348867110769 * density * ux * ux
        - 0.541830679447893 * density * ux
        - 0.108548765646749 * density * uy * uy
        + 1.38777878078145e-17 * density * uy
        - 0.108548765646749 * density * uz * uz
        + 6.93889390390723e-18 * density * uz
        - 0.145300107942581 * density
        + 0.333333333333333 * f1 * riv
        + 0.323624595469256 * f1
        + 0.166666666666667 * f10 * riv
        - 1.28331749374181 * f10
        + 0.166666666666667 * f11 * riv
        + 0.658430079073725 * f11
        + 0.166666666666667 * f12 * riv
        + 0.658430079073725 * f12
        + 0.166666666666667 * f13 * riv
        - 1.28331749374181 * f13
        + 0.166666666666667 * f14 * riv
        - 1.28331749374181 * f14
        - 0.333333333333333 * f15 * riv
        + 0.647249190938511 * f15
        - 0.333333333333333 * f16 * riv
        + 0.647249190938511 * f16
        - 0.333333333333333 * f17 * riv
        + 0.647249190938511 * f17
        - 0.333333333333333 * f18 * riv
        + 0.647249190938511 * f18
        + 0.98786518567773 * f19
        + 0.333333333333333 * f2 * riv
        + 1.32362459546926 * f2
        + 0.98786518567773 * f20
        + 0.98786518567773 * f21
        - 0.998096753122219 * f22
        + 0.98786518567773 * f23
        - 0.998096753122219 * f24
        - 0.998096753122219 * f25
        - 0.998096753122219 * f26
        - 0.166666666666667 * f3 * riv
        + 0.323624595469256 * f3
        - 0.166666666666667 * f4 * riv
        + 0.323624595469256 * f4
        - 0.166666666666667 * f5 * riv
        + 0.323624595469256 * f5
        - 0.166666666666667 * f6 * riv
        + 0.323624595469256 * f6
        + 0.166666666666667 * f7 * riv
        + 0.658430079073725 * f7
        + 0.166666666666667 * f8 * riv
        + 0.658430079073725 * f8
        + 0.166666666666667 * f9 * riv
        - 1.28331749374181 * f9;

    result[3] = 0.166666666666667 * density * riv * ux * ux
        - 0.333333333333333 * density * riv * uy * uy
        + 0.166666666666667 * density * riv * uz * uz
        - 0.108548765646748 * density * ux * ux
        - 1.38777878078145e-17 * density * ux
        + 0.211348867110769 * density * uy * uy
        + 0.541830679447893 * density * uy
        - 0.108548765646748 * density * uz * uz
        - 6.93889390390723e-18 * density * uz
        - 0.145300107942581 * density
        - 0.166666666666667 * f1 * riv
        + 0.323624595469256 * f1
        + 0.166666666666667 * f10 * riv
        + 0.658430079073725 * f10
        - 0.333333333333333 * f11 * riv
        + 0.647249190938511 * f11
        - 0.333333333333333 * f12 * riv
        + 0.647249190938511 * f12
        - 0.333333333333333 * f13 * riv
        + 0.647249190938511 * f13
        - 0.333333333333333 * f14 * riv
        + 0.647249190938511 * f14
        + 0.166666666666667 * f15 * riv
        - 1.28331749374181 * f15
        + 0.166666666666667 * f16 * riv
        - 1.28331749374181 * f16
        + 0.166666666666667 * f17 * riv
        + 0.658430079073725 * f17
        + 0.166666666666667 * f18 * riv
        + 0.658430079073725 * f18
        - 0.998096753122219 * f19
        - 0.166666666666667 * f2 * riv
        + 0.323624595469256 * f2
        - 0.998096753122219 * f20
        + 0.98786518567773 * f21
        - 0.998096753122219 * f22
        + 0.98786518567773 * f23
        + 0.98786518567773 * f24
        - 0.998096753122219 * f25
        + 0.98786518567773 * f26
        + 0.333333333333333 * f3 * riv
        + 1.32362459546926 * f3
        + 0.333333333333333 * f4 * riv
        + 0.323624595469256 * f4
        - 0.166666666666667 * f5 * riv
        + 0.323624595469256 * f5
        - 0.166666666666667 * f6 * riv
        + 0.323624595469256 * f6
        + 0.166666666666667 * f7 * riv
        - 1.28331749374181 * f7
        + 0.166666666666667 * f8 * riv
        + 0.658430079073725 * f8
        + 0.166666666666667 * f9 * riv
        - 1.28331749374181 * f9;

    result[4] = 0.166666666666667 * density * riv * ux * ux
        - 0.333333333333333 * density * riv * uy * uy
        + 0.166666666666667 * density * riv * uz * uz
        - 0.108548765646748 * density * ux * ux
        - 2.77555756156289e-17 * density * ux * uy
        + 1.38777878078145e-17 * density * ux
        + 0.211348867110769 * density * uy * uy
        - 0.541830679447893 * density * uy
        - 0.108548765646749 * density * uz * uz
        - 2.08166817117217e-17 * density * uz
        - 0.145300107942581 * density
        - 0.166666666666667 * f1 * riv
        + 0.323624595469256 * f1
        + 0.166666666666667 * f10 * riv
        - 1.28331749374181 * f10
        - 0.333333333333333 * f11 * riv
        + 0.647249190938511 * f11
        - 0.333333333333333 * f12 * riv
        + 0.647249190938511 * f12
        - 0.333333333333333 * f13 * riv
        + 0.647249190938511 * f13
        - 0.333333333333333 * f14 * riv
        + 0.647249190938511 * f14
        + 0.166666666666667 * f15 * riv
        + 0.658430079073725 * f15
        + 0.166666666666667 * f16 * riv
        + 0.658430079073725 * f16
        + 0.166666666666667 * f17 * riv
        - 1.28331749374181 * f17
        + 0.166666666666667 * f18 * riv
        - 1.28331749374181 * f18
        + 0.98786518567773 * f19
        - 0.166666666666667 * f2 * riv
        + 0.323624595469256 * f2
        + 0.98786518567773 * f20
        - 0.998096753122219 * f21
        + 0.98786518567773 * f22
        - 0.998096753122219 * f23
        - 0.998096753122219 * f24
        + 0.98786518567773 * f25
        - 0.998096753122219 * f26
        + 0.333333333333333 * f3 * riv
        + 0.323624595469256 * f3
        + 0.333333333333333 * f4 * riv
        + 1.32362459546926 * f4
        - 0.166666666666667 * f5 * riv
        + 0.323624595469256 * f5
        - 0.166666666666667 * f6 * riv
        + 0.323624595469256 * f6
        + 0.166666666666667 * f7 * riv
        + 0.658430079073725 * f7
        + 0.166666666666667 * f8 * riv
        - 1.28331749374181 * f8
        + 0.166666666666667 * f9 * riv
        + 0.658430079073725 * f9;

    result[5] = 0.166666666666667 * density * riv * ux * ux
        + 0.166666666666667 * density * riv * uy * uy
        - 0.333333333333333 * density * riv * uz * uz
        - 0.108548765646748 * density * ux * ux
        - 6.93889390390723e-18 * density * ux
        - 0.108548765646748 * density * uy * uy
        - 6.93889390390723e-18 * density * uy
        + 0.211348867110769 * density * uz * uz
        + 0.541830679447893 * density * uz
        - 0.145300107942581 * density
        - 0.166666666666667 * f1 * riv
        + 0.323624595469256 * f1
        - 0.333333333333333 * f10 * riv
        + 0.647249190938511 * f10
        + 0.166666666666667 * f11 * riv
        - 1.28331749374181 * f11
        + 0.166666666666667 * f12 * riv
        + 0.658430079073725 * f12
        + 0.166666666666667 * f13 * riv
        - 1.28331749374181 * f13
        + 0.166666666666667 * f14 * riv
        + 0.658430079073725 * f14
        + 0.166666666666667 * f15 * riv
        - 1.28331749374181 * f15
        + 0.166666666666667 * f16 * riv
        + 0.658430079073725 * f16
        + 0.166666666666667 * f17 * riv
        - 1.28331749374181 * f17
        + 0.166666666666667 * f18 * riv
        + 0.658430079073725 * f18
        - 0.998096753122219 * f19
        - 0.166666666666667 * f2 * riv
        + 0.323624595469256 * f2
        + 0.98786518567773 * f20
        - 0.998096753122219 * f21
        - 0.998096753122219 * f22
        + 0.98786518567773 * f23
        - 0.998096753122219 * f24
        + 0.98786518567773 * f25
        + 0.98786518567773 * f26
        - 0.166666666666667 * f3 * riv
        + 0.323624595469256 * f3
        - 0.166666666666667 * f4 * riv
        + 0.323624595469256 * f4
        + 0.333333333333333 * f5 * riv
        + 1.32362459546926 * f5
        + 0.333333333333333 * f6 * riv
        + 0.323624595469256 * f6
        - 0.333333333333333 * f7 * riv
        + 0.647249190938511 * f7
        - 0.333333333333333 * f8 * riv
        + 0.647249190938511 * f8
        - 0.333333333333333 * f9 * riv
        + 0.647249190938511 * f9;

    result[6] = 0.166666666666667 * density * riv * ux * ux
        + 0.166666666666667 * density * riv * uy * uy
        - 0.333333333333333 * density * riv * uz * uz
        - 0.108548765646748 * density * ux * ux
        - 2.08166817117217e-17 * density * ux
        - 0.108548765646749 * density * uy * uy
        - 2.08166817117217e-17 * density * uy
        + 0.211348867110769 * density * uz * uz
        - 0.541830679447893 * density * uz
        - 0.145300107942581 * density
        - 0.166666666666667 * f1 * riv
        + 0.323624595469256 * f1
        - 0.333333333333333 * f10 * riv
        + 0.647249190938511 * f10
        + 0.166666666666667 * f11 * riv
        + 0.658430079073725 * f11
        + 0.166666666666667 * f12 * riv
        - 1.28331749374181 * f12
        + 0.166666666666667 * f13 * riv
        + 0.658430079073725 * f13
        + 0.166666666666667 * f14 * riv
        - 1.28331749374181 * f14
        + 0.166666666666667 * f15 * riv
        + 0.658430079073725 * f15
        + 0.166666666666667 * f16 * riv
        - 1.28331749374181 * f16
        + 0.166666666666667 * f17 * riv
        + 0.658430079073725 * f17
        + 0.166666666666667 * f18 * riv
        - 1.28331749374181 * f18
        + 0.98786518567773 * f19
        - 0.166666666666667 * f2 * riv
        + 0.323624595469256 * f2
        - 0.998096753122219 * f20
        + 0.98786518567773 * f21
        + 0.98786518567773 * f22
        - 0.998096753122219 * f23
        + 0.98786518567773 * f24
        - 0.998096753122219 * f25
        - 0.998096753122219 * f26
        - 0.166666666666667 * f3 * riv
        + 0.323624595469256 * f3
        - 0.166666666666667 * f4 * riv
        + 0.323624595469256 * f4
        + 0.333333333333333 * f5 * riv
        + 0.323624595469256 * f5
        + 0.333333333333333 * f6 * riv
        + 1.32362459546926 * f6
        - 0.333333333333333 * f7 * riv
        + 0.647249190938511 * f7
        - 0.333333333333333 * f8 * riv
        + 0.647249190938511 * f8
        - 0.333333333333333 * f9 * riv
        + 0.647249190938511 * f9;

    result[7] = -5.20417042793042e-18 * density * riv * ux * ux
        - 0.25 * density * riv * ux * uy
        + 1.73472347597681e-18 * density * riv * ux
        - 5.20417042793042e-18 * density * riv * uy * uy
        + 1.73472347597681e-18 * density * riv * uy
        - 0.107537914911254 * density * ux * ux
        + 0.159948816378759 * density * ux * uy
        - 0.109103041989319 * density * ux
        - 0.107537914911254 * density * uy * uy
        - 0.109103041989319 * density * uy
        + 0.0524109014675052 * density * uz * uz
        - 0.0358459716370845 * density
        + 0.25 * f10 * riv
        - 0.491027337271491 * f10
        + 0.25 * f19 * riv
        - 0.44959093021534 * f19
        + 0.25 * f20 * riv
        - 0.44959093021534 * f20
        - 0.25 * f21 * riv
        + 0.487994785065006 * f21
        - 0.25 * f22 * riv
        + 0.487994785065006 * f22
        - 0.25 * f23 * riv
        + 0.487994785065006 * f23
        + 0.25 * f24 * riv
        - 0.493805296199754 * f24
        - 0.25 * f25 * riv
        + 0.487994785065006 * f25
        + 0.25 * f26 * riv
        - 0.493805296199754 * f26
        + 0.25 * f7 * riv
        + 2.45072023554404 * f7
        - 0.25 * f8 * riv
        + 0.479846449136276 * f8
        - 0.25 * f9 * riv
        + 0.479846449136276 * f9;

    result[8] = -5.20417042793042e-18 * density * riv * ux * ux
        + 0.25 * density * riv * ux * uy
        + 1.73472347597681e-18 * density * riv * ux
        - 5.20417042793042e-18 * density * riv * uy * uy
        + 1.73472347597681e-18 * density * riv * uy
        - 0.107537914911254 * density * ux * ux
        - 0.159948816378759 * density * ux * uy
        - 3.46944695195361e-18 * density * ux * uz
        - 0.109103041989319 * density * ux
        - 0.107537914911254 * density * uy * uy
        + 0.109103041989319 * density * uy
        + 0.0524109014675052 * density * uz * uz
        - 0.0358459716370845 * density
        - 0.25 * f10 * riv
        + 0.479846449136276 * f10
        - 0.25 * f19 * riv
        + 0.487994785065006 * f19
        - 0.25 * f20 * riv
        + 0.487994785065006 * f20
        + 0.25 * f21 * riv
        - 0.44959093021534 * f21
        + 0.25 * f22 * riv
        - 0.493805296199754 * f22
        + 0.25 * f23 * riv
        - 0.44959093021534 * f23
        - 0.25 * f24 * riv
        + 0.487994785065006 * f24
        + 0.25 * f25 * riv
        - 0.493805296199754 * f25
        - 0.25 * f26 * riv
        + 0.487994785065006 * f26
        - 0.25 * f7 * riv
        + 0.479846449136276 * f7
        + 0.25 * f8 * riv
        + 2.45072023554404 * f8
        + 0.25 * f9 * riv
        - 0.491027337271491 * f9;

    result[9] = -5.20417042793042e-18 * density * riv * ux * ux
        + 0.25 * density * riv * ux * uy
        + 1.73472347597681e-18 * density * riv * ux
        - 5.20417042793042e-18 * density * riv * uy * uy
        + 1.73472347597681e-18 * density * riv * uy
        - 0.107537914911254 * density * ux * ux
        - 0.159948816378759 * density * ux * uy
        + 0.109103041989319 * density * ux
        - 0.107537914911254 * density * uy * uy
        - 3.46944695195361e-18 * density * uy * uz
        - 0.109103041989319 * density * uy
        + 0.0524109014675052 * density * uz * uz
        - 0.0358459716370845 * density
        - 0.25 * f10 * riv
        + 0.479846449136276 * f10
        - 0.25 * f19 * riv
        + 0.487994785065006 * f19
        - 0.25 * f20 * riv
        + 0.487994785065006 * f20
        + 0.25 * f21 * riv
        - 0.493805296199754 * f21
        + 0.25 * f22 * riv
        - 0.44959093021534 * f22
        + 0.25 * f23 * riv
        - 0.493805296199754 * f23
        - 0.25 * f24 * riv
        + 0.487994785065006 * f24
        + 0.25 * f25 * riv
        - 0.44959093021534 * f25
        - 0.25 * f26 * riv
        + 0.487994785065006 * f26
        - 0.25 * f7 * riv
        + 0.479846449136276 * f7
        + 0.25 * f8 * riv
        - 0.491027337271491 * f8
        + 0.25 * f9 * riv
        + 2.45072023554404 * f9;

    result[10] = -5.20417042793042e-18 * density * riv * ux * ux
        - 0.25 * density * riv * ux * uy
        + 1.73472347597681e-18 * density * riv * ux
        - 5.20417042793042e-18 * density * riv * uy * uy
        + 1.73472347597681e-18 * density * riv * uy
        - 0.107537914911254 * density * ux * ux
        + 0.159948816378759 * density * ux * uy
        - 3.46944695195361e-18 * density * ux * uz
        + 0.109103041989319 * density * ux
        - 0.107537914911254 * density * uy * uy
        - 3.46944695195361e-18 * density * uy * uz
        + 0.109103041989319 * density * uy
        + 0.0524109014675052 * density * uz * uz
        - 8.67361737988404e-19 * density * uz
        - 0.0358459716370845 * density
        + 0.25 * f10 * riv
        + 2.45072023554404 * f10
        + 0.25 * f19 * riv
        - 0.493805296199754 * f19
        + 0.25 * f20 * riv
        - 0.493805296199754 * f20
        - 0.25 * f21 * riv
        + 0.487994785065006 * f21
        - 0.25 * f22 * riv
        + 0.487994785065006 * f22
        - 0.25 * f23 * riv
        + 0.487994785065006 * f23
        + 0.25 * f24 * riv
        - 0.44959093021534 * f24
        - 0.25 * f25 * riv
        + 0.487994785065006 * f25
        + 0.25 * f26 * riv
        - 0.44959093021534 * f26
        + 0.25 * f7 * riv
        - 0.491027337271491 * f7
        - 0.25 * f8 * riv
        + 0.479846449136276 * f8
        - 0.25 * f9 * riv
        + 0.479846449136276 * f9;

    result[11] = -5.20417042793042e-18 * density * riv * ux * ux
        - 0.25 * density * riv * ux * uz
        + 1.73472347597681e-18 * density * riv * ux
        - 5.20417042793042e-18 * density * riv * uz * uz
        + 1.73472347597681e-18 * density * riv * uz
        - 0.107537914911254 * density * ux * ux
        + 0.159948816378759 * density * ux * uz
        - 0.109103041989319 * density * ux
        + 0.0524109014675052 * density * uy * uy
        - 0.107537914911254 * density * uz * uz
        - 0.109103041989319 * density * uz
        - 0.0358459716370845 * density
        + 0.25 * f11 * riv
        + 2.45072023554404 * f11
        - 0.25 * f12 * riv
        + 0.479846449136276 * f12
        - 0.25 * f13 * riv
        + 0.479846449136276 * f13
        + 0.25 * f14 * riv
        - 0.491027337271491 * f14
        + 0.25 * f19 * riv
        - 0.44959093021534 * f19
        - 0.25 * f20 * riv
        + 0.487994785065006 * f20
        + 0.25 * f21 * riv
        - 0.44959093021534 * f21
        - 0.25 * f22 * riv
        + 0.487994785065006 * f22
        - 0.25 * f23 * riv
        + 0.487994785065006 * f23
        - 0.25 * f24 * riv
        + 0.487994785065006 * f24
        + 0.25 * f25 * riv
        - 0.493805296199754 * f25
        + 0.25 * f26 * riv
        - 0.493805296199754 * f26;

    result[12] = -5.20417042793042e-18 * density * riv * ux * ux
        + 0.25 * density * riv * ux * uz
        + 1.73472347597681e-18 * density * riv * ux
        - 5.20417042793042e-18 * density * riv * uz * uz
        + 1.73472347597681e-18 * density * riv * uz
        - 0.107537914911254 * density * ux * ux
        - 3.46944695195361e-18 * density * ux * uy
        - 0.159948816378759 * density * ux * uz
        - 0.109103041989319 * density * ux
        + 0.0524109014675052 * density * uy * uy
        - 0.107537914911254 * density * uz * uz
        + 0.109103041989319 * density * uz
        - 0.0358459716370845 * density
        - 0.25 * f11 * riv
        + 0.479846449136276 * f11
        + 0.25 * f12 * riv
        + 2.45072023554404 * f12
        + 0.25 * f13 * riv
        - 0.491027337271491 * f13
        - 0.25 * f14 * riv
        + 0.479846449136276 * f14
        - 0.25 * f19 * riv
        + 0.487994785065006 * f19
        + 0.25 * f20 * riv
        - 0.44959093021534 * f20
        - 0.25 * f21 * riv
        + 0.487994785065006 * f21
        + 0.25 * f22 * riv
        - 0.493805296199754 * f22
        + 0.25 * f23 * riv
        - 0.44959093021534 * f23
        + 0.25 * f24 * riv
        - 0.493805296199754 * f24
        - 0.25 * f25 * riv
        + 0.487994785065006 * f25
        - 0.25 * f26 * riv
        + 0.487994785065006 * f26;

    result[13] = -5.20417042793042e-18 * density * riv * ux * ux
        + 0.25 * density * riv * ux * uz
        + 1.73472347597681e-18 * density * riv * ux
        - 5.20417042793042e-18 * density * riv * uz * uz
        + 1.73472347597681e-18 * density * riv * uz
        - 0.107537914911254 * density * ux * ux
        - 0.159948816378759 * density * ux * uz
        + 0.109103041989319 * density * ux
        + 0.0524109014675052 * density * uy * uy
        - 3.46944695195361e-18 * density * uy * uz
        - 0.107537914911254 * density * uz * uz
        - 0.109103041989319 * density * uz
        - 0.0358459716370845 * density
        - 0.25 * f11 * riv
        + 0.479846449136276 * f11
        + 0.25 * f12 * riv
        - 0.491027337271491 * f12
        + 0.25 * f13 * riv
        + 2.45072023554404 * f13
        - 0.25 * f14 * riv
        + 0.479846449136276 * f14
        - 0.25 * f19 * riv
        + 0.487994785065006 * f19
        + 0.25 * f20 * riv
        - 0.493805296199754 * f20
        - 0.25 * f21 * riv
        + 0.487994785065006 * f21
        + 0.25 * f22 * riv
        - 0.44959093021534 * f22
        + 0.25 * f23 * riv
        - 0.493805296199754 * f23
        + 0.25 * f24 * riv
        - 0.44959093021534 * f24
        - 0.25 * f25 * riv
        + 0.487994785065006 * f25
        - 0.25 * f26 * riv
        + 0.487994785065006 * f26;

    result[14] = -5.20417042793042e-18 * density * riv * ux * ux
        - 0.25 * density * riv * ux * uz
        + 1.73472347597681e-18 * density * riv * ux
        - 5.20417042793042e-18 * density * riv * uz * uz
        + 1.73472347597681e-18 * density * riv * uz
        - 0.107537914911254 * density * ux * ux
        - 3.46944695195361e-18 * density * ux * uy
        + 0.159948816378759 * density * ux * uz
        + 0.109103041989319 * density * ux
        + 0.0524109014675052 * density * uy * uy
        - 3.46944695195361e-18 * density * uy * uz
        - 8.67361737988404e-19 * density * uy
        - 0.107537914911254 * density * uz * uz
        + 0.109103041989319 * density * uz
        - 0.0358459716370845 * density
        + 0.25 * f11 * riv
        - 0.491027337271491 * f11
        - 0.25 * f12 * riv
        + 0.479846449136276 * f12
        - 0.25 * f13 * riv
        + 0.479846449136276 * f13
        + 0.25 * f14 * riv
        + 2.45072023554404 * f14
        + 0.25 * f19 * riv
        - 0.493805296199754 * f19
        - 0.25 * f20 * riv
        + 0.487994785065006 * f20
        + 0.25 * f21 * riv
        - 0.493805296199754 * f21
        - 0.25 * f22 * riv
        + 0.487994785065006 * f22
        - 0.25 * f23 * riv
        + 0.487994785065006 * f23
        - 0.25 * f24 * riv
        + 0.487994785065006 * f24
        + 0.25 * f25 * riv
        - 0.44959093021534 * f25
        + 0.25 * f26 * riv
        - 0.44959093021534 * f26;

    result[15] = -5.20417042793042e-18 * density * riv * uy * uy
        - 0.25 * density * riv * uy * uz
        + 1.73472347597681e-18 * density * riv * uy
        - 5.20417042793042e-18 * density * riv * uz * uz
        + 1.73472347597681e-18 * density * riv * uz
        + 0.0524109014675052 * density * ux * ux
        - 0.107537914911254 * density * uy * uy
        + 0.159948816378759 * density * uy * uz
        - 0.109103041989319 * density * uy
        - 0.107537914911254 * density * uz * uz
        - 0.109103041989319 * density * uz
        - 0.0358459716370845 * density
        + 0.25 * f15 * riv
        + 2.45072023554404 * f15
        - 0.25 * f16 * riv
        + 0.479846449136276 * f16
        - 0.25 * f17 * riv
        + 0.479846449136276 * f17
        + 0.25 * f18 * riv
        - 0.491027337271491 * f18
        + 0.25 * f19 * riv
        - 0.44959093021534 * f19
        - 0.25 * f20 * riv
        + 0.487994785065006 * f20
        - 0.25 * f21 * riv
        + 0.487994785065006 * f21
        + 0.25 * f22 * riv
        - 0.44959093021534 * f22
        + 0.25 * f23 * riv
        - 0.493805296199754 * f23
        - 0.25 * f24 * riv
        + 0.487994785065006 * f24
        - 0.25 * f25 * riv
        + 0.487994785065006 * f25
        + 0.25 * f26 * riv
        - 0.493805296199754 * f26;

    result[16] = -5.20417042793042e-18 * density * riv * uy * uy
        + 0.25 * density * riv * uy * uz
        + 1.73472347597681e-18 * density * riv * uy
        - 5.20417042793042e-18 * density * riv * uz * uz
        + 1.73472347597681e-18 * density * riv * uz
        + 0.0524109014675052 * density * ux * ux
        - 3.46944695195361e-18 * density * ux * uy
        - 0.107537914911254 * density * uy * uy
        - 0.159948816378759 * density * uy * uz
        - 0.109103041989319 * density * uy
        - 0.107537914911254 * density * uz * uz
        + 0.109103041989319 * density * uz
        - 0.0358459716370845 * density
        - 0.25 * f15 * riv
        + 0.479846449136276 * f15
        + 0.25 * f16 * riv
        + 2.45072023554404 * f16
        + 0.25 * f17 * riv
        - 0.491027337271491 * f17
        - 0.25 * f18 * riv
        + 0.479846449136276 * f18
        - 0.25 * f19 * riv
        + 0.487994785065006 * f19
        + 0.25 * f20 * riv
        - 0.44959093021534 * f20
        + 0.25 * f21 * riv
        - 0.493805296199754 * f21
        - 0.25 * f22 * riv
        + 0.487994785065006 * f22
        - 0.25 * f23 * riv
        + 0.487994785065006 * f23
        + 0.25 * f24 * riv
        - 0.493805296199754 * f24
        + 0.25 * f25 * riv
        - 0.44959093021534 * f25
        - 0.25 * f26 * riv
        + 0.487994785065006 * f26;

    result[17] = -5.20417042793042e-18 * density * riv * uy * uy
        + 0.25 * density * riv * uy * uz
        + 1.73472347597681e-18 * density * riv * uy
        - 5.20417042793042e-18 * density * riv * uz * uz
        + 1.73472347597681e-18 * density * riv * uz
        + 0.0524109014675052 * density * ux * ux
        - 3.46944695195361e-18 * density * ux * uz
        - 0.107537914911254 * density * uy * uy
        - 0.159948816378759 * density * uy * uz
        + 0.109103041989319 * density * uy
        - 0.107537914911254 * density * uz * uz
        - 0.109103041989319 * density * uz
        - 0.0358459716370845 * density
        - 0.25 * f15 * riv
        + 0.479846449136276 * f15
        + 0.25 * f16 * riv
        - 0.491027337271491 * f16
        + 0.25 * f17 * riv
        + 2.45072023554404 * f17
        - 0.25 * f18 * riv
        + 0.479846449136276 * f18
        - 0.25 * f19 * riv
        + 0.487994785065006 * f19
        + 0.25 * f20 * riv
        - 0.493805296199754 * f20
        + 0.25 * f21 * riv
        - 0.44959093021534 * f21
        - 0.25 * f22 * riv
        + 0.487994785065006 * f22
        - 0.25 * f23 * riv
        + 0.487994785065006 * f23
        + 0.25 * f24 * riv
        - 0.44959093021534 * f24
        + 0.25 * f25 * riv
        - 0.493805296199754 * f25
        - 0.25 * f26 * riv
        + 0.487994785065006 * f26;

    result[18] = -5.20417042793042e-18 * density * riv * uy * uy
        - 0.25 * density * riv * uy * uz
        + 1.73472347597681e-18 * density * riv * uy
        - 5.20417042793042e-18 * density * riv * uz * uz
        + 1.73472347597681e-18 * density * riv * uz
        + 0.0524109014675052 * density * ux * ux
        - 3.46944695195361e-18 * density * ux * uy
        - 8.67361737988404e-19 * density * ux
        - 0.107537914911254 * density * uy * uy
        + 0.159948816378759 * density * uy * uz
        + 0.109103041989319 * density * uy
        - 0.107537914911254 * density * uz * uz
        + 0.109103041989319 * density * uz
        - 0.0358459716370845 * density
        + 0.25 * f15 * riv
        - 0.491027337271491 * f15
        - 0.25 * f16 * riv
        + 0.479846449136276 * f16
        - 0.25 * f17 * riv
        + 0.479846449136276 * f17
        + 0.25 * f18 * riv
        + 2.45072023554404 * f18
        + 0.25 * f19 * riv
        - 0.493805296199754 * f19
        - 0.25 * f20 * riv
        + 0.487994785065006 * f20
        - 0.25 * f21 * riv
        + 0.487994785065006 * f21
        + 0.25 * f22 * riv
        - 0.493805296199754 * f22
        + 0.25 * f23 * riv
        - 0.44959093021534 * f23
        - 0.25 * f24 * riv
        + 0.487994785065006 * f24
        - 0.25 * f25 * riv
        + 0.487994785065006 * f25
        + 0.25 * f26 * riv
        - 0.44959093021534 * f26;

    result[19] = -0.0262054507337526 * density * ux * ux
        - 0.0799744081893794 * density * ux * uy
        - 0.0799744081893794 * density * ux * uz
        - 0.0263546278726544 * density * ux
        - 0.0262054507337526 * density * uy * uy
        - 0.0799744081893794 * density * uy * uz
        - 0.0263546278726544 * density * uy
        - 0.0262054507337526 * density * uz * uz
        - 0.0263546278726544 * density * uz
        - 0.00873515024458421 * density
        + 2.9099121294718 * f19
        - 0.00960096371241645 * f20
        - 0.00960096371241645 * f21
        - 0.00960096371241645 * f22
        + 0.00145262778368718 * f23
        + 0.00145262778368718 * f24
        + 0.00145262778368718 * f25
        + 0.00132533114457659 * f26;

    result[20] = -0.0262054507337526 * density * ux * ux
        - 0.0799744081893794 * density * ux * uy
        + 0.0799744081893794 * density * ux * uz
        - 0.0263546278726544 * density * ux
        - 0.0262054507337526 * density * uy * uy
        + 0.0799744081893794 * density * uy * uz
        - 0.0263546278726544 * density * uy
        - 0.0262054507337526 * density * uz * uz
        + 0.0263546278726544 * density * uz
        - 0.00873515024458421 * density
        - 0.00960096371241645 * f19
        + 2.9099121294718 * f20
        + 0.00145262778368718 * f21
        + 0.00145262778368718 * f22
        - 0.00960096371241645 * f23
        + 0.00132533114457659 * f24
        - 0.00960096371241645 * f25
        + 0.00145262778368718 * f26;

    result[21] = -0.0262054507337526 * density * ux * ux
        + 0.0799744081893794 * density * ux * uy
        - 0.0799744081893794 * density * ux * uz
        - 0.0263546278726544 * density * ux
        - 0.0262054507337526 * density * uy * uy
        + 0.0799744081893794 * density * uy * uz
        + 0.0263546278726544 * density * uy
        - 0.0262054507337526 * density * uz * uz
        - 0.0263546278726544 * density * uz
        - 0.00873515024458421 * density
        - 0.00960096371241645 * f19
        + 0.00145262778368718 * f20
        + 2.9099121294718 * f21
        + 0.00145262778368718 * f22
        - 0.00960096371241645 * f23
        - 0.00960096371241645 * f24
        + 0.00132533114457659 * f25
        + 0.00145262778368718 * f26;

    result[22] = -0.0262054507337526 * density * ux * ux
        + 0.0799744081893794 * density * ux * uy
        + 0.0799744081893794 * density * ux * uz
        + 0.0263546278726544 * density * ux
        - 0.0262054507337526 * density * uy * uy
        - 0.0799744081893794 * density * uy * uz
        - 0.0263546278726544 * density * uy
        - 0.0262054507337526 * density * uz * uz
        - 0.0263546278726544 * density * uz
        - 0.00873515024458421 * density
        - 0.00960096371241645 * f19
        + 0.00145262778368718 * f20
        + 0.00145262778368718 * f21
        + 2.9099121294718 * f22
        + 0.00132533114457659 * f23
        - 0.00960096371241645 * f24
        - 0.00960096371241645 * f25
        + 0.00145262778368718 * f26;

    result[23] = -0.0262054507337526 * density * ux * ux
        + 0.0799744081893794 * density * ux * uy
        + 0.0799744081893794 * density * ux * uz
        - 0.0263546278726544 * density * ux
        - 0.0262054507337526 * density * uy * uy
        - 0.0799744081893794 * density * uy * uz
        + 0.0263546278726544 * density * uy
        - 0.0262054507337526 * density * uz * uz
        + 0.0263546278726544 * density * uz
        - 0.00873515024458421 * density
        + 0.00145262778368718 * f19
        - 0.00960096371241645 * f20
        - 0.00960096371241645 * f21
        + 0.00132533114457659 * f22
        + 2.9099121294718 * f23
        + 0.00145262778368718 * f24
        + 0.00145262778368718 * f25
        - 0.00960096371241645 * f26;

    result[24] = -0.0262054507337526 * density * ux * ux
        - 0.0799744081893794 * density * ux * uy
        + 0.0799744081893794 * density * ux * uz
        + 0.0263546278726544 * density * ux
        - 0.0262054507337526 * density * uy * uy
        + 0.0799744081893794 * density * uy * uz
        + 0.0263546278726544 * density * uy
        - 0.0262054507337526 * density * uz * uz
        - 0.0263546278726544 * density * uz
        - 0.00873515024458421 * density
        + 0.00145262778368718 * f19
        + 0.00132533114457659 * f20
        - 0.00960096371241645 * f21
        - 0.00960096371241645 * f22
        + 0.00145262778368718 * f23
        + 2.9099121294718 * f24
        + 0.00145262778368718 * f25
        - 0.00960096371241645 * f26;

    result[25] = -0.0262054507337526 * density * ux * ux
        + 0.0799744081893794 * density * ux * uy
        - 0.0799744081893794 * density * ux * uz
        + 0.0263546278726544 * density * ux
        - 0.0262054507337526 * density * uy * uy
        + 0.0799744081893794 * density * uy * uz
        - 0.0263546278726544 * density * uy
        - 0.0262054507337526 * density * uz * uz
        + 0.0263546278726544 * density * uz
        - 0.00873515024458421 * density
        + 0.00145262778368718 * f19
        - 0.00960096371241645 * f20
        + 0.00132533114457659 * f21
        - 0.00960096371241645 * f22
        + 0.00145262778368718 * f23
        + 0.00145262778368718 * f24
        + 2.9099121294718 * f25
        - 0.00960096371241645 * f26;

    result[26] = -0.0262054507337526 * density * ux * ux
        - 0.0799744081893794 * density * ux * uy
        - 0.0799744081893794 * density * ux * uz
        + 0.0263546278726544 * density * ux
        - 0.0262054507337526 * density * uy * uy
        - 0.0799744081893794 * density * uy * uz
        + 0.0263546278726544 * density * uy
        - 0.0262054507337526 * density * uz * uz
        + 0.0263546278726544 * density * uz
        - 0.00873515024458421 * density
        + 0.00132533114457659 * f19
        + 0.00145262778368718 * f20
        + 0.00145262778368718 * f21
        + 0.00145262778368718 * f22
        - 0.00960096371241645 * f23
        - 0.00960096371241645 * f24
        - 0.00960096371241645 * f25
        + 2.9099121294718 * f26;

    result
}
