use lbm3::*;
use nalgebra::matrix;

#[tokio::main]
async fn main() {
    //SimpleLogger::new().init().unwrap();
    let output_dir = "example_output/compute_shader_diagram";
    let _ = std::fs::remove_dir_all(output_dir);
    std::fs::create_dir(output_dir).unwrap();
    let output_file = format!("{}/diagram.vtu", output_dir);
    let grid_dimensions = matrix![0, 5; 0, 5; 0, 5];

    let faces = Faces::new(&grid_dimensions);
    let mut grid = VTKGrid::new(&grid_dimensions);

    let n_points = box_buffer_size(&grid_dimensions);
    let mut group_flag = vec![0i32; n_points];

    // Slip_xz
    let slip_xz = 1;
    for coord in coord_iter(faces.top) {
        let index = coord_to_linear_in_box(&coord, &grid_dimensions);
        group_flag[index] = slip_xz;
    }
    for coord in coord_iter(faces.bottom) {
        let index = coord_to_linear_in_box(&coord, &grid_dimensions);
        group_flag[index] = slip_xz;
    }

    let slip_yz = 2;
    for coord in coord_iter(faces.left) {
        let index = coord_to_linear_in_box(&coord, &grid_dimensions);
        group_flag[index] = slip_yz;
    }
    for coord in coord_iter(faces.right) {
        let index = coord_to_linear_in_box(&coord, &grid_dimensions);
        group_flag[index] = slip_yz;
    }

    let dirichlet = 3;
    for coord in coord_iter(faces.front) {
        let index = coord_to_linear_in_box(&coord, &grid_dimensions);
        group_flag[index] = dirichlet;
    }
    for coord in coord_iter(faces.back) {
        let index = coord_to_linear_in_box(&coord, &grid_dimensions);
        group_flag[index] = dirichlet;
    }

    grid.add_flag("group".to_string(), group_flag);
    grid.write(&output_file);
}
