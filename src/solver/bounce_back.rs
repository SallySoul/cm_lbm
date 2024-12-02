use crate::*;
use wgpu::util::DeviceExt;
use nalgebra::matrix;

/// Bindgroup for bounceback cells
/// Every cell has a flag,
/// if positive, its a bounceback cell, and the flag is the index
/// into the normal buffer
/// Otherwise, normal collision
pub struct BounceBack {
    // bounce_back
    // Number of bounce back cells
    pub normal_buffer: wgpu::Buffer,

    // W * H * L (i32)
    // -1 for normal
    // 0+ for indicies into bounce_back_normals
    pub flag_buffer: wgpu::Buffer,

    // normals bindings 0
    // flag bindings 1
    pub layout: wgpu::BindGroupLayout,
    pub bindgroup: wgpu::BindGroup,
}

impl BounceBack {
    fn new_sphere(device: &wgpu::Device, grid_dimensions: &AABB3, center: Coord3, radius: i32) -> Self {
        println!("Creating Bounce Back Region");
        let flag_buffer_size = box_buffer_size(grid_dimensions);
        let mut flags = vec![-1; flag_buffer_size];
        let mut normals = Vec::new();
        let min: Coord3 = center.add_scalar(-radius);
        let max: Coord3 = center.add_scalar(radius);
        let sphere_bounds: AABB3 = AABB3::from_columns(&[min, max]);
        for coord in coord_iter(sphere_bounds) {
            let flag = coord_to_linear_in_box(&coord, grid_dimensions)
        }
    
        Self::new(device, &flags, &normals)
    }

    fn new(device: &wgpu::Device, flags: &[i32], normals: &[Vec3]) -> Self {
        let normal_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("bounce_back_normal_buffer"),
            contents: bytemuck::cast_slice(&normals),
            usage: wgpu::BufferUsages::STORAGE,
        });

        let flag_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("bounce_back_flag_buffer"),
            contents: bytemuck::cast_slice(&flags),
            usage: wgpu::BufferUsages::STORAGE,
        });

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bounce_back_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bounce_back_bindgroup"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: normal_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: flag_buffer.as_entire_binding(),
            }],
        });

        BounceBack {
            normal_buffer,
            flag_buffer,
            layout,
            bindgroup,
        }
    }
}
