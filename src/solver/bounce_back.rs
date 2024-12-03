use crate::*;
use wgpu::util::DeviceExt;

/// Bindgroup for bounceback cells
/// Every cell has a flag,
/// if positive, its a bounceback cell, and the flag is the index
/// into the normal buffer
/// Otherwise, normal collision
pub struct BounceBack {
    // bounce_back
    // Number of bounce back cells
    pub normal_buffer: wgpu::Buffer,

    // W * H * L (i32)
    // -1 for normal
    // 0+ for indicies into bounce_back_normals
    pub flag_buffer: wgpu::Buffer,

    // normals bindings 0
    // flag bindings 1
    pub layout: wgpu::BindGroupLayout,
    pub bindgroup: wgpu::BindGroup,
}

impl BounceBack {
    pub fn new_spheres(
        device: &wgpu::Device,
        spheres: &[(Coord3f, f32)],
        grid_dimensions: &AABB3,
        world_coords: &WorldCoords,
        write_vtk: Option<&str>,
    ) -> Self {
        println!("Creating Bounce Back Region");
        let buffer_size = box_buffer_size(grid_dimensions);
        let mut flags = vec![-1; buffer_size];
        let mut normals = Vec::new();
        for coord in coord_iter(*grid_dimensions) {
            let flag_index = coord_to_linear_in_box(&coord, grid_dimensions);
            let coordf = world_coords.convert(&coord);
            for (center, radius) in spheres {
                let mut normal = coordf - center;
                let distance = normal.norm();
                if distance <= *radius {
                    normal /= distance;
                    let norm_index = normals.len() as i32;
                    normals.push(normal);
                    flags[flag_index] = norm_index;
                    break;
                }
            }
        }

        if let Some(path) = write_vtk {
            let mut normals_raw = Vec::with_capacity(3 * buffer_size);
            let mut is_bb = Vec::with_capacity(buffer_size);
            for coord in coord_iter(*grid_dimensions) {
                let flag_index =
                    coord_to_linear_in_box(&coord, grid_dimensions);
                let flag = flags[flag_index];
                if flag < 0 {
                    is_bb.push(0.0);
                    normals_raw.push(0.0);
                    normals_raw.push(0.0);
                    normals_raw.push(0.0);
                } else {
                    is_bb.push(1.0);
                    let normal = normals[flag as usize];
                    normals_raw.push(normal[0]);
                    normals_raw.push(normal[1]);
                    normals_raw.push(normal[2]);
                }
            }

            let mut vtk_grid = VTKGrid::new(grid_dimensions);
            vtk_grid.add_attribute("normal".to_string(), 3, normals_raw);
            vtk_grid.add_attribute("is_bb".to_string(), 1, is_bb);
            vtk_grid.write(path);
        }

        Self::new(device, &flags, &normals)
    }

    pub fn empty(device: &wgpu::Device, grid_dimensions: &AABB3) -> Self {
        let buffer_size = box_buffer_size(grid_dimensions);
        let flags = vec![-1; buffer_size];
        let normals = vec![Vec3::zero()];
        Self::new(device, &flags, &normals)
    }

    fn new(device: &wgpu::Device, flags: &[i32], normals: &[Vec3]) -> Self {
        let normal_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("bounce_back_normal_buffer"),
                contents: bytemuck::cast_slice(&normals),
                usage: wgpu::BufferUsages::STORAGE,
            });

        let flag_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("bounce_back_flag_buffer"),
                contents: bytemuck::cast_slice(&flags),
                usage: wgpu::BufferUsages::STORAGE,
            });

        let layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("bounce_back_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage {
                                read_only: false,
                            },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage {
                                read_only: false,
                            },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bounce_back_bindgroup"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: normal_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: flag_buffer.as_entire_binding(),
                },
            ],
        });

        BounceBack {
            normal_buffer,
            flag_buffer,
            layout,
            bindgroup,
        }
    }
}
