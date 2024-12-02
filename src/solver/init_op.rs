use crate::*;
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

pub fn set_initial_conditions(
    driver: &Driver,
    distributions: &Distributions,
    bc_params_uniform: &BCParamsUniform,
    grid_dimensions_uniform: &GridDimensionsUniform,
    work_groups: &[u32; 3],
) {
    println!("Set Initial Conditions");
    let device = &driver.device;

    // need shader module
    let mut shader_builder = ShaderBuilder::new();
    shader_builder.add_dimensions_uniform(0);
    shader_builder.add_bc_params_uniform(1);
    shader_builder.add_distributions(2);
    shader_builder.add_lattice_constants();
    shader_builder.add_index_ops_periodic();
    shader_builder.add_equil_fn();
    shader_builder.add_init_main([4, 4, 4]);
    let shader_source = shader_builder.finish("shader_debug/init.wgsl");

    let pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ic_pipeline_layout"),
            bind_group_layouts: &[
                &grid_dimensions_uniform.layout,
                &bc_params_uniform.layout,
                &distributions.layout,
            ],
            push_constant_ranges: &[],
        });

    let shader_module: wgpu::ShaderModule =
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ic_shader_module"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                &shader_source,
            )),
        });

    let pipeline: wgpu::ComputePipeline =
        device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("ic_pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader_module,
            entry_point: "main",
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("ic_encoder"),
        });
    {
        let mut cpass =
            encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("ic_pass"),
                timestamp_writes: None,
            });

        cpass.set_pipeline(&pipeline);
        cpass.set_bind_group(0, &grid_dimensions_uniform.bindgroup, &[]);
        cpass.set_bind_group(1, &bc_params_uniform.bindgroup, &[]);
        cpass.set_bind_group(2, &distributions.bindgroup, &[]);
        cpass.dispatch_workgroups(
            work_groups[0],
            work_groups[1],
            work_groups[2],
        );
    }

    let submission = driver.queue.submit(Some(encoder.finish()));
    device.poll(wgpu::Maintain::WaitForSubmissionIndex(submission));
}
