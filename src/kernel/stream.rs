// Stream Shader needs
// * input buffer
// * output buffer
// * LatticeDimensions
// * layout descriptors for all of those

use crate::kernel::dimensions::LatticeDimensionsUniform;

pub struct Stream<'a> {
    lattice_dimensions: &'a LatticeDimensionsUniform,
    pipeline: wgpu::ComputePipeline,
}

impl<'a> Stream<'a> {
    pub fn new(
        device: &wgpu::Device,
        density_bg_layout: &wgpu::BindGroupLayout,
        lattice_dimensions: &'a LatticeDimensionsUniform,
    ) -> Self {
        // Compute
        let shader_label = "stream_shader";
        let shader_module: wgpu::ShaderModule =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(shader_label),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                    "../shaders/stream_test.wgsl"
                ))),
            });

        let pipeline_layout_label = "stream_pipeline_layout";
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(pipeline_layout_label),
            bind_group_layouts: &[
                &lattice_dimensions.layout,
                density_bg_layout,
                density_bg_layout,
            ],
            push_constant_ranges: &[],
        });

        let pipeline_label = "stream_pipeline";
        let pipeline: wgpu::ComputePipeline =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(pipeline_label),
                layout: Some(&pipeline_layout),
                module: &shader_module,
                entry_point: "main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                cache: None,
            });

        Stream {
            lattice_dimensions,
            pipeline,
        }
    }

    pub fn stream(
        &self,
        work_groups: [u32; 3],
        encoder: &mut wgpu::CommandEncoder,
        input_buffer: &wgpu::BindGroup,
        output_buffer: &wgpu::BindGroup,
    ) {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("stream"),
            timestamp_writes: None,
        });
        cpass.set_pipeline(&self.pipeline);
        cpass.set_bind_group(0, &self.lattice_dimensions.bind_group, &[]);
        cpass.set_bind_group(1, input_buffer, &[]);
        cpass.set_bind_group(2, output_buffer, &[]);
        cpass.dispatch_workgroups(work_groups[0], work_groups[1], work_groups[2]);
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::wgpu_util::*;

    #[tokio::test]
    async fn test_stream() {
        let device = setup_wgpu().await;

        // Create
    }
}
