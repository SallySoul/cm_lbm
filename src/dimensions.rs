use bytemuck::{Pod, Zeroable};

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct LatticeDimensions {
    pub rows: i32,
    pub cols: i32,
    pub total: i32,
    pub q: i32,
    pub size: f32,
}

impl LatticeDimensions {
    pub fn float_buffer_byte_size(&self) -> u64 {
        self.total as u64 * std::mem::size_of::<f32>() as u64
    }
}
