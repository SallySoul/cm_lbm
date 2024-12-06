use lbm3::*;
use nalgebra::{matrix, vector};

#[tokio::main]
async fn main() {
    let output_dir = "example_output/stream_animation";
    let _ = std::fs::remove_dir_all(output_dir);
    std::fs::create_dir(output_dir).unwrap();

    println!("Start Stream Figure");
    let grid_dimensions = matrix![0, 4; 0, 4; 0, 4];
    let omega = 0.5;
    let ic_density = 1.0;
    let ic_velocity = vector![0.0, 0.0, 0.0];
    let bc_density = 1.0;
    let bc_velocity = vector![0.0, 0.0, 0.0];
    let n_it = 3;

    let driver = setup_wgpu().await;

    let bounce_back = BounceBack::empty(&driver.device, &grid_dimensions);

    let ic_params =
        BCParamsUniform::new(&driver.device, ic_velocity, bc_density);

    let bc_params =
        BCParamsUniform::new(&driver.device, bc_velocity, ic_density);

    let mut solver = Solver::new(
        &driver,
        bounce_back,
        ic_params,
        bc_params,
        grid_dimensions,
        omega,
        true,
    );

    solver.write_vtk(&driver, &format!("{}/moments_{:06}.vtu", output_dir, 0));
    for iter in 1..n_it {
        solver.apply_stream(&driver);

        run_submission(&driver, |encoder| {
            solver.moments(encoder);
        });

        solver.write_vtk(
            &driver,
            &format!("{}/moments_{:06}.vtu", output_dir, iter),
        );
    }
}
