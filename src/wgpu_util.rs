/// The things we need to run compute shaders
pub struct Driver {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

/// Create WGPU driver
pub async fn setup_wgpu() -> Driver {
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

    Driver { device, queue }
}

/// Create a generic storage buffer
/// that we can copy to and from a map buffer
pub fn create_storage_buffer(
    device: &wgpu::Device,
    buffer_byte_size: u64,
    label: Option<&str>,
) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label,
        size: buffer_byte_size,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

/// Populate a command buffer, submit it, and wait for it to complete.
/// I don't fully understand it, but it seems safe to do this prior to
/// * reading data
/// * swapping distributions buffers
pub fn run_submission<F: FnOnce(&mut wgpu::CommandEncoder)>(driver: &Driver, f: F) {
    let mut encoder =
        driver
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("submission_encoder"),
            });
    f(&mut encoder);
    let submission = driver.queue.submit(Some(encoder.finish()));
    driver
        .device
        .poll(wgpu::Maintain::WaitForSubmissionIndex(submission));
}
