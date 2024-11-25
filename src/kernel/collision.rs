use crate::kernel::collision_gen::*;
use crate::kernel::Densities;
use crate::kernel::LatticeDimensionsUniform;
use crate::kernel::Macros;
use crate::lattice::D2Q9;
use crate::lattice::D2Q9_W;
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct CollisionParams {
    pub c_s: f32,
    pub delta_t: f32,
    pub tau: f32,
}

pub struct BGKCollision<'a> {
    // params uniform
    params_buffer: wgpu::Buffer,
    params_bindgroup: wgpu::BindGroup,

    // collision pipelines
    pipelines: Vec<wgpu::ComputePipeline>,

    lattice_dimensions: &'a LatticeDimensionsUniform,
}

impl<'a> BGKCollision<'a> {
    pub fn new(
        device: &wgpu::Device,
        densities: &Densities,
        macros: &Macros,
        lattice_dimensions: &'a LatticeDimensionsUniform,
        params: CollisionParams,
    ) -> Self {
        let q = densities.dimensions.q as usize;

        let params_bindgroup_layout_label = "collision_params_layout";
        let params_bindgroup_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(params_bindgroup_layout_label),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            std::mem::size_of::<CollisionParams>() as u64,
                        ),
                    },
                    count: None,
                }],
            });

        let params_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(&params),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let params_bindgroup_label = "params_bindgroup";
        let params_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(params_bindgroup_label),
            layout: &params_bindgroup_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: params_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout_label = "stream_pipeline_layout";
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(pipeline_layout_label),
            bind_group_layouts: &[
                &lattice_dimensions.layout,
                &params_bindgroup_layout,
                &densities.bindgroup_layout,
                &macros.pressure_bindgroup_layout,
                &macros.u_bindgroup_layout,
            ],
            push_constant_ranges: &[],
        });

        let mut pipelines = Vec::with_capacity(q);
        let mut first = true;
        for i in 0..q {
            let dir = D2Q9[i];
            let w_i = D2Q9_W[i];
            let mut b = Collision2DBuilder::new();
            b.add_dimensions_uniform(0);
            b.add_collision_params_uniform(1);
            b.add_input_output(2, 3, 4);
            b.add_index_ops_periodic();
            b.add_main([4, 4, 1], w_i, dir, first);
            first = false;

            let shader_source = b.finish();

            let shader_label = format!("collision_shader_{}", i);
            let shader_module: wgpu::ShaderModule =
                device.create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some(&shader_label),
                    source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(&shader_source)),
                });

            let pipeline_label = format!("collision_pipeline_{}", i);
            let pipeline: wgpu::ComputePipeline =
                device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some(&pipeline_label),
                    layout: Some(&pipeline_layout),
                    module: &shader_module,
                    entry_point: "main",
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    cache: None,
                });

            pipelines.push(pipeline);
        }

        BGKCollision {
            params_buffer,
            params_bindgroup,
            pipelines,
            lattice_dimensions,
        }
    }

    // densities
    // macros
    //
    pub fn collide(
        &self,
        work_groups: [u32; 3],
        encoder: &mut wgpu::CommandEncoder,
        densities: &mut Densities,
        macros: &Macros,
    ) {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("stream"),
            timestamp_writes: None,
        });
        for i in 0..self.lattice_dimensions.dimensions.q as usize {
            cpass.set_pipeline(&self.pipelines[i]);
            cpass.set_bind_group(0, &self.lattice_dimensions.bind_group, &[]);
            cpass.set_bind_group(1, &self.params_bindgroup, &[]);
            cpass.set_bind_group(2, &densities.input_bindgroups[i], &[]);
            cpass.set_bind_group(3, &macros.pressure_bindgroup, &[]);
            cpass.set_bind_group(4, &macros.u_bindgroup, &[]);
            cpass.dispatch_workgroups(work_groups[0], work_groups[1], work_groups[2]);
        }
    }
}
