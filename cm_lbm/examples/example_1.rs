use cm_lbm::*;
use nalgebra::{matrix, vector};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    output_dir: std::path::PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let output_dir = args.output_dir.to_str().unwrap();
    let _ = std::fs::remove_dir_all(output_dir);
    std::fs::create_dir(output_dir).unwrap();

    println!("Example 1");
    let velocity = 0.12;
    let density = 1.0;
    let riv = 1.98;
    let operator = CollisionType::CMMRT(riv);
    let grid_dimensions = matrix![0, 79; 0, 79; 0, 190];
    let ic_density = density;
    let ic_velocity = vector![0.0, 0.0, velocity];
    let bc_density = density;
    let bc_velocity = vector![0.0, 0.0, velocity];
    let n_it = 20000;
    let n_out = 10;

    let world_coords = WorldCoords::new(vector![-39.0, -39.0, -37.0], 1.0);
    let spheres = vec![(vector![0.0, 0.0, 0.0], 16.0)];

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

    let params = SolverParams {
        bounce_back,
        ic_params_uniform: ic_params,
        bc_params_uniform: bc_params,
        grid_dimensions,
        collision_type: operator,
        stream_figure: false,
    };

    let mut solver = Solver::new(&driver, params);
    solver.run(&driver, output_dir, n_it, n_out, None);
}
