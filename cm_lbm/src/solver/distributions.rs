use crate::*;

pub struct Distributions {
    pub layout: wgpu::BindGroupLayout,
    pub bindgroup: wgpu::BindGroup,
    pub bindgroup_scratch: wgpu::BindGroup,
}

impl Distributions {
    pub fn new(device: &wgpu::Device, grid_dimensions: &AABB3) -> Self {
        println!("Creating distributions");
        let buffer_size = box_buffer_size(&grid_dimensions);
        let distributions_buffer_bytes =
            (27 * buffer_size * std::mem::size_of::<f32>()) as u64;
        let buffer = create_storage_buffer(
            device,
            distributions_buffer_bytes,
            Some("distributions_buffer_a"),
        );

        let buffer_scratch = create_storage_buffer(
            device,
            distributions_buffer_bytes,
            Some("distributions_buffer_b"),
        );

        let layout: wgpu::BindGroupLayout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage {
                            read_only: false,
                        },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("distributions_layout"),
            });

        let bindgroup: wgpu::BindGroup =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("distributions_bindgroup_a"),
                layout: &layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
            });

        let bindgroup_scratch: wgpu::BindGroup =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("distributions_bindgroup_b"),
                layout: &layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer_scratch.as_entire_binding(),
                }],
            });

        Distributions {
            layout,
            bindgroup,
            bindgroup_scratch,
        }
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.bindgroup, &mut self.bindgroup_scratch);
    }
}
