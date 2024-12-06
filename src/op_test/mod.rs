mod cm_mrt;
mod eq_op;
mod moments_op;

pub use cm_mrt::*;
pub use eq_op::*;
pub use moments_op::*;

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::*;
    use nalgebra::vector;

    #[test]
    fn eq_comp() {
        let directions = gen_d3q27_directions();
        {
            let u = vector!(0.0, 0.0, 0.0);
            let d = 1.0;

            let old = f_equilibrium(&directions, d, u);
            println!("old: {:?}", old);
            let o_o_moments = moments(&directions, &old);
            println!("o_o_moments: {:?}", o_o_moments);
            let n_o_moments = moments_op(old);
            println!("n_o_moments: {:?}", n_o_moments);

            let new = eq_op(u[0], u[1], u[2], d);
            println!("new: {:?}", new);
            let o_n_moments = moments(&directions, &new);
            println!("o_n_moments: {:?}", o_o_moments);
            let n_n_moments = moments_op(new);
            println!("n_n_moments: {:?}", n_o_moments);
        }

        {
            let u = vector!(0.1, -0.4, 0.3);
            let d = 0.4;

            let old = f_equilibrium(&directions, d, u);
            println!("old: {:?}", old);
            let o_o_moments = moments(&directions, &old);
            println!("o_o_moments: {:?}", o_o_moments);
            let n_o_moments = moments_op(old);
            println!("n_o_moments: {:?}", n_o_moments);

            let new = eq_op(u[0], u[1], u[2], d);
            println!("new: {:?}", new);
            let o_n_moments = moments(&directions, &new);
            println!("o_n_moments: {:?}", o_o_moments);
            let n_n_moments = moments_op(new);
            println!("n_n_moments: {:?}", n_o_moments);
        }
    }

    #[test]
    fn coll_op() {

    }
}
