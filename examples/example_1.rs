use lbm3::*;
use nalgebra::{matrix, vector};

#[tokio::main]
async fn main() {
    let output_dir = "example_output/example_1";
    let _ = std::fs::remove_dir_all(output_dir);
    std::fs::create_dir(output_dir).unwrap();

    println!("Start Example 1");
    let grid_dimensions = matrix![0, 100; 0, 100; 0, 100];
    let omega = 0.5;
    let ic_density = 1.0;
    let ic_velocity = vector![0.0, 0.0, 1.0];
    let bc_density = 1.0;
    let bc_velocity = vector![0.0, 0.0, 1.0];
    let n_it = 1000;
    let n_out = 10;

    let world_coords = WorldCoords::new(vector![-50.0, -50.0, -50.0], 1.0);
    let spheres = vec![
        (vector![20.0, -30.0, -10.0], 15.0),
        (vector![10.0, 5.0, -20.0], 15.0),
        (vector![-20.0, 30.0, 30.0], 15.0),
    ];

    let driver = setup_wgpu().await;

    let bounce_back = BounceBack::new_spheres(
        &driver.device,
        &spheres,
        &grid_dimensions,
        &world_coords,
        Some(&format!("{}/bounce_back.vtu", output_dir)),
    );

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
            solver.apply_collision(encoder);
            solver.moments(encoder);
        });

        solver.write_vtk(
            &driver,
            &format!("{}/moments_{:06}.vtu", output_dir, iter),
        );
    }
}
