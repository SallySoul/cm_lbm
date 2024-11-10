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

#[repr(C)]
pub struct Dimensions {
    rows: u32,
    cols: u32,
    total: u32,
}

// Assume inflow along x = 0
// Assume outflow along x = cols - 1
// Assume bounce back along y = 0 and y = rows - 1
pub struct LBM2D {
    // Domain specification
    pub dimensions: Dimensions,
    pub q: usize,
    pub workgroup_size: u32,
    pub workgroup_n: u32,

    // Velocity Bufers
    // We need a buffer for each q direction
    // We also treat compute shaders as per functions,
    // i.e. no in-place modifications,
    // so we have and input group and and output group
    pub lattice_velocities_input: Vec<wgpu::Buffer>,
    pub lattice_velocities_output: Vec<wgpu::Buffer>,

    // Moment Buffers
    pub fluid_density: wgpu::Buffer,
    pub fluid_velocity: Vec<wgpu::Buffer>,

    // Mappable buffer
    pub data_map: wgpu::Buffer,
    // Bind Groups
    pub lattice_velocities_input_bg: wgpu::BindGroup,
    pub lattice_velocities_output_bg: wgpu::BindGroup,

    // Includes both density and velocity
    pub fluid_moments_bg: wgpu::BindGroup,
    /*

    // Compute Pipelines
    stream_pipeline: wgpu::ComputePipeline,
    moment_pipeline: wgpu::ComputePipeline,
    collision_pipeline: wgpu::ComputePipeline,

    */
}

impl LBM2D {
    pub fn new(driver: &Driver, rows: u32, cols: u32) -> LBM2D {
        // Set Grid Dimensions / Parameters
        let total = rows * cols;
        let q = 9;
        let dimension: usize = 2;
        let dimensions = Dimensions { rows, cols, total };
        let workgroup_size = 64;
        let mut workgroup_n = total / workgroup_size;
        if total % workgroup_size > 0 {
            workgroup_n += 1;
        }
        let buffer_byte_size: u64 = total as u64 * std::mem::size_of::<f32>() as u64;

        // Create buffers
        let (lattice_velocities_input, lattice_velocities_output) =
            Self::create_lattice_velocity_buffers(&driver.device, q, buffer_byte_size);
        let fluid_density =
            Self::create_storage_buffer(&driver.device, buffer_byte_size, "rho_buffer");
        let fluid_velocity =
            Self::create_fluid_velocity_buffers(&driver.device, dimension, buffer_byte_size);
        let data_map =
            Self::create_mappable_buffer(&driver.device, buffer_byte_size, "data_map_buffer");

        let lattice_velocities_input_bg =
            Self::create_lattice_velocities_bg(&driver.device, &lattice_velocities_input);
        let lattice_velocities_output_bg =
            Self::create_lattice_velocities_bg(&driver.device, &lattice_velocities_output);
        let fluid_moments_bg =
            Self::create_fluid_moments_bg(&driver.device, &fluid_density, &fluid_velocity);

        LBM2D {
            dimensions,
            q,
            workgroup_size,
            workgroup_n,
            lattice_velocities_input,
            lattice_velocities_output,
            fluid_density,
            fluid_velocity,
            data_map,
            lattice_velocities_input_bg,
            lattice_velocities_output_bg,
            fluid_moments_bg,
        }
    }

    fn create_storage_buffer(
        device: &wgpu::Device,
        buffer_byte_size: u64,
        label: &str,
    ) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: buffer_byte_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        })
    }

    fn create_mappable_buffer(
        device: &wgpu::Device,
        buffer_byte_size: u64,
        label: &str,
    ) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: buffer_byte_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn create_lattice_velocity_buffers(
        device: &wgpu::Device,
        q: usize,
        buffer_byte_size: u64,
    ) -> (Vec<wgpu::Buffer>, Vec<wgpu::Buffer>) {
        let mut inputs = Vec::with_capacity(q);
        let mut outputs = Vec::with_capacity(q);
        let mut a_label;
        let mut b_label;
        for i in 0..q {
            a_label = format!("a_vel_buffer_{}", i);
            inputs.push(Self::create_storage_buffer(
                device,
                buffer_byte_size,
                &a_label,
            ));
            b_label = format!("b_vel_buffer_{}", i);
            outputs.push(Self::create_storage_buffer(
                device,
                buffer_byte_size,
                &b_label,
            ));
        }

        (inputs, outputs)
    }

    fn create_fluid_velocity_buffers(
        device: &wgpu::Device,
        dimension: usize,
        buffer_byte_size: u64,
    ) -> Vec<wgpu::Buffer> {
        let mut result = Vec::with_capacity(dimension);
        let mut label;
        for i in 0..dimension {
            label = format!("fluid_velocity_buffer_{}", i);
            result.push(Self::create_storage_buffer(
                device,
                buffer_byte_size,
                &label,
            ));
        }
        result
    }

    fn create_lattice_velocities_bg(
        device: &wgpu::Device,
        buffers: &[wgpu::Buffer],
    ) -> wgpu::BindGroup {
        let n_buffers = buffers.len();
        let mut layout_entries = Vec::with_capacity(n_buffers);
        for i in 0..n_buffers as u32 {
            layout_entries.push(wgpu::BindGroupLayoutEntry {
                binding: i,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            });
        }

        let lattice_velocities_layout_label = "lattice_velocities_layout";
        let lattice_velocities_layout: wgpu::BindGroupLayout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &layout_entries,
                label: Some(&lattice_velocities_layout_label),
            });

        let mut bg_entries = Vec::with_capacity(n_buffers);
        for (i, buffer) in buffers.iter().enumerate() {
            bg_entries.push(wgpu::BindGroupEntry {
                binding: i as u32,
                resource: buffer.as_entire_binding(),
            });
        }

        let lattice_velocities_bg_label = "lattice_velocities_bind_group";
        let lattice_velocities_bg: wgpu::BindGroup =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(lattice_velocities_bg_label),
                layout: &lattice_velocities_layout,
                entries: &bg_entries,
            });

        lattice_velocities_bg
    }

    fn create_fluid_moments_bg(
        device: &wgpu::Device,
        density_buffer: &wgpu::Buffer,
        velocity_buffers: &[wgpu::Buffer],
    ) -> wgpu::BindGroup {
        let n_vel_buffers = velocity_buffers.len();
        // Layout for density and velocity is the same,
        let mut layout_entries = Vec::with_capacity(n_vel_buffers + 1);
        for i in 0..n_vel_buffers as u32 + 1 {
            layout_entries.push(wgpu::BindGroupLayoutEntry {
                binding: i,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            });
        }

        let fluid_moments_layout_label = "fluid_moments_layout";
        let fluid_moments_layout: wgpu::BindGroupLayout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &layout_entries,
                label: Some(&fluid_moments_layout_label),
            });

        let mut bg_entries = Vec::with_capacity(n_vel_buffers + 1);
        bg_entries.push(wgpu::BindGroupEntry {
            binding: 0,
            resource: density_buffer.as_entire_binding(),
        });
        for (i, buffer) in velocity_buffers.iter().enumerate() {
            bg_entries.push(wgpu::BindGroupEntry {
                binding: i as u32 + 1,
                resource: buffer.as_entire_binding(),
            });
        }

        let fluid_moments_bg_label = "fluid_moments_bind_group";
        let fluid_moments_bg: wgpu::BindGroup =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(fluid_moments_bg_label),
                layout: &fluid_moments_layout,
                entries: &bg_entries,
            });

        fluid_moments_bg
    }
}
