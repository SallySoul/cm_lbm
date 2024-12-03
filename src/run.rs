use crate::*;

pub fn run(driver: &Driver, solver: &mut Solver, n_it: usize, n_out: usize) {
    println!("Starting Run");
    let mut iter = 0;

    println!("  writing first snapshot {:06}", iter);
    //solver.moments();
    solver.write_vtk(driver, &format!("vtk_test/moments_{:06}.vtu", iter));
    while iter < n_it {
        iter += 1;

        println!("  iter: {}", iter);
        let write_output = n_out > 0 && (iter + 1) % n_out == 0;
        println!("    streaming...");
        //solver.streaming();
        println!("    collision...");
        //solver.collision();

        if write_output {
            println!("    writing snapshot {:06}", iter);
            //solver.write_vtk(iter);
        }
    }
}
