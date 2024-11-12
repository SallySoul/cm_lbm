use wgpu::util::DeviceExt;

pub struct Driver {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

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

#[repr(C)]
pub struct Dimensions {
    rows: i32,
    cols: i32,
    total: i32,
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
    pub lattice_velocities_input: wgpu::Buffer,
    pub lattice_velocities_output: wgpu::Buffer,

    // Moment Buffers
    pub fluid_density: wgpu::Buffer,
    pub fluid_velocity: wgpu::Buffer,

    // Mappable buffer
    pub data_map: wgpu::Buffer,
    // Bind Groups
    pub lattice_velocities_input_bg: wgpu::BindGroup,
    pub lattice_velocities_output_bg: wgpu::BindGroup,

    // Includes both density and velocity
    pub fluid_moments_bg: wgpu::BindGroup,

    pub dimensions_bg: wgpu::BindGroup,
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
        let lattice_velocities_input = Self::create_storage_buffer(
            &driver.device,
            q * buffer_byte_size,
            "lattice_velocities_input",
        );
        let lattice_velocities_output = Self::create_storage_buffer(
            &driver.device,
            q * buffer_byte_size,
            "lattice_velocities_input",
        );

        let fluid_density =
            Self::create_storage_buffer(&driver.device, buffer_byte_size, "rho_buffer");
        let fluid_velocity =
            Self::create_storage_buffer(&driver.device, 3 * buffer_byte_size, "velocity_buffer");
        let data_map =
            Self::create_mappable_buffer(&driver.device, buffer_byte_size, "data_map_buffer");

        let lattice_velocities_input_bg =
            Self::create_lattice_velocities_bg(&driver.device, &lattice_velocities_input);
        let lattice_velocities_output_bg =
            Self::create_lattice_velocities_bg(&driver.device, &lattice_velocities_output);
        let fluid_moments_bg =
            Self::create_fluid_moments_bg(&driver.device, &fluid_density, &fluid_velocity);
        let dimensions_bg = Self::create_dimension_bg(&driver.device, rows, cols);

        LBM2D {
            dimensions,
            q: q as usize,
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
            dimensions_bg,
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

    fn create_lattice_velocities_bg(
        device: &wgpu::Device,
        buffer: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        let lattice_velocities_layout_label = "lattice_velocities_layout";
        let lattice_velocities_layout: wgpu::BindGroupLayout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                label: Some(&lattice_velocities_layout_label),
            });

        let lattice_velocities_bg_label = "lattice_velocities_bind_group";
        let lattice_velocities_bg: wgpu::BindGroup =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(lattice_velocities_bg_label),
                layout: &lattice_velocities_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
            });

        lattice_velocities_bg
    }

    fn create_fluid_moments_bg(
        device: &wgpu::Device,
        density_buffer: &wgpu::Buffer,
        velocity_buffer: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        // Layout for density and velocity is the same,
        let layout_entries = vec![
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ];

        let fluid_moments_layout_label = "fluid_moments_layout";
        let fluid_moments_layout: wgpu::BindGroupLayout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &layout_entries,
                label: Some(fluid_moments_layout_label),
            });

        let bg_entries = vec![
            wgpu::BindGroupEntry {
                binding: 0,
                resource: density_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: velocity_buffer.as_entire_binding(),
            },
        ];

        let fluid_moments_bg_label = "fluid_moments_bind_group";
        let fluid_moments_bg: wgpu::BindGroup =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(fluid_moments_bg_label),
                layout: &fluid_moments_layout,
                entries: &bg_entries,
            });

        fluid_moments_bg
    }


    fn create_dimension_bg(
        device: &wgpu::Device,
        x: i32,
        y: i32,
    ) -> wgpu::BindGroup {
        let dimension_layout_label = "dimension_layout";
        let dimension_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(dimension_layout_label),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new((3 * std::mem::size_of::<i32>()) as _),
                },
                count: None,
            }],
        });

        let dimension_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[x, y, x * y]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let dimensions_bg_label = "dimensions_bg";
        let dimensions_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(dimensions_bg_label),
            layout: &dimension_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: dimension_buffer.as_entire_binding(),
            }],
        });

        dimensions_bg
    }
}
