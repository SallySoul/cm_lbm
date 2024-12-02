use crate::*;
use nalgebra::matrix;

/// Mutually exclusive AABBs for each face
/// of the domain
pub struct Faces {
    pub top: AABB<3>,
    pub bottom: AABB<3>,
    pub left: AABB<3>,
    pub right: AABB<3>,
    pub front: AABB<3>,
    pub back: AABB<3>,
}

impl Faces {
    pub fn new(grid_dimensions: AABB<3>) -> Faces {
        let x_min = grid_dimensions[(0, 0)];
        let x_max = grid_dimensions[(0, 1)];
        let y_min = grid_dimensions[(1, 0)];
        let y_max = grid_dimensions[(1, 1)];
        let z_min = grid_dimensions[(2, 0)];
        let z_max = grid_dimensions[(2, 1)];

        let bottom = matrix![x_min, x_max; y_min, y_min; z_min, z_max;];
        let top = matrix![x_min, x_max; y_max, y_max; z_min, z_max;];
        let left = matrix![x_min, x_min; y_min + 1, y_max - 1; z_min, z_max;];
        let right = matrix![x_max, x_max; y_min + 1, y_max - 1; z_min, z_max;];
        let front = matrix![x_min + 1, x_max - 1; y_min + 1, y_max - 1;  z_min, z_min];
        let back = matrix![x_min + 1, x_max - 1; y_min + 1, y_max - 1; z_max, z_max];

        Faces {
            top,
            bottom,
            left,
            right,
            front,
            back,
        }
    }
}
