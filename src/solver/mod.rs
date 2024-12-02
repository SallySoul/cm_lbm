mod bc_uniform;
mod bounce_back;
mod dimensions_uniform;
mod distributions;
mod faces;
mod init_op;
mod moments;
mod shader_template;

pub use bc_uniform::*;
pub use bounce_back::*;
pub use dimensions_uniform::*;
pub use distributions::*;
pub use faces::*;
pub use init_op::*;
pub use moments::*;
pub use shader_template::*;

use crate::*;
use lattice::*;
use nalgebra::{matrix, vector};
use std::collections::HashSet;
use vtkio::model::*;

struct StreamPipelines {}

pub struct Solver {
    grid_dimensions: AABB3,

    // coord AABBs for mutually exclusive faces
    faces: Faces,

    work_groups: [u32; 3],

    grid_dimensions_uniform: GridDimensionsUniform,

    bc_params_uniform: BCParamsUniform,

    bounce_back: BounceBack,

    distributions: Distributions,

    moments: Moments,

    // W * H * L * 3
    // velocity_buffer: wgpu::Buffer,

    // Peform stream op into density buffer
    // stream_pipelines: [wgpu::ComputePipeline; 27],

    // Copy density buffer into distributions
    // copy_pipelines: [wgpu::ComputePipeline; 27],

    // moments?
    // err lets have collision just do it
    // we can pull it out later
    // collision_pipeline: wgpu::ComputePipeline,
    omega: f32,
}

impl Solver {
    pub fn new(
        driver: &Driver,
        bounce_back: BounceBack,
        bc_params_uniform: BCParamsUniform,
        grid_dimensions: AABB3,
        omega: f32,
    ) -> Self {
        let device = &driver.device;
        let faces = Faces::new(grid_dimensions);
        let grid_dimensions_uniform =
            GridDimensionsUniform::new(device, &grid_dimensions);
        let distributions = Distributions::new(device, &grid_dimensions);

        let max = (grid_dimensions.column(1) - grid_dimensions.column(0))
            .add_scalar(1);

        let work_groups = [
            (max[0] / 4 + 1) as u32,
            (max[1] / 4 + 1) as u32,
            (max[2] / 4 + 1) as u32,
        ];

        let moments = Moments::new(
            device,
            &grid_dimensions,
            &distributions,
            &grid_dimensions_uniform,
        );

        set_initial_conditions(
            driver,
            &distributions,
            &bc_params_uniform,
            &grid_dimensions_uniform,
            &work_groups,
        );

        Solver {
            grid_dimensions,
            faces,
            work_groups,
            grid_dimensions_uniform,
            bc_params_uniform,
            bounce_back,
            distributions,
            moments,
            // density_stream_buffer,
            // velocity_buffer,
            // stream_pipelines,
            // copy_pipelines,
            omega,
        }
    }

    pub fn moments(&self, encoder: &mut wgpu::CommandEncoder) {
        self.moments.compute(
            &self.work_groups,
            encoder,
            &self.distributions,
            &self.grid_dimensions_uniform,
        );
    }

    pub fn write_vtk(&self, driver: &Driver, path: &str) {
        println!("  writing VTK: {}", path);
        let mut grid = VTKGrid::new(&self.grid_dimensions);

        let (densities, velocities) = self.moments.get_data(driver);
        grid.add_attribute("density".to_string(), 1, densities);
        grid.add_attribute("velocity".to_string(), 3, velocities);

        grid.write(path);
    }

    pub fn streaming(&mut self) {}

