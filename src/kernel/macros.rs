use crate::kernel::densities::Densities;
use crate::kernel::map_buffer::*;
use crate::kernel::pressure_gen::*;
use crate::kernel::setup_bind_group::*;
use crate::kernel::u_gen::*;
use crate::kernel::LatticeDimensionsUniform;
use crate::lattice::D2Q9;
use crate::wgpu_util;

pub struct MacrosData {
    pub pressure: Vec<f32>,
    pub ux: Vec<f32>,
    pub uy: Vec<f32>,
}

pub struct Macros<'a> {
    lattice_dimensions: &'a LatticeDimensionsUniform,
    pressure_bindgroup_layout: wgpu::BindGroupLayout,
    u_bindgroup_layout: wgpu::BindGroupLayout,
    pub macros_bindgroup_layout: wgpu::BindGroupLayout,
    pressure_buffer: wgpu::Buffer,
    ux_buffer: wgpu::Buffer,
    uy_buffer: wgpu::Buffer,
    pressure_bindgroup: wgpu::BindGroup,
    u_bindgroup: wgpu::BindGroup,
    pub macros_bindgroup: wgpu::BindGroup,

    // For the compute pipelines,
    // we have a pass for each of the q densities.
    // Since we don't zero the buffers,
    // the first one does an assign operation,
    // while the rest do an add assign
    pressure_first_pipeline: wgpu::ComputePipeline,
    pressure_rest_pipeline: wgpu::ComputePipeline,
    u_pipelines: Vec<wgpu::ComputePipeline>,
}

