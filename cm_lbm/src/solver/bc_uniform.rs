use crate::*;
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct BCParamsGPU {
    velocity: Vec3,
    density: f32,
}

pub struct BCParamsUniform {
    pub layout: wgpu::BindGroupLayout,
    pub bindgroup: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
}

impl BCParamsUniform {
    pub fn new(device: &wgpu::Device, velocity: Vec3, density: f32) -> Self {
        println!("Creating BC params uniform");
        let data = BCParamsGPU { velocity, density };

        let layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("bc_params_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            std::mem::size_of::<BCParamsGPU>() as u64,
                        ),
                    },
                    count: None,
                }],
            });

        let buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("bc_params_buffer"),
                contents: bytemuck::bytes_of(&data),
                usage: wgpu::BufferUsages::UNIFORM,
            });

        let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bc_params_bindgroup"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        BCParamsUniform {
            layout,
            bindgroup,
            buffer,
        }
    }
}
