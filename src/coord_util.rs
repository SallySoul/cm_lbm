pub use num_traits::{Num, One, Zero};

pub type Vec3 = nalgebra::SVector<f32, 3>;

pub type Coord<const GRID_DIMENSION: usize> = nalgebra::SVector<i32, { GRID_DIMENSION }>;
pub type AABB<const GRID_DIMENSION: usize> = nalgebra::SMatrix<i32, { GRID_DIMENSION }, 2>;

pub type AABB3 = AABB<3>;
pub type Coord3 = Coord<3>;

pub fn coord_to_linear_in_box<const GRID_DIMENSION: usize>(
    coord: &Coord<GRID_DIMENSION>,
    b: &AABB<GRID_DIMENSION>,
) -> usize {
    #[cfg(debug_assertions)]
    {
        // TODO replace with `debug_assert!(b,contains ...)`
        for d in 0..GRID_DIMENSION {
            assert!(coord[d] >= b[(d, 0)]);
            assert!(coord[d] <= b[(d, 1)]);
        }
    }

    let bound = (b.column(1) - b.column(0)).add_scalar(1);
    let translated_coord = coord - b.column(0);
    let mut accumulator = 0;
    for d in 0..GRID_DIMENSION {
        let mut dim_accumulator = translated_coord[d] as usize;
        for dn in (d + 1)..GRID_DIMENSION {
            dim_accumulator *= bound[dn] as usize;
        }
        accumulator += dim_accumulator;
    }
    accumulator
}

pub fn linear_to_coord_in_box<const GRID_DIMENSION: usize>(
    index: usize,
    b: &AABB<GRID_DIMENSION>,
) -> Coord<GRID_DIMENSION> {
    let bound: Coord<GRID_DIMENSION> = (b.column(1) - b.column(0)).add_scalar(1);

    let mut result = Coord::zero();
    let mut index_accumulator = index;
    for d in 0..GRID_DIMENSION - 1 {
        let mut dim_accumulator = 1;
        for dn in (d + 1)..GRID_DIMENSION {
            dim_accumulator *= bound[dn] as usize;
        }

        result[d] = (index_accumulator / dim_accumulator) as i32;
        index_accumulator %= dim_accumulator;
    }
    result[GRID_DIMENSION - 1] = index_accumulator as i32;
    result + b.column(0)
}

pub fn box_buffer_size<const GRID_DIMENSION: usize>(view_box: &AABB<GRID_DIMENSION>) -> usize {
    let diff = (view_box.column(1) - view_box.column(0)).add_scalar(1);
    real_buffer_size(&diff)
}

pub fn real_buffer_size<const GRID_DIMENSION: usize>(space_size: &Coord<GRID_DIMENSION>) -> usize {
    let mut accumulator = 1;
    for d in space_size {
        accumulator *= *d as usize;
    }
    accumulator
}

pub fn box_contains_coord<const GRID_DIMENSION: usize>(aabb: &AABB<GRID_DIMENSION>, coord: &Coord<GRID_DIMENSION>) -> bool {
    for d in 0..GRID_DIMENSION {
        if coord[d] < aabb[(d, 0)] || coord[d] > aabb[(d, 1)] {
            return false;
        }
    }
    true
}

pub fn periodic_coord<const GRID_DIMENSION: usize>(
    index: &Coord<GRID_DIMENSION>,
    bound: &AABB<GRID_DIMENSION>,
) -> Coord<GRID_DIMENSION> {
    let mut result = Coord::zero();
    for d in 0..GRID_DIMENSION {
        let di_raw = index[d];
        result[d] = if di_raw < bound[(d, 0)] {
            bound[(d, 1)] + 1 + di_raw
        } else if di_raw > bound[(d, 1)] {
            bound[(d, 0)] + (di_raw - bound[(d, 1)] - 1)
        } else {
            di_raw
        }
    }
    //println!("periodic_coord, c: {:?}, r: {:?}", index, result);
    result
}

// Coord Iterator
pub fn coord_iter(aabb: AABB<3>) -> impl std::iter::Iterator<Item = Coord<3>> {
    let size = box_buffer_size(&aabb);
    (0..size).map(move |index| linear_to_coord_in_box(index, &aabb))
}

pub fn cell_coord_iter(aabb: AABB<3>) -> impl std::iter::Iterator<Item = Coord<3>> {
    let mut cell_bounds = aabb.clone();
    cell_bounds.set_column(1, &cell_bounds.column(1).add_scalar(-1));
    let size = box_buffer_size(&cell_bounds);
    (0..size).map(move |index| linear_to_coord_in_box(index, &cell_bounds))
}

pub fn cell_count(aabb: AABB<3>) -> usize {
    let mut cell_bounds = aabb.clone();
    cell_bounds.set_column(1, &cell_bounds.column(1).add_scalar(-1));
    box_buffer_size(&cell_bounds)
}
