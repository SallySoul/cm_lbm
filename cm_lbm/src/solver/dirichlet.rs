use crate::*;
use nalgebra::matrix;

/// We hardcode the boundary conditions
/// so the two xy faces enforce velocity
/// Note also that this face is mutually exclusive
/// with the other faces
pub struct Dirichlet {
    face_uniform: GridDimensionsUniform,

    bc_uniform: BCParamsUniform,

    xy_pipeline: wgpu::ComputePipeline,

    work_groups: [u32; 3],
}

impl Dirichlet {
    pub fn new(
        device: &wgpu::Device,
        grid_dimensions: &AABB3,
        grid_dimensions_uniform: &GridDimensionsUniform,
        distributions: &Distributions,
        bc_uniform: BCParamsUniform,
    ) -> Self {
        println!("Creating Dirichlet");
        let x_min = grid_dimensions[(0, 0)];
        let x_max = grid_dimensions[(0, 1)];
        let y_min = grid_dimensions[(1, 0)];
        let y_max = grid_dimensions[(1, 1)];

        // xy face interior + z = 0
        let face_aabb = matrix![x_min, x_max; y_min, y_max;  0, 0];
        let face_uniform = GridDimensionsUniform::new(device, &face_aabb);

        // They don't use inclusive ranges or something?
        // add one fixes it
        let max = (face_aabb.column(1) - face_aabb.column(0)).add_scalar(1);
        let work_groups = [
            (max[0] / 4 + 1) as u32,
            (max[1] / 4 + 1) as u32,
            (max[2] / 4 + 1) as u32,
        ];

        let mut shader_builder = ShaderBuilder::new();
        shader_builder.add_dimensions_uniform(0);
        shader_builder.add_face_uniform(1);
        shader_builder.add_bc_params_uniform(2);
        shader_builder.add_distributions(3);
        shader_builder.add_lattice_constants();
        shader_builder.add_index_ops_periodic();
        shader_builder.add_equil_fn();
        shader_builder.add_dirichlet_main([4, 4, 1]);
        let shader_source =
            shader_builder.finish("shader_debug/dirichlet.wgsl");

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("dirichlet_pipeline_layout"),
                bind_group_layouts: &[
                    &grid_dimensions_uniform.layout,
                    &face_uniform.layout,
                    &bc_uniform.layout,
                    &distributions.layout,
                ],
                push_constant_ranges: &[],
            });

        let shader_module: wgpu::ShaderModule =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("dirichlet_shader_module"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                    &shader_source,
                )),
            });

        let xy_pipeline: wgpu::ComputePipeline = device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("dirichlet_pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader_module,
                entry_point: "main",
                compilation_options: wgpu::PipelineCompilationOptions::default(
                ),
                cache: None,
            });

        Dirichlet {
            face_uniform,
            bc_uniform,
            xy_pipeline,
            work_groups,
        }
    }

    pub fn apply(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        distributions: &Distributions,
        grid_dimensions_uniform: &GridDimensionsUniform,
    ) {
        let mut cpass =
            encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("dirichlet_apply_pass"),
                timestamp_writes: None,
            });
        cpass.set_pipeline(&self.xy_pipeline);
        cpass.set_bind_group(0, &grid_dimensions_uniform.bindgroup, &[]);
        cpass.set_bind_group(1, &self.face_uniform.bindgroup, &[]);
        cpass.set_bind_group(2, &self.bc_uniform.bindgroup, &[]);
        cpass.set_bind_group(3, &distributions.bindgroup, &[]);
        cpass.dispatch_workgroups(
            self.work_groups[0],
            self.work_groups[1],
            self.work_groups[2],
        );
    }
}
