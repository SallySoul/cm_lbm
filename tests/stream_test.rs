#[tokio::test]
async fn stream_test() {
    let driver = cm_lbm::setup_wgpu().await;

    // Dimensions
    let rows = 10;
    let cols = 10;
    let q = 9;
    let lattice_dimensions = cm_lbm::kernel::LatticeDimensionsUniform::new(
        &driver.device,
        cm_lbm::LatticeDimensions {
            rows,
            cols,
            total: rows * cols,
            q,
        },
    );

    let densities = cm_lbm::kernel::Densities::new(&driver.device, &lattice_dimensions.dimensions);

    let stream_pipeline =
        cm_lbm::kernel::Stream::new(&driver.device, &densities.layout, &lattice_dimensions);

    // Create encoder and invoke
    let encoder_label = "default_encoder";
    let mut encoder = driver
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some(encoder_label),
        });

    stream_pipeline.stream(
        [13, 13, 1],
        &mut encoder,
        &densities.input_bind_group,
        &densities.output_bind_group,
    );

    let buffer_byte_size: u64 =
        lattice_dimensions.dimensions.total as u64 * std::mem::size_of::<f32>() as u64;

    let map_buffer_label = "map_buffer";
    let map_buffer = std::sync::Arc::new(driver.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(map_buffer_label),
        size: buffer_byte_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }));

    encoder.copy_buffer_to_buffer(
        &densities.output_buffer,
        0,
        &map_buffer,
        0,
        buffer_byte_size,
    );

    let submission = driver.queue.submit(Some(encoder.finish()));
    driver
        .device
        .poll(wgpu::Maintain::WaitForSubmissionIndex(submission));

    // create mappable buffer, and collect
}
