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
    )

    // create mappable buffer, and collect
}
