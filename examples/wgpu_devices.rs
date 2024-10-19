fn main() {
    let backends = wgpu::Instance::enabled_backend_features();
    println!("enabled_backend_features: {:?}", backends);
    let wgpu_instance = wgpu::Instance::default();
    let adapters = wgpu_instance.enumerate_adapters(backends);
    println!("Instance adapters:");
    for (i, adapter) in adapters.iter().enumerate() {
        println!("  i: {}, {:?}", i, adapter);
        println!("    - features: {:?}", adapter.features());
        println!("    - limits: {:?}", adapter.limits());
        println!("    - info: {:?}", adapter.get_info());
    }
}
