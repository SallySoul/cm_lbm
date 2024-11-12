use crate::kernel::setup_bind_group::*;

pub struct Densities<'a> {
    pub dimensions: &'a crate::LatticeDimensions,
    pub input_buffer: wgpu::Buffer,
    pub output_buffer: wgpu::Buffer,
    pub layout: wgpu::BindGroupLayout,
    pub input_bind_group: wgpu::BindGroup,
    pub output_bind_group: wgpu::BindGroup,
}

impl<'a> Densities<'a> {
    pub fn new(device: &wgpu::Device, dimensions: &'a crate::LatticeDimensions) -> Self {
        let buffer_byte_size: u64 = dimensions.float_buffer_byte_size();

        let input_buffer =
            create_storage_buffer(device, buffer_byte_size, Some("densities_buffer_a"));
        let output_buffer =
            create_storage_buffer(device, buffer_byte_size, Some("densities_buffer_b"));

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("densities_layout"),
        });

        let input_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("densities_bindgroup_a"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: input_buffer.as_entire_binding(),
            }],
        });

        let output_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("densities_bindgroup_b"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: output_buffer.as_entire_binding(),
            }],
        });

        Densities {
            dimensions,
            input_buffer,
            output_buffer,
            layout,
            input_bind_group,
            output_bind_group,
        }
    }
}
