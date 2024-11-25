// Stream Shader needs
// * input buffers
// * output buffers
// * LatticeDimensions
// * layout descriptors for all of those

use crate::kernel::dimensions::LatticeDimensionsUniform;
use crate::kernel::stream_gen::*;
use crate::lattice::D2Q9;

use super::Densities;

pub struct Stream2D<'a> {
    lattice_dimensions: &'a LatticeDimensionsUniform,
    pipelines: Vec<wgpu::ComputePipeline>,
}

impl<'a> Stream2D<'a> {
    pub fn new(
        device: &wgpu::Device,
        density_bg_layout: &wgpu::BindGroupLayout,
        lattice_dimensions: &'a LatticeDimensionsUniform,
    ) -> Self {
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

        let mut pipelines = Vec::with_capacity(lattice_dimensions.dimensions.q as usize);
        for i in 0..lattice_dimensions.dimensions.q as usize {
            // Compute
            let dir = D2Q9[i];
            let mut b = StreamShader2DBuilder::new();
            b.add_dimensions_uniform(0);
            b.add_input_output(1, 2);
            b.add_index_ops_periodic();
            b.add_main(dir, [4, 4, 1]);
            let shader_source = b.finish();

            let shader_label = format!("stream_shader_{:?}", dir);
            let shader_module: wgpu::ShaderModule =
                device.create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some(&shader_label),
                    source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(&shader_source)),
                });

            let pipeline_label = format!("stream_pipeline_{:?}", dir);
            let pipeline: wgpu::ComputePipeline =
                device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some(&pipeline_label),
                    layout: Some(&pipeline_layout),
                    module: &shader_module,
                    entry_point: "main",
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    cache: None,
                });

            pipelines.push(pipeline);
        }

        Stream2D {
            lattice_dimensions,
            pipelines,
        }
    }

    pub fn stream(
        &self,
        work_groups: [u32; 3],
        encoder: &mut wgpu::CommandEncoder,
        densities: &mut Densities,
    ) {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("stream"),
            timestamp_writes: None,
        });
        for i in 0..self.lattice_dimensions.dimensions.q as usize {
            cpass.set_pipeline(&self.pipelines[i]);
            cpass.set_bind_group(0, &self.lattice_dimensions.bind_group, &[]);
            cpass.set_bind_group(1, &densities.input_bindgroups[i], &[]);
            cpass.set_bind_group(2, &densities.output_bindgroups[i], &[]);
            cpass.dispatch_workgroups(work_groups[0], work_groups[1], work_groups[2]);
        }
        std::mem::swap(&mut densities.input_bindgroups, &mut densities.output_bindgroups);
        std::mem::swap(&mut densities.input_buffers, &mut densities.output_buffers);
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[tokio::test]
    async fn test_stream_2d() {
        // Make Dimensions
        let driver = crate::setup_wgpu().await;
    }
}
