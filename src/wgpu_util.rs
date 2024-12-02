pub struct Driver {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

// TODO(error handling)
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
