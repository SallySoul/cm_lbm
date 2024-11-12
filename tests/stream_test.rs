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

    let write_map = cm_lbm::kernel::WriteMapBuffer::new(&driver.device, &lattice_dimensions.dimensions);
    let read_map = cm_lbm::kernel::ReadMapBuffer::new(&driver.device, &lattice_dimensions.dimensions);



    let densities = cm_lbm::kernel::Densities::new(&driver.device, &lattice_dimensions.dimensions);

        // Write and Read
        write_map.write_data(&driver, &densities.input_buffer, |slice| {
            assert_eq!(slice.len(), 100);
            for (i, x) in slice.iter_mut().enumerate() {
                *x = i as f32;
            }
            println!("slice before: {:?}", slice);
        });

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

    driver.device.poll(wgpu::Maintain::Wait);

    read_map.read_data(&driver, &densities.output_buffer, |slice| {
            println!("slice result: {:?}", slice);
            assert_eq!(slice.len(), 100);
            for (i, x) in slice.iter().enumerate() {
                assert_eq!(*x, i as f32 * 9.0);
            }
    });
}
