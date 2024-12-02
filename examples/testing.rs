use lbm3::*;
use nalgebra::{matrix, vector};

#[tokio::main]
async fn main() {
    let grid_dimensions = matrix![0, 100; 0, 100; 0, 100];
    //let omega = 1.85;
    let omega = 0.5;
    let inflow_density = 1.0;
    let inflow_accel = 0.015;
    let inflow_velocity = vector![0.0, 0.0, 0.2];

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
        Some("vtk_test/bounce_back.vtu"),
    );

    let _solver = Solver::new(
        &driver.device,
        bounce_back,
        grid_dimensions,
        omega,
        inflow_density,
        inflow_velocity,
    );
}
