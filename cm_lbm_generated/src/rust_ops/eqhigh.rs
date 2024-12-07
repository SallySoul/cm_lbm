pub fn eqhigh(ux: f32, uy: f32, uz: f32, density: f32) -> [f32; 27] {
    let mut result = [0.0; 27];
    result[0] = density
        * (-1.0 * ux * ux * uy * uy * uz * uz
            + 0.666666666666667 * ux * ux * uy * uy
            + 3.86193626520161e-16 * ux * ux * uy * uz
            - 6.43656044200268e-17 * ux * ux * uy
            + 0.666666666666667 * ux * ux * uz * uz
            - 0.444444444444444 * ux * ux
            + 1.28731208840054e-16 * ux * uy * uy * uz * uz
            + 3.86193626520161e-16 * ux * uy * uy * uz
            + 3.86193626520161e-16 * ux * uy * uz * uz
            + 6.43656044200268e-17 * ux * uy
            + 0.666666666666667 * uy * uy * uz * uz
            - 6.43656044200268e-17 * uy * uy * uz
            - 0.444444444444444 * uy * uy
            - 6.43656044200268e-17 * uy * uz * uz
            + 6.43656044200268e-17 * uy * uz
            - 0.444444444444444 * uz * uz
            + 0.296296296296296);

    result[1] = density
        * (0.5 * ux * ux * uy * uy * uz * uz
            - 0.333333333333333 * ux * ux * uy * uy
            - 2.85746701977739e-16 * ux * ux * uy * uz
            + 3.57183377472174e-17 * ux * ux * uy
            - 0.333333333333333 * ux * ux * uz * uz
            + 3.57183377472174e-17 * ux * ux * uz
            + 0.222222222222222 * ux * ux
            + 0.5 * ux * uy * uy * uz * uz
            - 2.14310026483304e-16 * ux * uy * uy * uz
            - 0.333333333333333 * ux * uy * uy
            - 2.14310026483304e-16 * ux * uy * uz * uz
            - 3.57183377472174e-17 * ux * uy
            - 0.333333333333333 * ux * uz * uz
            - 3.57183377472174e-17 * ux * uz
            + 0.222222222222222 * ux
            + 0.166666666666667 * uy * uy * uz * uz
            + 1.07155013241652e-16 * uy * uy * uz
            - 0.111111111111111 * uy * uy
            + 7.14366754944348e-17 * uy * uz * uz
            - 7.14366754944348e-17 * uy * uz
            - 0.111111111111111 * uz * uz
            + 0.0740740740740741);

    result[2] = density
        * (0.5 * ux * ux * uy * uy * uz * uz
            - 0.333333333333333 * ux * ux * uy * uy
            - 1.85791804933454e-16 * ux * ux * uy * uz
            - 0.333333333333333 * ux * ux * uz * uz
            + 0.222222222222222 * ux * ux
            - 0.5 * ux * uy * uy * uz * uz
            - 9.28959024667269e-17 * ux * uy * uy * uz
            + 0.333333333333333 * ux * uy * uy
            - 9.28959024667269e-17 * ux * uy * uz * uz
            + 0.333333333333333 * ux * uz * uz
            - 0.222222222222222 * ux
            + 0.166666666666667 * uy * uy * uz * uz
            + 4.64479512333634e-17 * uy * uy * uz
            - 0.111111111111111 * uy * uy
            + 2.32239756166817e-17 * uy * uz * uz
            - 2.32239756166817e-17 * uy * uz
            - 0.111111111111111 * uz * uz
            + 0.0740740740740741);

    result[3] = density
        * (0.5 * ux * ux * uy * uy * uz * uz
            - 0.333333333333333 * ux * ux * uy * uy
            + 0.5 * ux * ux * uy * uz * uz
            - 2.37853679073478e-16 * ux * ux * uy * uz
            - 0.333333333333333 * ux * ux * uy
            + 0.166666666666667 * ux * ux * uz * uz
            - 0.111111111111111 * ux * ux
            - 5.94634197683696e-17 * ux * uy * uy * uz * uz
            - 1.78390259305109e-16 * ux * uy * uy * uz
            + 5.94634197683696e-17 * ux * uy * uy
            - 1.78390259305109e-16 * ux * uy * uz * uz
            - 1.78390259305109e-16 * ux * uy * uz
            + 2.97317098841848e-17 * ux * uy
            + 2.97317098841848e-17 * ux * uz * uz
            - 0.333333333333333 * uy * uy * uz * uz
            + 5.94634197683696e-17 * uy * uy * uz
            + 0.222222222222222 * uy * uy
            - 0.333333333333333 * uy * uz * uz
            + 2.97317098841848e-17 * uy * uz
            + 0.222222222222222 * uy
            - 0.111111111111111 * uz * uz
            + 0.0740740740740741);

    result[4] = density
        * (0.5 * ux * ux * uy * uy * uz * uz
            - 0.333333333333333 * ux * ux * uy * uy
            - 0.5 * ux * ux * uy * uz * uz
            - 2.09015780550136e-16 * ux * ux * uy * uz
            + 0.333333333333333 * ux * ux * uy
            + 0.166666666666667 * ux * ux * uz * uz
            + 2.61269725687669e-17 * ux * ux * uz
            - 0.111111111111111 * ux * ux
            - 1.04507890275068e-16 * ux * uy * uy * uz * uz
            - 1.04507890275068e-16 * ux * uy * uy * uz
            + 5.22539451375339e-17 * ux * uy * uy
            - 1.04507890275068e-16 * ux * uy * uz * uz
            - 2.61269725687669e-17 * ux * uy
            + 2.61269725687669e-17 * ux * uz * uz
            - 2.61269725687669e-17 * ux * uz
            - 0.333333333333333 * uy * uy * uz * uz
            + 7.83809177063008e-17 * uy * uy * uz
            + 0.222222222222222 * uy * uy
            + 0.333333333333333 * uy * uz * uz
            - 5.22539451375339e-17 * uy * uz
            - 0.222222222222222 * uy
            - 0.111111111111111 * uz * uz
            + 0.0740740740740741);

    result[5] = density
        * (0.5 * ux * ux * uy * uy * uz * uz
            + 0.5 * ux * ux * uy * uy * uz
            + 0.166666666666667 * ux * ux * uy * uy
            - 2.25775171933028e-16 * ux * ux * uy * uz
            + 2.82218964916285e-17 * ux * ux * uy
            - 0.333333333333333 * ux * ux * uz * uz
            - 0.333333333333333 * ux * ux * uz
            - 0.111111111111111 * ux * ux
            - 5.64437929832571e-17 * ux * uy * uy * uz * uz
            - 1.69331378949771e-16 * ux * uy * uy * uz
            + 5.64437929832571e-17 * ux * uy * uy
            - 1.69331378949771e-16 * ux * uy * uz * uz
            + 1.12887585966514e-16 * ux * uy * uz
            - 2.82218964916285e-17 * ux * uy
            + 2.82218964916285e-17 * ux * uz * uz
            - 2.82218964916285e-17 * ux * uz
            - 0.333333333333333 * uy * uy * uz * uz
            - 0.333333333333333 * uy * uy * uz
            - 0.111111111111111 * uy * uy
            + 5.64437929832571e-17 * uy * uz * uz
            - 5.64437929832571e-17 * uy * uz
            + 0.222222222222222 * uz * uz
            + 0.222222222222222 * uz
            + 0.0740740740740741);

    result[6] = density
        * (0.5 * ux * ux * uy * uy * uz * uz - 0.5 * ux * ux * uy * uy * uz
            + 0.166666666666667 * ux * ux * uy * uy
            - 2.11867438185395e-16 * ux * ux * uy * uz
            - 0.333333333333333 * ux * ux * uz * uz
            + 0.333333333333333 * ux * ux * uz
            - 0.111111111111111 * ux * ux
            - 7.06224793951318e-17 * ux * uy * uy * uz * uz
            - 1.41244958790264e-16 * ux * uy * uy * uz
            + 7.06224793951318e-17 * ux * uy * uy
            - 1.41244958790264e-16 * ux * uy * uz * uz
            + 3.53112396975659e-17 * ux * uz * uz
            - 3.53112396975659e-17 * ux * uz
            - 0.333333333333333 * uy * uy * uz * uz
            + 0.333333333333333 * uy * uy * uz
            - 0.111111111111111 * uy * uy
            + 3.53112396975659e-17 * uy * uz * uz
            - 3.53112396975659e-17 * uy * uz
            + 0.222222222222222 * uz * uz
            - 0.222222222222222 * uz
            + 0.0740740740740741);

    result[7] = density
        * (-0.0833333333333333 * ux * ux * uy * uy
            - 0.0833333333333334 * ux * ux * uy
            - 0.0833333333333333 * ux * ux * uz * uz
            - 2.22987824131386e-17 * ux * ux * uz
            + 0.0555555555555556 * ux * ux
            - 0.0833333333333334 * ux * uy * uy
            - ux * uy
                * (0.25 * ux * uy * uz * uz
                    + 4.45975648262772e-17 * ux * uy * uz
                    - 0.25 * ux * uy
                    + 0.25 * ux * uz * uz
                    - 1.33792694478832e-16 * ux * uz
                    - 0.25 * ux
                    + 0.25 * uy * uz * uz
                    - 1.33792694478832e-16 * uy * uz
                    - 0.25 * uy
                    + 0.25 * uz * uz
                    + 4.45975648262772e-17 * uz
                    - 0.25)
            - 0.0833333333333333 * ux * uy
            - 0.0833333333333334 * ux * uz * uz
            + 2.22987824131386e-17 * ux * uz
            + 0.0555555555555556 * ux
            - 0.0833333333333333 * uy * uy * uz * uz
            - 4.45975648262772e-17 * uy * uy * uz
            + 0.0555555555555556 * uy * uy
            - 0.0833333333333334 * uy * uz * uz
            + 3.34481736197079e-17 * uy * uz
            + 0.0555555555555556 * uy
            - 0.0277777777777778 * uz * uz
            - 4.95528498069747e-18 * uz
            + 0.0185185185185185);

    result[8] = density
        * (-0.0833333333333333 * ux * ux * uy * uy
            + 0.0833333333333333 * ux * ux * uy
            - 0.0833333333333334 * ux * ux * uz * uz
            - 9.7976147132876e-18 * ux * ux * uz
            + 0.0555555555555555 * ux * ux
            - 0.0833333333333334 * ux * uy * uy
            + ux * uy
                * (-0.25 * ux * uy * uz * uz
                    + 0.25 * ux * uy
                    + 0.25 * ux * uz * uz
                    + 1.17571376559451e-16 * ux * uz
                    - 0.25 * ux
                    - 0.25 * uy * uz * uz
                    + 7.83809177063008e-17 * uy * uz
                    + 0.25 * uy
                    + 0.25 * uz * uz
                    - 0.25)
            + 0.0833333333333333 * ux * uy
            - 0.0833333333333333 * ux * uz * uz
            + 9.7976147132876e-18 * ux * uz
            + 0.0555555555555555 * ux
            - 0.0833333333333333 * uy * uy * uz * uz
            - 3.91904588531504e-17 * uy * uy * uz
            + 0.0555555555555556 * uy * uy
            + 0.0833333333333333 * uy * uz * uz
            + 2.93928441398628e-17 * uy * uz
            - 0.0555555555555556 * uy
            - 0.0277777777777778 * uz * uz
            + 0.0185185185185185);

    result[9] = density
        * (-0.0833333333333333 * ux * ux * uy * uy
            - 0.0833333333333334 * ux * ux * uy
            - 0.0833333333333334 * ux * ux * uz * uz
            - 1.08739602338738e-17 * ux * ux * uz
            + 0.0555555555555556 * ux * ux
            + 0.0833333333333333 * ux * uy * uy
            + ux * uy
                * (-0.25 * ux * uy * uz * uz + 0.25 * ux * uy
                    - 0.25 * ux * uz * uz
                    + 8.69916818709907e-17 * ux * uz
                    + 0.25 * ux
                    + 0.25 * uy * uz * uz
                    + 8.69916818709907e-17 * uy * uz
                    - 0.25 * uy
                    + 0.25 * uz * uz
                    - 0.25)
            + 0.0833333333333334 * ux * uy
            + 0.0833333333333333 * ux * uz * uz
            + 1.08739602338738e-17 * ux * uz
            - 0.0555555555555556 * ux
            - 0.0833333333333333 * uy * uy * uz * uz
            - 3.26218807016215e-17 * uy * uy * uz
            + 0.0555555555555556 * uy * uy
            - 0.0833333333333334 * uy * uz * uz
            + 4.34958409354954e-17 * uy * uz
            + 0.0555555555555556 * uy
            - 0.0277777777777778 * uz * uz
            + 0.0185185185185185);

    result[10] = density
        * (-0.0833333333333333 * ux * ux * uy * uy
            + 0.0833333333333333 * ux * ux * uy
            - 0.0833333333333334 * ux * ux * uz * uz
            + 0.0555555555555555 * ux * ux
            + 0.0833333333333333 * ux * uy * uy
            + ux * uy
                * (-0.25 * ux * uy * uz * uz
                    + 0.25 * ux * uy
                    + 0.25 * ux * uz * uz
                    + 9.06010319844033e-17 * ux * uz
                    - 0.25 * ux
                    + 0.25 * uy * uz * uz
                    + 9.06010319844033e-17 * uy * uz
                    - 0.25 * uy
                    - 0.25 * uz * uz
                    + 0.25)
            - 0.0833333333333333 * ux * uy
            + 0.0833333333333333 * ux * uz * uz
            - 0.0555555555555555 * ux
            - 0.0833333333333333 * uy * uy * uz * uz
            - 3.39753869941512e-17 * uy * uy * uz
            + 0.0555555555555556 * uy * uy
            + 0.0833333333333333 * uy * uz * uz
            + 1.13251289980504e-17 * uy * uz
            - 0.0555555555555556 * uy
            - 0.0277777777777778 * uz * uz
            + 0.0185185185185185);

    result[11] = density
        * (-0.0833333333333333 * ux * ux * uy * uy
            - 2.11664223687214e-17 * ux * ux * uy
            - 0.0833333333333333 * ux * ux * uz * uz
            - 0.0833333333333334 * ux * ux * uz
            + 0.0555555555555556 * ux * ux
            - 0.0833333333333334 * ux * uy * uy
            + 2.11664223687214e-17 * ux * uy
            - 0.0833333333333334 * ux * uz * uz
            - ux * uz
                * (0.25 * ux * uy * uy * uz
                    + 0.25 * ux * uy * uy
                    + 4.23328447374428e-17 * ux * uy * uz
                    - 1.26998534212328e-16 * ux * uy
                    - 0.25 * ux * uz
                    - 0.25 * ux
                    + 0.25 * uy * uy * uz
                    + 0.25 * uy * uy
                    - 1.26998534212328e-16 * uy * uz
                    + 4.23328447374428e-17 * uy
                    - 0.25 * uz
                    - 0.25)
            - 0.0833333333333333 * ux * uz
            + 0.0555555555555556 * ux
            - 0.0833333333333333 * uy * uy * uz * uz
            - 0.0833333333333334 * uy * uy * uz
            - 0.0277777777777778 * uy * uy
            - 3.17496335530821e-17 * uy * uz * uz
            + 3.17496335530821e-17 * uy * uz
            - 4.70364941527142e-18 * uy
            + 0.0555555555555556 * uz * uz
            + 0.0555555555555556 * uz
            + 0.0185185185185185);

    result[12] = density
        * (-0.0833333333333333 * ux * ux * uy * uy
            - 0.0833333333333334 * ux * ux * uz * uz
            + 0.0833333333333333 * ux * ux * uz
            + 0.0555555555555555 * ux * ux
            - 0.0833333333333333 * ux * uy * uy
            - 0.0833333333333333 * ux * uz * uz
            + ux * uz
                * (-0.25 * ux * uy * uy * uz
                    + 0.25 * ux * uy * uy
                    + 1.05933719092698e-16 * ux * uy
                    + 0.25 * ux * uz
                    - 0.25 * ux
                    - 0.25 * uy * uy * uz
                    + 0.25 * uy * uy
                    + 5.29668595463488e-17 * uy * uz
                    + 0.25 * uz
                    - 0.25)
            + 0.0833333333333333 * ux * uz
            + 0.0555555555555555 * ux
            - 0.0833333333333333 * uy * uy * uz * uz
            + 0.0833333333333333 * uy * uy * uz
            - 0.0277777777777778 * uy * uy
            - 1.32417148865872e-17 * uy * uz * uz
            + 1.32417148865872e-17 * uy * uz
            + 0.0555555555555556 * uz * uz
            - 0.0555555555555556 * uz
            + 0.0185185185185185);

    result[13] = density
        * (-0.0833333333333333 * ux * ux * uy * uy
            - 1.37623559209966e-17 * ux * ux * uy
            - 0.0833333333333334 * ux * ux * uz * uz
            - 0.0833333333333333 * ux * ux * uz
            + 0.0555555555555556 * ux * ux
            + 0.0833333333333333 * ux * uy * uy
            + 1.37623559209966e-17 * ux * uy
            + 0.0833333333333333 * ux * uz * uz
            + ux * uz
                * (-0.25 * ux * uy * uy * uz - 0.25 * ux * uy * uy
                    + 1.10098847367973e-16 * ux * uy
                    + 0.25 * ux * uz
                    + 0.25 * ux
                    + 0.25 * uy * uy * uz
                    + 0.25 * uy * uy
                    + 1.10098847367973e-16 * uy * uz
                    - 0.25 * uz
                    - 0.25)
            + 0.0833333333333333 * ux * uz
            - 0.0555555555555556 * ux
            - 0.0833333333333333 * uy * uy * uz * uz
            - 0.0833333333333334 * uy * uy * uz
            - 0.0277777777777778 * uy * uy
            - 2.75247118419932e-17 * uy * uz * uz
            + 2.75247118419932e-17 * uy * uz
            + 0.0555555555555556 * uz * uz
            + 0.0555555555555556 * uz
            + 0.0185185185185185);

    result[14] = density
        * (-0.0833333333333333 * ux * ux * uy * uy
            + 1.36055001262589e-17 * ux * ux * uy
            - 0.0833333333333334 * ux * ux * uz * uz
            + 0.0833333333333333 * ux * ux * uz
            + 0.0555555555555555 * ux * ux
            + 0.0833333333333333 * ux * uy * uy
            - 1.36055001262589e-17 * ux * uy
            + 0.0833333333333333 * ux * uz * uz
            + ux * uz
                * (-0.25 * ux * uy * uy * uz
                    + 0.25 * ux * uy * uy
                    + 1.08844001010071e-16 * ux * uy
                    + 0.25 * ux * uz
                    - 0.25 * ux
                    + 0.25 * uy * uy * uz
                    - 0.25 * uy * uy
                    + 1.08844001010071e-16 * uy * uz
                    - 0.25 * uz
                    + 0.25)
            - 0.0833333333333333 * ux * uz
            - 0.0555555555555555 * ux
            - 0.0833333333333333 * uy * uy * uz * uz
            + 0.0833333333333333 * uy * uy * uz
            - 0.0277777777777778 * uy * uy
            + 0.0555555555555555 * uz * uz
            - 0.0555555555555555 * uz
            + 0.0185185185185185);

    result[15] = density
        * (-0.0833333333333334 * ux * ux * uy * uy
            - 0.0833333333333333 * ux * ux * uy
            - 0.0833333333333334 * ux * ux * uz * uz
            - 0.0833333333333333 * ux * ux * uz
            - 0.0277777777777778 * ux * ux
            - 9.53564509926323e-18 * ux * uy * uy
            + 4.23806448856144e-18 * ux
            - 0.0833333333333333 * uy * uy * uz * uz
            - 0.0833333333333334 * uy * uy * uz
            + 0.0555555555555556 * uy * uy
            - 0.0833333333333334 * uy * uz * uz
            + uy * uz
                * (-0.25 * ux * ux * uy * uz
                    - 0.25 * ux * ux * uy
                    - 0.25 * ux * ux * uz
                    - 0.25 * ux * ux
                    + 7.62851607941059e-17 * ux * uy * uz
                    + 3.81425803970529e-17 * ux * uy
                    + 3.81425803970529e-17 * ux * uz
                    + 7.62851607941059e-17 * ux
                    + 0.25 * uy * uz
                    + 0.25 * uy
                    + 0.25 * uz
                    + 0.25)
            - 0.0833333333333333 * uy * uz
            + 0.0555555555555556 * uy
            + 0.0555555555555556 * uz * uz
            + 0.0555555555555556 * uz
            + 0.0185185185185185);

    result[16] = density
        * (-0.0833333333333333 * ux * ux * uy * uy
            - 0.0833333333333333 * ux * ux * uy
            - 0.0833333333333333 * ux * ux * uz * uz
            + 0.0833333333333333 * ux * ux * uz
            - 0.0277777777777778 * ux * ux
            - 1.19310000967115e-17 * ux * uy * uy
            - 0.0833333333333333 * uy * uy * uz * uz
            + 0.0833333333333333 * uy * uy * uz
            + 0.0555555555555556 * uy * uy
            - 0.0833333333333333 * uy * uz * uz
            + uy * uz
                * (-0.25 * ux * ux * uy * uz + 0.25 * ux * ux * uy
                    - 0.25 * ux * ux * uz
                    + 0.25 * ux * ux
                    + 9.54480007736924e-17 * ux * uy
                    + 9.54480007736924e-17 * ux * uz
                    + 0.25 * uy * uz
                    - 0.25 * uy
                    + 0.25 * uz
                    - 0.25)
            + 0.0833333333333333 * uy * uz
            + 0.0555555555555556 * uy
            + 0.0555555555555556 * uz * uz
            - 0.0555555555555556 * uz
            + 0.0185185185185185);

    result[17] = density
        * (-0.0833333333333333 * ux * ux * uy * uy
            + 0.0833333333333333 * ux * ux * uy
            - 0.0833333333333334 * ux * ux * uz * uz
            - 0.0833333333333334 * ux * ux * uz
            - 0.0277777777777778 * ux * ux
            - 3.35180908066739e-17 * ux * uy * uy
            + 1.67590454033369e-17 * ux * uy
            - 1.67590454033369e-17 * ux * uz * uz
            + 1.67590454033369e-17 * ux * uz
            - 0.0833333333333333 * uy * uy * uz * uz
            - 0.0833333333333334 * uy * uy * uz
            + 0.0555555555555556 * uy * uy
            + 0.0833333333333333 * uy * uz * uz
            + uy * uz
                * (-0.25 * ux * ux * uy * uz - 0.25 * ux * ux * uy
                    + 0.25 * ux * ux * uz
                    + 0.25 * ux * ux
                    + 3.35180908066739e-17 * ux * uy * uz
                    + 1.00554272420022e-16 * ux * uy
                    + 1.00554272420022e-16 * ux * uz
                    + 0.25 * uy * uz
                    + 0.25 * uy
                    - 0.25 * uz
                    - 0.25)
            + 0.0833333333333334 * uy * uz
            - 0.0555555555555556 * uy
            + 0.0555555555555556 * uz * uz
            + 0.0555555555555556 * uz
            + 0.0185185185185185);

    result[18] = density
        * (-0.0833333333333333 * ux * ux * uy * uy
            + 0.0833333333333333 * ux * ux * uy
            - 0.0833333333333334 * ux * ux * uz * uz
            + 0.0833333333333333 * ux * ux * uz
            - 0.0277777777777778 * ux * ux
            - 2.20907133110069e-17 * ux * uy * uy
            + 1.10453566555034e-17 * ux * uy
            - 1.10453566555034e-17 * ux * uz * uz
            + 1.10453566555034e-17 * ux * uz
            - 0.0833333333333333 * uy * uy * uz * uz
            + 0.0833333333333333 * uy * uy * uz
            + 0.0555555555555556 * uy * uy
            + 0.0833333333333333 * uy * uz * uz
            + uy * uz
                * (-0.25 * ux * ux * uy * uz
                    + 0.25 * ux * ux * uy
                    + 0.25 * ux * ux * uz
                    - 0.25 * ux * ux
                    + 4.41814266220137e-17 * ux * uy * uz
                    + 8.83628532440274e-17 * ux * uy
                    + 8.83628532440274e-17 * ux * uz
                    + 0.25 * uy * uz
                    - 0.25 * uy
                    - 0.25 * uz
                    + 0.25)
            - 0.0833333333333333 * uy * uz
            - 0.0555555555555556 * uy
            + 0.0555555555555556 * uz * uz
            - 0.0555555555555556 * uz
            + 0.0185185185185185);

    result[19] = density
        * (0.0416666666666667 * ux * ux * uy * uy
            + 0.0416666666666667 * ux * ux * uy
            + 0.0416666666666667 * ux * ux * uz * uz
            + 0.0416666666666667 * ux * ux * uz
            + 0.0138888888888889 * ux * ux
            + 0.0416666666666667 * ux * uy * uy
            + ux * uy
                * uz
                * (0.125 * ux * uy * uz
                    + 0.125 * ux * uy
                    + 0.125 * ux * uz
                    + 0.125 * ux
                    + 0.125 * uy * uz
                    + 0.125 * uy
                    + 0.125 * uz
                    + 0.125)
            + 0.0416666666666667 * ux * uy
            + 0.0416666666666667 * ux * uz * uz
            + 0.0416666666666667 * ux * uz
            + 0.0138888888888889 * ux
            + 0.0416666666666666 * uy * uy * uz * uz
            + 0.0416666666666667 * uy * uy * uz
            + 0.0138888888888889 * uy * uy
            + 0.0416666666666667 * uy * uz * uz
            + 0.0416666666666667 * uy * uz
            + 0.0138888888888889 * uy
            + 0.0138888888888889 * uz * uz
            + 0.0138888888888889 * uz
            + 0.00462962962962963);

    result[20] = density
        * (0.0416666666666667 * ux * ux * uy * uy
            + 0.0416666666666667 * ux * ux * uy
            + 0.0416666666666667 * ux * ux * uz * uz
            - 0.0416666666666667 * ux * ux * uz
            + 0.0138888888888889 * ux * ux
            + 0.0416666666666667 * ux * uy * uy
            + ux * uy
                * uz
                * (0.125 * ux * uy * uz - 0.125 * ux * uy + 0.125 * ux * uz
                    - 0.125 * ux
                    + 0.125 * uy * uz
                    - 0.125 * uy
                    + 0.125 * uz
                    - 0.125)
            + 0.0416666666666667 * ux * uy
            + 0.0416666666666667 * ux * uz * uz
            - 0.0416666666666667 * ux * uz
            + 0.0138888888888889 * ux
            + 0.0416666666666666 * uy * uy * uz * uz
            - 0.0416666666666667 * uy * uy * uz
            + 0.0138888888888889 * uy * uy
            + 0.0416666666666667 * uy * uz * uz
            - 0.0416666666666667 * uy * uz
            + 0.0138888888888889 * uy
            + 0.0138888888888889 * uz * uz
            - 0.0138888888888889 * uz
            + 0.00462962962962963);

    result[21] = density
        * (0.0416666666666667 * ux * ux * uy * uy
            - 0.0416666666666667 * ux * ux * uy
            + 0.0416666666666667 * ux * ux * uz * uz
            + 0.0416666666666667 * ux * ux * uz
            + 0.0138888888888889 * ux * ux
            + 0.0416666666666667 * ux * uy * uy
            + ux * uy
                * uz
                * (0.125 * ux * uy * uz + 0.125 * ux * uy
                    - 0.125 * ux * uz
                    - 0.125 * ux
                    + 0.125 * uy * uz
                    + 0.125 * uy
                    - 0.125 * uz
                    - 0.125)
            - 0.0416666666666667 * ux * uy
            + 0.0416666666666667 * ux * uz * uz
            + 0.0416666666666667 * ux * uz
            + 0.0138888888888889 * ux
            + 0.0416666666666666 * uy * uy * uz * uz
            + 0.0416666666666667 * uy * uy * uz
            + 0.0138888888888889 * uy * uy
            - 0.0416666666666667 * uy * uz * uz
            - 0.0416666666666667 * uy * uz
            - 0.0138888888888889 * uy
            + 0.0138888888888889 * uz * uz
            + 0.0138888888888889 * uz
            + 0.00462962962962963);

    result[22] = density
        * (0.0416666666666667 * ux * ux * uy * uy
            + 0.0416666666666667 * ux * ux * uy
            + 0.0416666666666667 * ux * ux * uz * uz
            + 0.0416666666666667 * ux * ux * uz
            + 0.0138888888888889 * ux * ux
            - 0.0416666666666667 * ux * uy * uy
            + ux * uy
                * uz
                * (0.125 * ux * uy * uz
                    + 0.125 * ux * uy
                    + 0.125 * ux * uz
                    + 0.125 * ux
                    - 0.125 * uy * uz
                    - 0.125 * uy
                    - 0.125 * uz
                    - 0.125)
            - 0.0416666666666667 * ux * uy
            - 0.0416666666666667 * ux * uz * uz
            - 0.0416666666666667 * ux * uz
            - 0.0138888888888889 * ux
            + 0.0416666666666666 * uy * uy * uz * uz
            + 0.0416666666666667 * uy * uy * uz
            + 0.0138888888888889 * uy * uy
            + 0.0416666666666667 * uy * uz * uz
            + 0.0416666666666667 * uy * uz
            + 0.0138888888888889 * uy
            + 0.0138888888888889 * uz * uz
            + 0.0138888888888889 * uz
            + 0.00462962962962963);

    result[23] = density
        * (0.0416666666666667 * ux * ux * uy * uy
            - 0.0416666666666667 * ux * ux * uy
            + 0.0416666666666667 * ux * ux * uz * uz
            - 0.0416666666666667 * ux * ux * uz
            + 0.0138888888888889 * ux * ux
            + 0.0416666666666667 * ux * uy * uy
            + ux * uy
                * uz
                * (0.125 * ux * uy * uz - 0.125 * ux * uy - 0.125 * ux * uz
                    + 0.125 * ux
                    + 0.125 * uy * uz
                    - 0.125 * uy
                    - 0.125 * uz
                    + 0.125)
            - 0.0416666666666667 * ux * uy
            + 0.0416666666666667 * ux * uz * uz
            - 0.0416666666666667 * ux * uz
            + 0.0138888888888889 * ux
            + 0.0416666666666666 * uy * uy * uz * uz
            - 0.0416666666666667 * uy * uy * uz
            + 0.0138888888888889 * uy * uy
            - 0.0416666666666667 * uy * uz * uz
            + 0.0416666666666667 * uy * uz
            - 0.0138888888888889 * uy
            + 0.0138888888888889 * uz * uz
            - 0.0138888888888889 * uz
            + 0.00462962962962963);

    result[24] = density
        * (0.0416666666666667 * ux * ux * uy * uy
            - 0.0416666666666667 * ux * ux * uy
            + 0.0416666666666667 * ux * ux * uz * uz
            + 0.0416666666666667 * ux * ux * uz
            + 0.0138888888888889 * ux * ux
            - 0.0416666666666667 * ux * uy * uy
            + ux * uy
                * uz
                * (0.125 * ux * uy * uz + 0.125 * ux * uy
                    - 0.125 * ux * uz
                    - 0.125 * ux
                    - 0.125 * uy * uz
                    - 0.125 * uy
                    + 0.125 * uz
                    + 0.125)
            + 0.0416666666666667 * ux * uy
            - 0.0416666666666667 * ux * uz * uz
            - 0.0416666666666667 * ux * uz
            - 0.0138888888888889 * ux
            + 0.0416666666666666 * uy * uy * uz * uz
            + 0.0416666666666667 * uy * uy * uz
            + 0.0138888888888889 * uy * uy
            - 0.0416666666666667 * uy * uz * uz
            - 0.0416666666666667 * uy * uz
            - 0.0138888888888889 * uy
            + 0.0138888888888889 * uz * uz
            + 0.0138888888888889 * uz
            + 0.00462962962962963);

    result[25] = density
        * (0.0416666666666667 * ux * ux * uy * uy
            + 0.0416666666666667 * ux * ux * uy
            + 0.0416666666666667 * ux * ux * uz * uz
            - 0.0416666666666667 * ux * ux * uz
            + 0.0138888888888889 * ux * ux
            - 0.0416666666666667 * ux * uy * uy
            + ux * uy
                * uz
                * (0.125 * ux * uy * uz - 0.125 * ux * uy + 0.125 * ux * uz
                    - 0.125 * ux
                    - 0.125 * uy * uz
                    + 0.125 * uy
                    - 0.125 * uz
                    + 0.125)
            - 0.0416666666666667 * ux * uy
            - 0.0416666666666667 * ux * uz * uz
            + 0.0416666666666667 * ux * uz
            - 0.0138888888888889 * ux
            + 0.0416666666666666 * uy * uy * uz * uz
            - 0.0416666666666666 * uy * uy * uz
            + 0.0138888888888889 * uy * uy
            + 0.0416666666666667 * uy * uz * uz
            - 0.0416666666666667 * uy * uz
            + 0.0138888888888889 * uy
            + 0.0138888888888889 * uz * uz
            - 0.0138888888888889 * uz
            + 0.00462962962962963);

    result[26] = density
        * (0.0416666666666667 * ux * ux * uy * uy
            - 0.0416666666666667 * ux * ux * uy
            + 0.0416666666666667 * ux * ux * uz * uz
            - 0.0416666666666667 * ux * ux * uz
            + 0.0138888888888889 * ux * ux
            - 0.0416666666666667 * ux * uy * uy
            + ux * uy
                * uz
                * (0.125 * ux * uy * uz - 0.125 * ux * uy - 0.125 * ux * uz
                    + 0.125 * ux
                    - 0.125 * uy * uz
                    + 0.125 * uy
                    + 0.125 * uz
                    - 0.125)
            + 0.0416666666666667 * ux * uy
            - 0.0416666666666667 * ux * uz * uz
            + 0.0416666666666667 * ux * uz
            - 0.0138888888888889 * ux
            + 0.0416666666666666 * uy * uy * uz * uz
            - 0.0416666666666667 * uy * uy * uz
            + 0.0138888888888889 * uy * uy
            - 0.0416666666666667 * uy * uz * uz
            + 0.0416666666666667 * uy * uz
            - 0.0138888888888889 * uy
            + 0.0138888888888889 * uz * uz
            - 0.0138888888888889 * uz
            + 0.00462962962962963);

    result
}
