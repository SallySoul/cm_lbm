use crate::*;
use bytemuck::{Pod, Zeroable};
use std::sync::Arc;
use wgpu::util::DeviceExt;

pub struct Moments {
    pub density_buffer: wgpu::Buffer,
    pub velocity_buffer: wgpu::Buffer,
    pub layout: wgpu::BindGroupLayout,
    pub bindgroup: wgpu::BindGroup,
    pub pipeline: wgpu::ComputePipeline,
    density_read_map: ReadMapBuffer,
    velocity_read_map: ReadMapBuffer,
}

impl Moments {
    pub fn new(
        device: &wgpu::Device,
        grid_dimensions: &AABB3,
        distributions: &Distributions,
        grid_dimensions_uniform: &GridDimensionsUniform,
    ) -> Self {
        println!("Creating Moments");
        let density_read_map = ReadMapBuffer::new(device, &grid_dimensions, 1);
        let velocity_read_map = ReadMapBuffer::new(device, &grid_dimensions, 3);
        let buffer_size = box_buffer_size(grid_dimensions);

        let density_buffer_bytes =
            (buffer_size * std::mem::size_of::<f32>()) as u64;
        let density_buffer = create_storage_buffer(
            device,
            density_buffer_bytes,
            Some("density_buffer"),
        );
        let velocity_buffer_bytes =
            (3 * buffer_size * std::mem::size_of::<f32>()) as u64;
        let velocity_buffer = create_storage_buffer(
            device,
            velocity_buffer_bytes,
            Some("velocity_buffer"),
        );

        let layout: wgpu::BindGroupLayout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
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
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage {
                                read_only: false,
                            },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
                label: Some("moments_layout"),
            });

        let bindgroup: wgpu::BindGroup =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("moments_bindgroup"),
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: density_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: velocity_buffer.as_entire_binding(),
                    },
                ],
            });

        let mut shader_builder = ShaderBuilder::new();
        shader_builder.add_dimensions_uniform(0);
        shader_builder.add_distributions(1);
        shader_builder.add_moments_bindgroup(2);
        shader_builder.add_lattice_constants();
        shader_builder.add_index_ops_periodic();
        shader_builder.add_moments_main([4, 4, 4]);
        let shader_source = shader_builder.finish("shader_debug/moments.wgsl");

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("moments_pipeline_layout"),
                bind_group_layouts: &[
                    &grid_dimensions_uniform.layout,
                    &distributions.layout,
                    &layout,
                ],
                push_constant_ranges: &[],
            });

        let shader_module: wgpu::ShaderModule =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("moments_shader_module"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                    &shader_source,
                )),
            });

        let pipeline: wgpu::ComputePipeline =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("moments_pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader_module,
                entry_point: "main",
                compilation_options: wgpu::PipelineCompilationOptions::default(
                ),
                cache: None,
            });

        Moments {
            density_buffer,
            velocity_buffer,
            layout,
            bindgroup,
            density_read_map,
            velocity_read_map,
            pipeline,
        }
    }

    pub fn compute(
        &self,
        work_groups: &[u32; 3],
        encoder: &mut wgpu::CommandEncoder,
        distributions: &Distributions,
        grid_dimensions_uniform: &GridDimensionsUniform,
    ) {
        let mut cpass =
            encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("ic_pass"),
                timestamp_writes: None,
            });

        cpass.set_pipeline(&self.pipeline);
        cpass.set_bind_group(0, &grid_dimensions_uniform.bindgroup, &[]);
        cpass.set_bind_group(1, &distributions.bindgroup, &[]);
        cpass.set_bind_group(2, &self.bindgroup, &[]);
        cpass.dispatch_workgroups(
            work_groups[0],
            work_groups[1],
            work_groups[2],
        );
    }

    pub fn get_data(&self, driver: &Driver) -> (Vec<f32>, Vec<f32>) {
        let densities = self
            .density_read_map
            .clone_data(driver, &self.density_buffer);
        let velocities = self
            .velocity_read_map
            .clone_data(driver, &self.velocity_buffer);
        (densities, velocities)
    }
}
