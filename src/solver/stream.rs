use crate::*;

/// Streaming operator
///
/// This one is pretty simple really,
/// we only need a pipeline that operates
/// reads the distributions and writes to
/// distributions_scratch
pub struct Stream {
    pipeline: wgpu::ComputePipeline,

    work_groups: [u32; 3],
}

impl Stream {
    pub fn new(
        device: &wgpu::Device,
        grid_dimensions: &AABB3,
        grid_dimensions_uniform: &GridDimensionsUniform,
        distributions: &Distributions,
    ) -> Self {
        println!("Creating Stream");

        // They don't use inclusive ranges or something?
        // add one fixes it, or I don't need it?
        // TODO: see if we can remove it?
        let max = (grid_dimensions.column(1) - grid_dimensions.column(0))
            .add_scalar(1);
        let work_groups = [
            (max[0] / 4 + 1) as u32,
            (max[1] / 4 + 1) as u32,
            (max[2] / 4 + 1) as u32,
        ];

        let mut shader_builder = ShaderBuilder::new();
        shader_builder.add_dimensions_uniform(0);
        shader_builder.add_distributions(1);
        shader_builder.add_distributions_scratch(2);
        shader_builder.add_lattice_constants();
        shader_builder.add_index_ops_periodic();
        shader_builder.add_stream_main([4, 4, 4]);
        let shader_source = shader_builder.finish("shader_debug/stream.wgsl");

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("stream_pipeline_layout"),
                bind_group_layouts: &[
                    &grid_dimensions_uniform.layout,
                    &distributions.layout,
                    &distributions.layout,
                ],
                push_constant_ranges: &[],
            });

        let shader_module: wgpu::ShaderModule =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("stream_shader_module"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                    &shader_source,
                )),
            });

        let pipeline: wgpu::ComputePipeline =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("stream_pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader_module,
                entry_point: "main",
                compilation_options: wgpu::PipelineCompilationOptions::default(
                ),
                cache: None,
            });

        Stream {
            pipeline,
            work_groups,
        }
    }

    /// Stream distributions, writing to distributions_scratch
    /// NOTE! the caller is responsible for submitting the commands,
    /// waiting for the submission to complete, and swapping the distributions buffers.
    pub fn apply(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        distributions: &Distributions,
        grid_dimensions_uniform: &GridDimensionsUniform,
    ) {
        let mut cpass =
            encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("stream_apply_pass"),
                timestamp_writes: None,
            });
        cpass.set_pipeline(&self.pipeline);
        cpass.set_bind_group(0, &grid_dimensions_uniform.bindgroup, &[]);
        cpass.set_bind_group(1, &distributions.bindgroup, &[]);
        cpass.set_bind_group(2, &distributions.bindgroup_scratch, &[]);
        cpass.dispatch_workgroups(
            self.work_groups[0],
            self.work_groups[1],
            self.work_groups[2],
        );
    }
}
