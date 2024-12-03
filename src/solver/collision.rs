use crate::*;
use nalgebra::matrix;
use wgpu::util::DeviceExt;

pub struct Collision {
    /// Bumping into bindgroup limits
    /// so we combine grid and interiod bindgroups
    dimensions_bindgroup: wgpu::BindGroup,

    pipeline: wgpu::ComputePipeline,

    work_groups: [u32; 3],
}

impl Collision {
    pub fn new(
        device: &wgpu::Device,
        grid_dimensions: &AABB3,
        grid_dimensions_uniform: &GridDimensionsUniform,
        moments: &Moments,
        bounce_back: &BounceBack,
        distributions: &Distributions,
        omega: f32,
    ) -> Self {
        println!("Creating Collision");
        let x_min = grid_dimensions[(0, 0)];
        let x_max = grid_dimensions[(0, 1)];
        let y_min = grid_dimensions[(1, 0)];
        let y_max = grid_dimensions[(1, 1)];
        let z_min = grid_dimensions[(2, 0)];
        let z_max = grid_dimensions[(2, 1)];
        let interior_aabb = matrix![
            x_min + 1, x_max - 1; y_min + 1, y_max - 1; z_min + 1, z_max - 1];
        let interior_max =
            (interior_aabb.column(1) - interior_aabb.column(0)).add_scalar(1);
        let interior_data = GridDimensionsGPU {
            max: interior_max,
            total: box_buffer_size(&interior_aabb) as i32,
        };

        let grid_max = (grid_dimensions.column(1) - grid_dimensions.column(0))
            .add_scalar(1);
        let grid_data = GridDimensionsGPU {
            max: grid_max,
            total: box_buffer_size(grid_dimensions) as i32,
        };

        let work_groups = [
            (interior_max[0] / 4 + 1) as u32,
            (interior_max[1] / 4 + 1) as u32,
            (interior_max[2] / 4 + 1) as u32,
        ];

        let dimensions_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("collision_dimensions_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
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
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(
                                std::mem::size_of::<GridDimensionsGPU>() as u64,
                            ),
                        },
                        count: None,
                    },
                ],
            });

        let interior_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("interior_dimensions_buffer"),
                contents: bytemuck::bytes_of(&interior_data),
                usage: wgpu::BufferUsages::UNIFORM,
            });

        let grid_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("interior_dimensions_buffer"),
                contents: bytemuck::bytes_of(&grid_data),
                usage: wgpu::BufferUsages::UNIFORM,
            });

        let dimensions_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("collision_dimensions_bindgroup"),
            layout: &dimensions_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: interior_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: grid_buffer.as_entire_binding(),
            }],
        });

        let mut shader_builder = ShaderBuilder::new();
        shader_builder.add_collision_uniform(0);
        shader_builder.add_moments_bindgroup(1);
        shader_builder.add_bounceback_bindgroup(2);
        shader_builder.add_distributions(3);
        shader_builder.add_lattice_constants();
        shader_builder.add_index_ops_periodic();
        shader_builder.add_equil_fn();
        shader_builder.add_specular_reflection();
        shader_builder.add_collision_main([4, 4, 4], omega);
        let shader_source =
            shader_builder.finish("shader_debug/collision.wgsl");

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("collision_pipeline_layout"),
                bind_group_layouts: &[
                    &dimensions_layout,
                    &moments.layout,
                    &bounce_back.layout,
                    &distributions.layout,
                ],
                push_constant_ranges: &[],
            });

        let shader_module: wgpu::ShaderModule =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("collision_shader_module"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                    &shader_source,
                )),
            });

        let pipeline: wgpu::ComputePipeline =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("collision_pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader_module,
                entry_point: "main",
                compilation_options: wgpu::PipelineCompilationOptions::default(
                ),
                cache: None,
            });

        Collision {
            dimensions_bindgroup,
            pipeline,
            work_groups,
        }
    }

    pub fn apply(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        distributions: &Distributions,
        moments: &Moments,
        bounce_back: &BounceBack,
    ) {
        let mut cpass =
            encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("collision_apply_pass"),
                timestamp_writes: None,
            });
        cpass.set_pipeline(&self.pipeline);
        cpass.set_bind_group(0, &self.dimensions_bindgroup, &[]);
        cpass.set_bind_group(1, &moments.bindgroup, &[]);
        cpass.set_bind_group(2, &bounce_back.bindgroup, &[]);
        cpass.set_bind_group(3, &distributions.bindgroup, &[]);
        cpass.dispatch_workgroups(
            self.work_groups[0],
            self.work_groups[1],
            self.work_groups[2],
        );
    }
}
