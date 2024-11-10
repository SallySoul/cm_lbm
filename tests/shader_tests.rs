// Compute Shader Reference
//
// This simple integration tests demonstrates a simple hello world level example
// We will allocate a buffer of u32,
// then have the GPU set each buffer element to its index,
// and then we'll add one.

// While using compute shaders in wgpu,
// there are generally two things we'll need
//   * A device instance,
//     - Handle to compute device
//     - Create GPU resources and pipelines
//   * A queue
//     - Handles the device's command queue
//     - Use to submit pipeline passes
//     - Also has other commands, like copy buffer
pub struct Driver {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

async fn setup_wgpu() -> Driver {
    let instance = wgpu::Instance::default();

    // Adapter is how we communicate with the device
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .expect("Failed to create device");

    return Driver { device, queue };
}

/// wgpu relies on asynchonous functions
/// in Rust, that means we need an executor
/// I prefer tokio,
/// This is how we execute an asynchonous test using tokio
#[tokio::test]
async fn add_1() {
    let driver = setup_wgpu().await;

    // Compute shaders are always parallelized over a compute shader grid
    //   * Up to three dimensions, for example (x, y = 1, z = 1)
    //   * We get one invocation of the shader per integer coords into this grid
    //
    // Shader grid dimensions are specified with two factors
    //   * Workgroup size
    //   * Number of workgroups on each axis
    //
    // The shader grid is split into a number of workgroups,
    //   * Can have per-workgroup memory / settings
    //   * Workgroup size is up to three dimensional
    //
    // When dispatching a compute pass, we specify how many workgroups along each axis.

    // The compute shader also needs this be declared.
    // Make sure they match
    // Note we are only using the x-axis for the grid.
    let work_group_size = 128;

    // We're going to use 2 workgroups
    let n_work_groups = 2;

    // So this is the total size of our shader grid
    // We need a data buffer this size.
    let data_buffer_n = work_group_size * n_work_groups;

    // But wgpu is a byte oriented interface,
    // so memory sizes need to be in bytes.
    // For this test we're using u32 data.
    let data_buffer_size_bytes = data_buffer_n * std::mem::size_of::<u32>();

    // Create wgpu buffers
    //
    // Buffers are regions of memory that we can utilize in shaders
    // They are passed into pipelines via a BindGroup
    // Note that Buffers are not general purpose,
    // we must declare up front what kind of operations we want to use them for

    // Our data buffer is marked for STORAGE and COPY_SRC
    // This means we can both read and write to it in compute shaders
    // As well copy its contents to other buffers.
    let data_buffer_label = "data";
    let data_buffer: std::sync::Arc<wgpu::Buffer> =
        std::sync::Arc::new(driver.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&data_buffer_label),
            size: data_buffer_size_bytes as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        }));

    // To get data off the gpu, we need to be able to map a buffer to CPU memory
    // For whatever reason, MAP_* usage enums are mutually exclusive with STORAGE
    // So we need a separate buffer we can copy to and map for CPU usage.
    // Note that this buffer also has COPY_DST,
    // so we can copy the contents of other buffers here
    let data_map_buffer_label = "data_map";
    let data_map_buffer: std::sync::Arc<wgpu::Buffer> =
        std::sync::Arc::new(driver.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&data_map_buffer_label),
            size: data_buffer_size_bytes as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }));

    // Bind group layout
    let data_bind_group_layout_label = "add_1_data_layout";
    let data_bind_group_layout: wgpu::BindGroupLayout =
        driver
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                label: Some(&data_bind_group_layout_label),
            });

    // Bind group
    let data_bind_group_label = "add_1_data_bind_group";
    let data_bind_group: wgpu::BindGroup =
        driver.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&data_bind_group_label),
            layout: &data_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: data_buffer.as_entire_binding(),
            }],
        });

    // Compute
    let set_index_shader_label = "set_1_shader";
    let set_index_shader: wgpu::ShaderModule =
        driver
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(&set_index_shader_label),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                    "shaders/set_index.wgsl"
                ))),
            });

    let add_one_shader_label = "add_1_shader";
    let add_one_shader: wgpu::ShaderModule =
        driver
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(&add_one_shader_label),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                    "shaders/add_one.wgsl"
                ))),
            });

    // Create Pipeline layout
    let set_index_pipeline_layout_label = "set_1_pipeline_layout";
    let set_index_pipeline_layout =
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

    let add_one_pipeline_label = "add_1_pipeline";
    let add_one_pipeline: wgpu::ComputePipeline =
        driver
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(&add_one_pipeline_label),
                layout: Some(&set_index_pipeline_layout),
                module: &add_one_shader,
                entry_point: "main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                cache: None,
            });

    // Run Set one pipeline
    // Create encoder and invoke
    let encoder_label = "default_encoder";
    let mut encoder = driver
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some(&encoder_label),
        });
    {
        let set_index_pass_label = "set_index_compute_pass";
        let mut set_index_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some(&set_index_pass_label),
            timestamp_writes: None,
        });
        set_index_pass.set_pipeline(&set_index_pipeline);
        set_index_pass.set_bind_group(0, &data_bind_group, &[]);
        set_index_pass.dispatch_workgroups(n_work_groups as u32, 1, 1);
    }

    // Ensure we'll copy data into mappable buffer,
    // submit commands,
    // wait for them to complete
    encoder.copy_buffer_to_buffer(
        &data_buffer,
        0,
        &data_map_buffer,
        0,
        data_buffer_size_bytes as u64,
    );
    let submission = driver.queue.submit(Some(encoder.finish()));
    driver
        .device
        .poll(wgpu::Maintain::WaitForSubmissionIndex(submission));

    let capturable = data_map_buffer.clone();
    data_map_buffer
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            if result.is_ok() {
                let view = capturable.slice(..).get_mapped_range();
                let data_view: &[u32] = bytemuck::cast_slice(&view);
                for i in 0u32..data_buffer_n as u32 {
                    assert_eq!(i, data_view[i as usize]);
                }
                drop(view);
                capturable.unmap();
            }
        });

    driver.device.poll(wgpu::Maintain::Wait);

    // Run add pipeline
    let add_encoder_label = "add_one_encoder";
    let mut add_encoder = driver
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some(&add_encoder_label),
        });
    {
        let add_one_pass_label = "add_one_compute_pass";
        let mut add_one_pass = add_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some(&add_one_pass_label),
            timestamp_writes: None,
        });
        add_one_pass.set_pipeline(&add_one_pipeline);
        add_one_pass.set_bind_group(0, &data_bind_group, &[]);
        add_one_pass.dispatch_workgroups(n_work_groups as u32, 1, 1);
    }
    add_encoder.copy_buffer_to_buffer(
        &data_buffer,
        0,
        &data_map_buffer,
        0,
        data_buffer_size_bytes as u64,
    );
    let add_submission = driver.queue.submit(Some(add_encoder.finish()));
    driver
        .device
        .poll(wgpu::Maintain::WaitForSubmissionIndex(add_submission));

    let capturable = data_map_buffer.clone();
    data_map_buffer
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            if result.is_ok() {
                let view = capturable.slice(..).get_mapped_range();
                let data_view: &[u32] = bytemuck::cast_slice(&view);
                for i in 0u32..data_buffer_n as u32 {
                    assert_eq!(i + 1, data_view[i as usize]);
                }
                drop(view);
                capturable.unmap();
            }
        });

    driver.device.poll(wgpu::Maintain::Wait);
}
