use bytemuck::{Pod, Zeroable};

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct LatticeDimensions {
    pub rows: i32,
    pub cols: i32,
    pub total: i32,
    pub q: i32,
}
