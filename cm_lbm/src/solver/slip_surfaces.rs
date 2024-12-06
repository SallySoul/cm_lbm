use crate::*;
use nalgebra::matrix;

/// We hardcode the boundary conditions
/// the xz faces specular reflect with norms
/// [0, +/- 1, 0]
/// The yz faces specular reflect with norms
/// [+/- 1, 0, 0]
/// Note also that these faces are mutually exclusive
/// with the other faces
pub struct SlipSurfaces {
    xz_face_uniform: GridDimensionsUniform,

    yz_face_uniform: GridDimensionsUniform,

    xz_pipeline: wgpu::ComputePipeline,

    yz_pipeline: wgpu::ComputePipeline,

    xz_work_groups: [u32; 3],

    yz_work_groups: [u32; 3],
}

impl SlipSurfaces {
    pub fn new(
        device: &wgpu::Device,
        grid_dimensions: &AABB3,
        grid_dimensions_uniform: &GridDimensionsUniform,
        moments: &Moments,
        distributions: &Distributions,
    ) -> Self {
        println!("Creating Slip Surfaces");
        let x_min = grid_dimensions[(0, 0)];
        let x_max = grid_dimensions[(0, 1)];
        let y_min = grid_dimensions[(1, 0)];
        let y_max = grid_dimensions[(1, 1)];
        let z_min = grid_dimensions[(2, 0)];
        let z_max = grid_dimensions[(2, 1)];

        let xz_face_aabb = matrix![x_min, x_max; 0, 0;z_min , z_max;];
        let yz_face_aabb = matrix![0, 0; y_min + 1, y_max - 1; z_min, z_max;];

        let xz_face_uniform = GridDimensionsUniform::new(device, &xz_face_aabb);
        let yz_face_uniform = GridDimensionsUniform::new(device, &yz_face_aabb);

        // They don't use inclusive ranges or something?
        // add one fixes it
        let xz_max =
            (xz_face_aabb.column(1) - xz_face_aabb.column(0)).add_scalar(1);
        let xz_work_groups = [
            (xz_max[0] / 4 + 1) as u32,
            (xz_max[1] / 4 + 1) as u32,
            (xz_max[2] / 4 + 1) as u32,
        ];

        let yz_max =
            (yz_face_aabb.column(1) - yz_face_aabb.column(0)).add_scalar(1);
        let yz_work_groups = [
            (yz_max[0] / 4 + 1) as u32,
            (yz_max[1] / 4 + 1) as u32,
            (yz_max[2] / 4 + 1) as u32,
        ];

        let mut shader_builder = ShaderBuilder::new();
        shader_builder.add_dimensions_uniform(0);
        shader_builder.add_face_uniform(1);
        shader_builder.add_moments_bindgroup(2);
        shader_builder.add_distributions(3);
        shader_builder.add_lattice_constants();
        shader_builder.add_index_ops_periodic();
        shader_builder.add_equil_fn();
        shader_builder.add_specular_reflection();
        shader_builder.add_bounceback_fn();

        let mut xz_shader_builder = shader_builder.clone();
        xz_shader_builder.add_xz_bounce_main([4, 1, 4]);
        let xz_shader_source =
            xz_shader_builder.finish("shader_debug/xz_slip.wgsl");

        let mut yz_shader_builder = shader_builder.clone();
        yz_shader_builder.add_yz_bounce_main([1, 4, 4]);
        let yz_shader_source =
            yz_shader_builder.finish("shader_debug/yz_slip.wgsl");

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("slip_pipeline_layout"),
                bind_group_layouts: &[
                    &grid_dimensions_uniform.layout,
                    &xz_face_uniform.layout,
                    &moments.layout,
                    &distributions.layout,
                ],
                push_constant_ranges: &[],
            });

        let xz_shader_module: wgpu::ShaderModule =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("xz_slip_shader_module"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                    &xz_shader_source,
                )),
            });

        let xz_pipeline: wgpu::ComputePipeline = device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("xz_shader_pipeline"),
                layout: Some(&pipeline_layout),
                module: &xz_shader_module,
                entry_point: "main",
                compilation_options: wgpu::PipelineCompilationOptions::default(
                ),
                cache: None,
            });

        let yz_shader_module: wgpu::ShaderModule =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("yz_slip_shader_module"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                    &yz_shader_source,
                )),
            });

        let yz_pipeline: wgpu::ComputePipeline = device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("yz_shader_pipeline"),
                layout: Some(&pipeline_layout),
                module: &yz_shader_module,
                entry_point: "main",
                compilation_options: wgpu::PipelineCompilationOptions::default(
                ),
                cache: None,
            });

        SlipSurfaces {
            xz_face_uniform,
            yz_face_uniform,
            xz_pipeline,
            yz_pipeline,
            xz_work_groups,
            yz_work_groups,
        }
    }

    pub fn apply(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        moments: &Moments,
        distributions: &Distributions,
        grid_dimensions_uniform: &GridDimensionsUniform,
    ) {
        let mut cpass =
            encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("dirichlet_apply_pass"),
                timestamp_writes: None,
            });
        cpass.set_pipeline(&self.xz_pipeline);
        cpass.set_bind_group(0, &grid_dimensions_uniform.bindgroup, &[]);
        cpass.set_bind_group(1, &self.xz_face_uniform.bindgroup, &[]);
        cpass.set_bind_group(2, &moments.bindgroup, &[]);
        cpass.set_bind_group(3, &distributions.bindgroup, &[]);
        cpass.dispatch_workgroups(
            self.xz_work_groups[0],
            self.xz_work_groups[1],
            self.xz_work_groups[2],
        );

        cpass.set_pipeline(&self.yz_pipeline);
        cpass.set_bind_group(0, &grid_dimensions_uniform.bindgroup, &[]);
        cpass.set_bind_group(1, &self.yz_face_uniform.bindgroup, &[]);
        cpass.set_bind_group(2, &moments.bindgroup, &[]);
        cpass.set_bind_group(3, &distributions.bindgroup, &[]);
        cpass.dispatch_workgroups(
            self.yz_work_groups[0],
            self.yz_work_groups[1],
            self.yz_work_groups[2],
        )
    }
}
