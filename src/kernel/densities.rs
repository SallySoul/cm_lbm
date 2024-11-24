use crate::kernel::map_buffer::*;
use crate::kernel::setup_bind_group::*;
use crate::wgpu_util;

pub struct DensitiesData {
    pub densities: Vec<Vec<f32>>,
}

pub struct Densities<'a> {
    pub dimensions: &'a crate::LatticeDimensions,
    pub bindgroup_layout: wgpu::BindGroupLayout,
    pub input_buffers: Vec<wgpu::Buffer>,
    pub output_buffers: Vec<wgpu::Buffer>,
    pub input_bindgroups: Vec<wgpu::BindGroup>,
    pub output_bindgroups: Vec<wgpu::BindGroup>,
}

impl<'a> Densities<'a> {
    pub fn new(device: &wgpu::Device, dimensions: &'a crate::LatticeDimensions) -> Self {
        let buffer_byte_size: u64 = dimensions.float_buffer_byte_size();

        let q_usize = dimensions.q as usize;
        let mut input_buffers = Vec::with_capacity(q_usize);
        let mut output_buffers = Vec::with_capacity(q_usize);
        for i in 0..q_usize {
            let input_label = format!("density_{}_buffer_a", i);
            let output_label = format!("density_{}_buffer_b", i);
            let input_buffer = create_storage_buffer(device, buffer_byte_size, Some(&input_label));
            let output_buffer =
                create_storage_buffer(device, buffer_byte_size, Some(&output_label));
            input_buffers.push(input_buffer);
            output_buffers.push(output_buffer);
        }

        let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let mut input_bindgroups = Vec::with_capacity(q_usize);
        let mut output_bindgroups = Vec::with_capacity(q_usize);
        for i in 0..q_usize {
            let input_label = format!("density_{}_bindgroup_a", i);
            let output_label = format!("density_{}_bindgroup_b", i);
            let input_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(&input_label),
                layout: &bindgroup_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: input_buffers[i].as_entire_binding(),
                }],
            });
            let output_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(&output_label),
                layout: &bindgroup_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: output_buffers[i].as_entire_binding(),
                }],
            });
            input_bindgroups.push(input_bindgroup);
            output_bindgroups.push(output_bindgroup);
        }

        Densities {
            dimensions,
            input_buffers,
            output_buffers,
            bindgroup_layout,
            input_bindgroups,
            output_bindgroups,
        }
    }

    pub fn set_data(&mut self, driver: &wgpu_util::Driver, density_data: DensitiesData) {
        let data = std::sync::Arc::new(density_data);
        let write_buffer = WriteMapBuffer::new(&driver.device, self.dimensions);
        for i in 0..self.input_buffers.len() {
            let capturable = data.clone();
            write_buffer.write_data(driver, &self.input_buffers[i], move |slice| {
                slice.copy_from_slice(&capturable.densities[i]);
            });
        }
    }

    pub fn get_data(&self, driver: &wgpu_util::Driver) -> DensitiesData {
        let mut densities = Vec::with_capacity(self.dimensions.q as usize);

        let read_buffer = ReadMapBuffer::new(&driver.device, self.dimensions);
        for i in 0..self.input_buffers.len() {
            densities.push(read_buffer.clone_data(driver, &self.input_buffers[i]));
        }

        DensitiesData { densities }
    }
}
