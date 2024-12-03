use lbm3::*;
use nalgebra::{matrix, vector};

#[tokio::main]
async fn main() {
    let output_dir = "example_output/specular";
    let _ = std::fs::remove_dir_all(output_dir);
    std::fs::create_dir(output_dir).unwrap();

    println!("Start Specular Figure");
    let grid_dimensions = matrix![0, 100; 0, 100; 0, 100];

    let world_coords = WorldCoords::new(vector![-50.0, -50.0, -50.0], 1.0);
    let spheres = vec![
        (vector![20.0, -30.0, -10.0], 15.0),
        (vector![10.0, 5.0, -20.0], 15.0),
        (vector![-20.0, 30.0, 30.0], 15.0),
    ];

    let driver = setup_wgpu().await;

    let _bounce_back = BounceBack::new_spheres(
        &driver.device,
        &spheres,
        &grid_dimensions,
        &world_coords,
        Some(&format!("{}/specular.vtu", output_dir)),
    );
}
