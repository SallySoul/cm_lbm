/*

pub fn create_pipeline(
    device: &wgpu::Device,
    layout_label: &str,
    label: Option<&str>
) {
// Create Pipeline layout
    let pipeline_layout_label = "set_1_pipeline_layout";
    let pipeline_layout =
        driver
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(&set_index_pipeline_layout_label),
                bind_group_layouts: &[&data_bind_group_layout],
                push_constant_ranges: &[],
            });

    // pipeline
    let set_index_pipeline_label = "set_1_pipeline";
    let set_index_pipeline: wgpu::ComputePipeline =
        driver
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(&set_index_pipeline_label),
                layout: Some(&set_index_pipeline_layout),
                module: &set_index_shader,
                entry_point: "main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                cache: None,
            });


            */
