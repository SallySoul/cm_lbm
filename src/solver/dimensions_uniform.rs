use crate::*;
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct GridDimensionsGPU {
    pub max: Coord<3>,
    pub total: i32,
}

/// We pass in the GridDimensions AABB to most shaders
/// as a uniform.
/// This tracks the bindgroup for that uniform.
pub struct GridDimensionsUniform {
    pub layout: wgpu::BindGroupLayout,
    pub bindgroup: wgpu::BindGroup,
}

impl GridDimensionsUniform {
    pub fn new(device: &wgpu::Device, grid_dimensions: &AABB3) -> Self {
        println!("Creating grid dimensions uniform");

        let max = (grid_dimensions.column(1) - grid_dimensions.column(0))
            .add_scalar(1);
        let data = GridDimensionsGPU {
            max,
            total: box_buffer_size(grid_dimensions) as i32,
        };

        let layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("grid_dimensions_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            std::mem::size_of::<GridDimensionsGPU>() as u64,
                        ),
                    },
                    count: None,
                }],
            });

        let buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("grid_dimensions_buffer"),
                contents: bytemuck::bytes_of(&data),
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
        }
    }
}
