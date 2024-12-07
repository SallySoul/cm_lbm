use cm_lbm::*;
use nalgebra::{matrix, vector};

#[tokio::main]
async fn main() {
    let output_dir = "example_output/param_search";
    let _ = std::fs::remove_dir_all(output_dir);
    std::fs::create_dir(output_dir).unwrap();

    println!("Start Param Search");
    let velocity = 0.2;
    let density = 1.0;
    let riv = 1.95;

    let grid_dimensions = matrix![0, 60; 0, 60; 0, 100];
    let ic_density = density;
    let ic_velocity = vector![0.0, 0.0, velocity];
    let bc_density = density;
    let bc_velocity = vector![0.0, 0.0, velocity];
    let n_it = 100000;
    let n_out = 500;

    let driver = setup_wgpu().await;
    //let bounce_back = BounceBack::empty(&driver.device, &grid_dimensions);

    let world_coords = WorldCoords::new(vector![-40.0, -40.0, -40.0], 1.0);
    let spheres = vec![
        (vector![12.0, 12.0, 5.0], 10.0),
        (vector![-12.0, -12.0, -10.0], 10.0),
    ];

    let bounce_back = BounceBack::new_spheres(
        &driver.device,
        &spheres,
        &grid_dimensions,
        &world_coords,
        Some(&format!("{}/bounce_back.vtu", output_dir)),
    );

    /*
    let world_coords = WorldCoords::new(vector![-40.0, -40.0, -40.0], 1.0);
    let spheres = vec![
        (vector![12.0, 12.0, 5.0], 10.0),
        (vector![-12.0, -12.0, -10.0], 10.0),
    ];

    let bounce_back = BounceBack::new_spheres(
        &driver.device,
        &spheres,
        &grid_dimensions,
        &world_coords,
        Some(&format!("{}/bounce_back.vtu", output_dir)),
    );
    */

    let ic_params =
        BCParamsUniform::new(&driver.device, ic_velocity, bc_density);

    let bc_params =
        BCParamsUniform::new(&driver.device, bc_velocity, ic_density);

    let params = SolverParams {
        bounce_back,
        ic_params_uniform: ic_params,
        bc_params_uniform: bc_params,
        grid_dimensions,
        collision_type: CollisionType::CMMRT(riv),
        stream_figure: false,
    };

    let mut solver = Solver::new(&driver, params);

    solver.write_vtk(&driver, &format!("{}/moments_{:09}.vtu", output_dir, 0));
    for iter in 1..n_it {
        println!("iter: {}", iter);
        solver.apply_stream(&driver);

        run_submission(&driver, |encoder| {
            solver.moments(encoder);
            solver.apply_dirichlet(encoder);
            //solver.apply_slip_surfaces(encoder);
            solver.apply_collision(encoder);
            solver.apply_dirichlet(encoder);
        });

        let write_output = (iter + 1) % n_out == 0;
        if write_output {
            run_submission(&driver, |encoder| solver.moments(encoder));
            solver.write_vtk(
                &driver,
                &format!("{}/moments_{:09}.vtu", output_dir, iter),
            );
        }
    }
}
