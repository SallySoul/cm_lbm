use crate::*;
use wgpu::util::DeviceExt;

/// We pass in the GridDimensions AABB to most shaders
/// as a uniform.
/// This tracks the bindgroup for that uniform.
pub struct GridDimensionsUniform {
    pub layout: wgpu::BindGroupLayout,
    pub bindgroup: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
}

impl GridDimensionsUniform {
    pub fn new(device: &wgpu::Device, grid_dimensions: &AABB3) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("grid_dimensions_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<AABB3>() as u64),
                },
                count: None,
            }],
        });

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("grid_dimensions_buffer"),
            contents: bytemuck::cast_slice(grid_dimensions.data.as_slice()),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("grid_dimensions_bindgroup"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        GridDimensionsUniform {
            layout,
            bindgroup,
            buffer,
        }
    }
}
