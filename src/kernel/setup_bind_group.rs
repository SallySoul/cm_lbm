use crate::dimensions::*;
use wgpu::util::DeviceExt;

pub fn create_storage_buffer(
    device: &wgpu::Device,
    buffer_byte_size: u64,
    label: Option<&str>,
) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: label,
        size: buffer_byte_size,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

pub fn create_mappable_buffer(
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

pub fn create_buffer_bg(
    device: &wgpu::Device,
    buffer: &wgpu::Buffer,
    layout_label: Option<&str>,
    bg_label: Option<&str>,
) -> wgpu::BindGroup {
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
            label: layout_label,
        });

    let lattice_velocities_bg: wgpu::BindGroup =
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: bg_label,
            layout: &lattice_velocities_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

    lattice_velocities_bg
}

pub fn create_dimension_bg(
    device: &wgpu::Device,
    lattice_dimensions: &LatticeDimensions,
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
        contents: bytemuck::bytes_of(lattice_dimensions),
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
