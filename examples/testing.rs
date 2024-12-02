use lbm_clean::*;
use nalgebra::{vector, matrix};

fn main() {
    let grid_dimensions = matrix![0, 80; 0, 80; 0, 80];
    //let omega = 1.85;
    let omega = 0.5;
    let inflow_density = 1.0;
    let inflow_accel = 0.015;
    let inflow_velocity = vector![0.0, 0.0, 0.2];
    let mut solver = Solver::new(grid_dimensions, omega, inflow_density, inflow_accel, inflow_velocity);
    solver.flow_init();
    run(&mut solver, 1000, 10);
}