    pub fn collision(&mut self) {}
}
/*
    pub fn equilibrium_init(&mut self) {
        for coord in coord_iter(self.grid_dimensions) {
            for q in 0..27 {
                let value = self.inflow_density * D3Q27_W[q];
                self.distributions.set_q(&coord, q as i32, value)
            }
        }
    }

    pub fn flow_init(&mut self) {
        let f_eq = f_equilibrium(&self.directions, self.inflow_density, self.inflow_velocity);

        for coord in coord_iter(self.grid_dimensions) {
            for q_i in 0..27 {
                self.distributions.set_q(&coord, q_i as i32, f_eq[q_i]);
            }
        }
    }

    pub fn streaming(&mut self) {
        for coord in coord_iter(self.grid_dimensions) {
            for q_i in 0..27 {
                // Get q value
                let q = self.distributions.get_q(&coord, q_i);

                // Get neighbor intex
                let neighbor_coord = coord + self.offsets[q_i as usize];

                let periodic_n_c = periodic_coord(&neighbor_coord, &self.grid_dimensions);

                self.distributions_buffer.set_q(&periodic_n_c, q_i, q);
            }
        }
        std::mem::swap(&mut self.distributions, &mut self.distributions_buffer);
    }

    pub fn moments(&mut self) {
        for coord in coord_iter(self.grid_dimensions) {
            let mut pressure = 0.0;
            let mut u = Vec3::zero();
            for q_i in 0..27 {
                let q = self.distributions.get_q(&coord, q_i);
                pressure += q;
                u += self.directions[q_i as usize] * q;
            }
            if pressure.abs() > 0.00001 {
                u /= pressure;
            } else {
                println!("ERROR");
            }
            self.pressure.set(&coord, pressure);
            self.velocity.set(&coord, u);
        }
    }

    pub fn collide_coord(&mut self, coord: &Coord<3>) {
        let u = self.velocity.get(coord);
        let p = self.pressure.get(coord);
        let mut i_d = 0.0;
        let mut n_d = 0.0;
        let mut eq_d = 0.0;
        for q_i in 0..27 {
            // Calculate equilibrium
            let dir = self.directions[q_i as usize];
            let dir_u = dir.dot(&u);
            let w_i = D3Q27_W[q_i as usize];

            let t1 = 3.0 * dir_u;
            let t2 = 9.0 * dir_u * dir_u;
            let t3 = -(3.0 * u.dot(&u));
            let q_eq = w_i * p * (1.0 + t1 + t2 + t3);

            // relax
            let q = self.distributions.get_q(coord, q_i);
            let new_q = q + self.omega * (q_eq - q);
            //let new_q = (1.0 - self.omega) * q + self.omega * q_eq;
            self.distributions.set_q(coord, q_i, new_q);

            i_d += q;
            n_d += new_q;
            eq_d += q_eq;
        }
    }

    pub fn apply_bc(&mut self, coord: &Coord<3>) {
        let f_eq = f_equilibrium(&self.directions, self.inflow_density, self.inflow_velocity);
        for q_i in 0..27 {
            self.distributions.set_q(coord, q_i, f_eq[q_i as usize]);
        }
    }

    pub fn reflect_xz(&mut self, coord: &Coord<3>) {
        let density = self.pressure.get(coord);
        let mut velocity = self.velocity.get(coord);
        velocity[1] *= -1.0;

        let f_eq = f_equilibrium(&self.directions, density, velocity);
        for q_i in 0..27 {
            self.distributions.set_q(coord, q_i, f_eq[q_i as usize]);
        }
    }

    pub fn reflect_yz(&mut self, coord: &Coord<3>) {
        let density = self.pressure.get(coord);
        let mut velocity = self.velocity.get(coord);
        velocity[0] *= -1.0;

        let f_eq = f_equilibrium(&self.directions, density, velocity);
        for q_i in 0..27 {
            self.distributions.set_q(coord, q_i, f_eq[q_i as usize]);
        }
    }

    pub fn collision(&mut self) {
        for coord in coord_iter(self.grid_dimensions) {
            if self.bc_coords.contains(&coord) {
                self.apply_bc(&coord);
            } else if self.x_reflect_coords.contains(&coord) {
                self.reflect_xz(&coord);
            } else if self.y_reflect_coords.contains(&coord) {
                self.reflect_yz(&coord);
            } else if self.bounce_back.contains(&coord) {
                self.apply_bounce_back(&coord);
            } else {
                self.collide_coord(&coord);
            }
        }
    }

    pub fn apply_bounce_back(&mut self, coord: &Coord<3>) {
        let mut new_q = [0.0; 27];
        for q_i in 0..27 {
            let q = self.distributions.get_q(coord, q_i);
            new_q[D3Q27_OPP[q_i as usize]] = q;
        }
        for q_i in 0..27 {
            self.distributions.set_q(coord, q_i, new_q[q_i as usize]);
        }
    }
    /*
        pub fn apply_inflow_bc(&mut self, coord &Coord<3>) {
            for q_i in 0..27 {
                let t = self.inflow_density * self.inflow_accel * D3Q27_W[q_i];
            }


        }

        pub fn apply_outflow_bc(&mut self, coord: &Coord<3>) {
            for q_i in 0..27 {
                let t = self.inflow_density * self.inflow_accel * D3Q27_W[q_i];
      k      }
        }
    */

    pub fn write_vtk(&self, i: usize) {
        let buffer_size = box_buffer_size(&self.grid_dimensions);
        //let distributions = vec![vec![0.0; buffer_size]; 27];
        let mut bc = Vec::with_capacity(buffer_size);
        let mut x_r = Vec::with_capacity(buffer_size);
        let mut y_r = Vec::with_capacity(buffer_size);
        let mut bb = Vec::with_capacity(buffer_size);
        let mut density = Vec::with_capacity(buffer_size);
        let mut velocity = Vec::with_capacity(3 * buffer_size);
        let mut points = Vec::with_capacity(3 * buffer_size);
        let mut qs = vec![Vec::with_capacity(buffer_size); 27];
        for coord in coord_iter(self.grid_dimensions) {
            points.push(coord[0] as f32);
            points.push(coord[1] as f32);
            points.push(coord[2] as f32);

            bc.push(if self.bc_coords.contains(&coord) {
                1.0
            } else {
                0.0
            });


            bb.push(if self.bounce_back.contains(&coord) {
                1.0
            } else {
                0.0
            });

            x_r.push(if self.x_reflect_coords.contains(&coord) {
                1.0
            } else {
                0.0
            });

            y_r.push(if self.y_reflect_coords.contains(&coord) {
                1.0
            } else {
                0.0
            });

            density.push(self.pressure.get(&coord));

            let vel = self.velocity.get(&coord);
            velocity.push(vel[0]);
            velocity.push(vel[1]);
            velocity.push(vel[2]);

            for q_i in 0..27 {
                let q = self.distributions.get_q(&coord, q_i);
                qs[q_i as usize].push(q);
            }
        }

        let n_cells = cell_count(self.grid_dimensions);
        let mut connectivity = Vec::with_capacity(n_cells);
        let mut offsets = Vec::with_capacity(n_cells);
        let mut cell_types = Vec::with_capacity(n_cells);
        let mut offset = 8;
        for cell_coord in cell_coord_iter(self.grid_dimensions) {
            let n_1 = cell_coord + vector![0, 0, 1];
            let n_2 = cell_coord + vector![0, 1, 0];
            let n_3 = cell_coord + vector![1, 0, 0];
            let n_4 = cell_coord + vector![0, 1, 1];
            let n_5 = cell_coord + vector![1, 1, 0];
            let n_6 = cell_coord + vector![1, 0, 1];
            let n_7 = cell_coord + vector![1, 1, 1];

            let vertices = [&cell_coord, &n_3, &n_6, &n_1, &n_2, &n_5, &n_7, &n_4];
            for v in vertices {
                let index = coord_to_linear_in_box(v, &self.grid_dimensions) as u64;
                connectivity.push(index);
            }

            offsets.push(offset);
            cell_types.push(CellType::Hexahedron);
            offset += 8;
        }

        let mut point_attributes = vec![
            Attribute::DataArray(DataArrayBase {
                name: format!("bc"),
                elem: ElementType::Scalars {
                    num_comp: 1,
                    lookup_table: None,
                },
                data: IOBuffer::F32(bc),
            }),
            Attribute::DataArray(DataArrayBase {
                name: format!("bb"),
                elem: ElementType::Scalars {
                    num_comp: 1,
                    lookup_table: None,
                },
                data: IOBuffer::F32(bb),
            }),
            Attribute::DataArray(DataArrayBase {
                name: format!("x_r"),
                elem: ElementType::Scalars {
                    num_comp: 1,
                    lookup_table: None,
                },
                data: IOBuffer::F32(x_r),
            }),
            Attribute::DataArray(DataArrayBase {
                name: format!("y_r"),
                elem: ElementType::Scalars {
                    num_comp: 1,
                    lookup_table: None,
                },
                data: IOBuffer::F32(y_r),
            }),
            Attribute::DataArray(DataArrayBase {
                name: format!("density"),
                elem: ElementType::Scalars {
                    num_comp: 1,
                    lookup_table: None,
                },
                data: IOBuffer::F32(density),
            }),
            Attribute::DataArray(DataArrayBase {
                name: format!("velocity"),
                elem: ElementType::Scalars {
                    num_comp: 3,
                    lookup_table: None,
                },
                data: IOBuffer::F32(velocity),
            }),
        ];

        for q_i in 0..27 {
            point_attributes.push(Attribute::DataArray(DataArrayBase {
                name: format!("q_{}", q_i),
                elem: ElementType::Scalars {
                    num_comp: 1,
                    lookup_table: None,
                },
                data: IOBuffer::F32(qs[q_i].clone()),
            }));
        }

        Vtk {
            version: Version { major: 1, minor: 0 },
            title: String::new(),
            byte_order: ByteOrder::LittleEndian,
            file_path: None,
            data: DataSet::inline(UnstructuredGridPiece {
                points: IOBuffer::F32(points),
                cells: Cells {
                    cell_verts: VertexNumbers::XML {
                        connectivity,
                        offsets,
                    },
                    types: cell_types,
                },
                data: Attributes {
                    point: point_attributes,
                    cell: vec![],
                },
            }),
        }
        .export(format!("vtk_test/data_{:06}.vtu", i))
        .unwrap();
    }
}
*/
