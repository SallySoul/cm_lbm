use wgpu::util::DeviceExt;

pub struct LatticeDimensionsUniform {
    pub dimensions: crate::LatticeDimensions,
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
}

impl LatticeDimensionsUniform {
    pub fn new(device: &wgpu::Device, dimensions: crate::LatticeDimensions) -> Self {
        let layout_label = "dimension_layout";
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(layout_label),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<
                        crate::LatticeDimensions,
                    >() as u64),
                },
                count: None,
            }],
        });

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(&dimensions),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let bind_group_label = "dimensions_bg";
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(bind_group_label),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        LatticeDimensionsUniform {
            dimensions,
            layout,
            bind_group,
            buffer,
        }
    }
}