impl<'a> Macros<'a> {
    pub fn new(
        device: &wgpu::Device,
        densities: &Densities,
        lattice_dimensions: &'a LatticeDimensionsUniform,
    ) -> Self {
        let buffer_byte_size: u64 = lattice_dimensions.dimensions.float_buffer_byte_size();
        let pressure_buffer_label = "pressure_buffer";
        let pressure_buffer =
            create_storage_buffer(device, buffer_byte_size, Some(&pressure_buffer_label));
        let ux_buffer_label = "ux_buffer";
        let ux_buffer = create_storage_buffer(device, buffer_byte_size, Some(&ux_buffer_label));
        let uy_buffer_label = "uy_buffer";
        let uy_buffer = create_storage_buffer(device, buffer_byte_size, Some(&uy_buffer_label));

        let pressure_bindgroup_layout =
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
                label: Some("pressure_layout"),
            });

        let u_bindgroup_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
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
                ],
                label: Some("u_layout"),
            });

        let macros_bindgroup_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
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
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
                label: Some("macros_bindgroup_layout"),
            });

        let pressure_bindgroup_label = "pressure_bindgroup";
        let pressure_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(pressure_bindgroup_label),
            layout: &pressure_bindgroup_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: pressure_buffer.as_entire_binding(),
            }],
        });

        let u_bindgroup_label = "u_bindgroup";
        let u_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(u_bindgroup_label),
            layout: &u_bindgroup_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: ux_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: uy_buffer.as_entire_binding(),
                },
            ],
        });

        let macros_bindgroup_label = "macros_bindgroup";
        let macros_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(macros_bindgroup_label),
            layout: &macros_bindgroup_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: pressure_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: ux_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: uy_buffer.as_entire_binding(),
                },
            ],
        });

        let pressure_pipeline_layout_label = "pressure_pipeline_layout";
        let pressure_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(pressure_pipeline_layout_label),
                bind_group_layouts: &[
                    &lattice_dimensions.layout,
                    &densities.bindgroup_layout,
                    &pressure_bindgroup_layout,
                ],
                push_constant_ranges: &[],
            });

        let u_pipeline_layout_label = "u_pipeline_layout";
        let u_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(u_pipeline_layout_label),
            bind_group_layouts: &[
                &lattice_dimensions.layout,
                &densities.bindgroup_layout,
                &pressure_bindgroup_layout,
                &u_bindgroup_layout,
            ],
            push_constant_ranges: &[],
        });

        let mut pressure_first_builder = PressureShader2DBuilder::new();
        pressure_first_builder.add_dimensions_uniform(0);
        pressure_first_builder.add_input_output(1, 2);
        pressure_first_builder.add_index_ops_periodic();
        pressure_first_builder.add_main([4, 4, 1], true);
        let pressure_first_source = pressure_first_builder.finish();
        let pressure_first_shader_label = "pressure_shader_first";
        let pressure_first_shader_module: wgpu::ShaderModule =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(pressure_first_shader_label),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                    &pressure_first_source,
                )),
            });

        let mut pressure_rest_builder = PressureShader2DBuilder::new();
        pressure_rest_builder.add_dimensions_uniform(0);
        pressure_rest_builder.add_input_output(1, 2);
        pressure_rest_builder.add_index_ops_periodic();
        pressure_rest_builder.add_main([4, 4, 1], false);
        let pressure_rest_source = pressure_rest_builder.finish();
        let pressure_rest_shader_label = "pressure_shader_rest";
        let pressure_rest_shader_module: wgpu::ShaderModule =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(pressure_rest_shader_label),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(&pressure_rest_source)),
            });

        let pressure_first_pipeline_label = "pressure_first_pipeline";
        let pressure_first_pipeline: wgpu::ComputePipeline =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(pressure_first_pipeline_label),
                layout: Some(&pressure_pipeline_layout),
                module: &pressure_first_shader_module,
                entry_point: "main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                cache: None,
            });

        let pressure_rest_pipeline_label = "pressure_rest_pipeline";
        let pressure_rest_pipeline: wgpu::ComputePipeline =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(pressure_rest_pipeline_label),
                layout: Some(&pressure_pipeline_layout),
                module: &pressure_rest_shader_module,
                entry_point: "main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                cache: None,
            });

        let q = lattice_dimensions.dimensions.q as usize;
        let mut u_pipelines = Vec::with_capacity(q);
        let mut first = true;
        for i in 0..q {
            let dir = D2Q9[i];
            let mut u_builder = UShader2DBuilder::new();
            u_builder.add_dimensions_uniform(0);
            u_builder.add_input_output(1, 2, 3);
            u_builder.add_index_ops_periodic();
            u_builder.add_main([4, 4, 1], dir, first);
            let shader_source = u_builder.finish();
            let shader_label = format!("u_shader_{}", i);
            let shader_module: wgpu::ShaderModule =
                device.create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some(&shader_label),
                    source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(&shader_source)),
                });

            let pipeline_label = format!("u_pipeline_{}", i);
            let pipeline: wgpu::ComputePipeline =
                device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some(&pipeline_label),
                    layout: Some(&u_pipeline_layout),
                    module: &shader_module,
                    entry_point: "main",
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    cache: None,
                });

            u_pipelines.push(pipeline);

            first = false;
        }

        Macros {
            lattice_dimensions,
            pressure_bindgroup_layout,
            u_bindgroup_layout,
            pressure_buffer,
            ux_buffer,
            uy_buffer,
            pressure_bindgroup,
            u_bindgroup,
            pressure_first_pipeline,
            pressure_rest_pipeline,
            u_pipelines,
            macros_bindgroup,
            macros_bindgroup_layout,
        }
    }

    pub fn compute(
        &self,
        work_groups: [u32; 3],
        encoder: &mut wgpu::CommandEncoder,
        densities: &Densities,
    ) {
        let q = self.lattice_dimensions.dimensions.q as usize;
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("stream"),
            timestamp_writes: None,
        });
        cpass.set_pipeline(&self.pressure_first_pipeline);
        cpass.set_bind_group(0, &self.lattice_dimensions.bind_group, &[]);
        cpass.set_bind_group(1, &densities.input_bindgroups[0], &[]);
        cpass.set_bind_group(2, &self.pressure_bindgroup, &[]);
        cpass.dispatch_workgroups(work_groups[0], work_groups[1], work_groups[2]);
        cpass.set_pipeline(&self.pressure_rest_pipeline);
        for i in 1..q {
            cpass.set_bind_group(0, &self.lattice_dimensions.bind_group, &[]);
            cpass.set_bind_group(1, &densities.input_bindgroups[i], &[]);
            cpass.set_bind_group(2, &self.pressure_bindgroup, &[]);
            cpass.dispatch_workgroups(work_groups[0], work_groups[1], work_groups[2]);
        }

        for i in 0..q {
            cpass.set_pipeline(&self.u_pipelines[i]);
            cpass.set_bind_group(0, &self.lattice_dimensions.bind_group, &[]);
            cpass.set_bind_group(1, &densities.input_bindgroups[i], &[]);
            cpass.set_bind_group(2, &self.pressure_bindgroup, &[]);
            cpass.set_bind_group(3, &self.u_bindgroup, &[]);
            cpass.dispatch_workgroups(work_groups[0], work_groups[1], work_groups[2]);
        }
    }

    pub fn get_data(&self, driver: &wgpu_util::Driver) -> MacrosData {
        let read_buffer = ReadMapBuffer::new(&driver.device, &self.lattice_dimensions.dimensions);
        let pressure = read_buffer.clone_data(driver, &self.pressure_buffer);
        let ux = read_buffer.clone_data(driver, &self.ux_buffer);
        let uy = read_buffer.clone_data(driver, &self.uy_buffer);
        MacrosData { pressure, ux, uy }
    }
}
