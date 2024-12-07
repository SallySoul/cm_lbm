/// This is the kitchen sink class.
///
/// Shaders share alot of code,
/// indexing operations, constants, bind groups, ect.
///
/// Given the difficulties with tracking down bugs,
/// I really want it de-duplicated,
/// or at least all in one place.
///
/// However I'm not setup to do
/// any high level symbolic manipulation (yet)
/// so I just rely on templating, which is not very readable.
/// But every single shader we use is generated
/// by this file.
use crate::*;
use cm_lbm_generated::shader_ops;
use std::io::prelude::*;

#[derive(Clone)]
pub struct ShaderBuilder {
    buffer: String,
}

impl ShaderBuilder {
    pub fn new() -> Self {
        ShaderBuilder {
            buffer: String::new(),
        }
    }

    pub fn add_dimensions_uniform(&mut self, group: u32) {
        self.buffer += &format!(
            "
struct GridDimensions {{
    max: vec3<i32>,
    total: i32,
}}
@group({}) @binding(0) 
var<uniform> dimensions: GridDimensions;
",
            group
        );
    }

    pub fn add_collision_uniform(&mut self, group: u32) {
        self.buffer += &format!(
            "
struct GridDimensions {{
    max: vec3<i32>,
    total: i32,
}}
@group({g}) @binding(0) 
var<uniform> dimensions: GridDimensions;
@group({g}) @binding(1) 
var<uniform> interior_dimensions: GridDimensions;
",
            g = group
        );
    }

    pub fn add_face_uniform(&mut self, group: u32) {
        self.buffer += &format!(
            "
@group({}) @binding(0) 
var<uniform> face_dimensions: GridDimensions;
    ",
            group
        );
    }

    pub fn add_interior_uniform(&mut self, group: u32) {
        self.buffer += &format!(
            "
@group({}) @binding(0) 
var<uniform> interior_dimensions: GridDimensions;
    ",
            group
        );
    }

    pub fn add_bc_params_uniform(&mut self, group: u32) {
        self.buffer += &format!(
            "
struct BCParams {{
    velocity: vec3<f32>,
    density: f32,
}}
@group({}) @binding(0)
var<uniform> bc_params: BCParams;
",
            group
        );
    }

    pub fn add_distributions(&mut self, group: u32) {
        self.buffer += &format!(
            "
@group({g}) @binding(0) 
var<storage, read_write> distributions: array<f32>;
",
            g = group,
        );
    }

    pub fn add_distributions_scratch(&mut self, group: u32) {
        self.buffer += &format!(
            "
@group({g}) @binding(0) 
var<storage, read_write> distributions_scratch: array<f32>;
",
            g = group,
        );
    }

    pub fn add_bounceback_bindgroup(&mut self, group: u32) {
        self.buffer += &format!(
            "
@group({g}) @binding(0)
var<storage, read_write> bb_normals: array<f32>;
@group({g}) @binding(1)
var<storage, read_write> bb_flag: array<i32>;

fn get_normal(index: i32) -> vec3<f32> {{
    let base = index * 3;
    return vec3(
        bb_normals[base],
        bb_normals[base + 1],
        bb_normals[base + 2],
  ); 
}}
",
            g = group,
        );
    }

    pub fn add_moments_bindgroup(&mut self, group: u32) {
        self.buffer += &format!(
            "
@group({g}) @binding(0) 
var<storage, read_write> densities: array<f32>;
@group({g}) @binding(1) 
var<storage, read_write> velocities: array<f32>;

fn set_velocity(index: i32, velocity: vec3<f32>) {{
    let base = index * 3;
    velocities[base] = velocity[0];
    velocities[base + 1] = velocity[1];
    velocities[base + 2] = velocity[2];
}}

fn get_velocity(index: i32) -> vec3<f32> {{
    let base = index * 3;
    return vec3(
        velocities[base],
        velocities[base + 1],
        velocities[base + 2],
  ); 
}}
",
            g = group
        );
    }

    pub fn add_index_ops_periodic(&mut self) {
        self.buffer += "
fn coord_to_linear(x_raw: i32, y_raw: i32, z_raw: i32) -> i32 {
  var x = x_raw;
  if x_raw >= dimensions.max[0] {
    x = x_raw % dimensions.max[0];
  }
  if x_raw < 0 {
    x = dimensions.max[0] + x_raw;
  }

  var y = y_raw;
  if y_raw >= dimensions.max[1] {
    y = y_raw % dimensions.max[1];
  }
  if y_raw < 0 {
    y = dimensions.max[1] + y_raw;
  }

  var z = z_raw;
  if z_raw >= dimensions.max[2] {
    z = z_raw % dimensions.max[2];
  }
  if z_raw < 0 {
    z = dimensions.max[2] + z_raw;
  }

  return x * (dimensions.max[1] * dimensions.max[2]) + y * dimensions.max[2] + z;
}
";
    }

    // Only reflect incoming velocities
    pub fn add_specular_reflection(&mut self) {
        self.buffer += "
fn specular_reflect(velocity: vec3<f32>, norm: vec3<f32>) -> vec3<f32> {
    let v_dot_n = dot(velocity, norm);
    if v_dot_n < 0.0 {
        return velocity;
    } else {
        return velocity - (2.0 * v_dot_n * norm);
    }
}
";
    }

    fn add_main_invocation_id_block(&mut self, workgroup_size: [u32; 3]) {
        self.buffer += &format!(
            "
@compute
@workgroup_size({}, {}, {})
fn main(@builtin(global_invocation_id) global_invocation_id: vec3<u32>) {{
",
            workgroup_size[0], workgroup_size[1], workgroup_size[2]
        );

        self.buffer += "
  let x = i32(global_invocation_id.x);
  if x > dimensions.max[0] {{
    return;
  }}

  let y = i32(global_invocation_id.y);
  if y > dimensions.max[1] {{
    return;
  }}

  let z = i32(global_invocation_id.z);
  if z > dimensions.max[2] {{
    return;
  }}
";
    }

    // xy face is interior on both axis
    fn add_xy_face_main_invocation_id_block(
        &mut self,
        workgroup_size: [u32; 3],
    ) {
        self.buffer += &format!(
            "
@compute
@workgroup_size({}, {}, {})
fn main(@builtin(global_invocation_id) global_invocation_id: vec3<u32>) {{
",
            workgroup_size[0], workgroup_size[1], workgroup_size[2]
        );

        self.buffer += "
  var x = i32(global_invocation_id.x);
  if x >= face_dimensions.max[0] {{
    return;
  }}

  var y = i32(global_invocation_id.y);
  if y >= face_dimensions.max[1] {{
    return;
  }}
";
    }

    // xy face is interior on both axis
    fn add_interior_main_invocation_id_block(
        &mut self,
        workgroup_size: [u32; 3],
    ) {
        self.buffer += &format!(
            "
@compute
@workgroup_size({}, {}, {})
fn main(@builtin(global_invocation_id) global_invocation_id: vec3<u32>) {{
",
            workgroup_size[0], workgroup_size[1], workgroup_size[2]
        );

        self.buffer += "
  var x = i32(global_invocation_id.x);
  if x >= interior_dimensions.max[0] {{
    return;
  }}
  x += 1;

  var y = i32(global_invocation_id.y);
  if y >= interior_dimensions.max[1] {{
    return;
  }}
  y += 1;

  var z = i32(global_invocation_id.z);
  if z >= interior_dimensions.max[2] {{
    return;
  }}
  z += 1;
";
    }

    // xz gets full face
    fn add_xz_face_main_invocation_id_block(
        &mut self,
        workgroup_size: [u32; 3],
    ) {
        self.buffer += &format!(
            "
@compute
@workgroup_size({}, {}, {})
fn main(@builtin(global_invocation_id) global_invocation_id: vec3<u32>) {{
",
            workgroup_size[0], workgroup_size[1], workgroup_size[2]
        );

        self.buffer += "
  var x = i32(global_invocation_id.x);
  if x >= face_dimensions.max[0] {{
    return;
  }}

  var z = i32(global_invocation_id.z);
  if z >= face_dimensions.max[2] {{
    return;
  }}
";
    }

    fn add_yz_face_main_invocation_id_block(
        &mut self,
        workgroup_size: [u32; 3],
    ) {
        self.buffer += &format!(
            "
@compute
@workgroup_size({}, {}, {})
fn main(@builtin(global_invocation_id) global_invocation_id: vec3<u32>) {{
",
            workgroup_size[0], workgroup_size[1], workgroup_size[2]
        );

        self.buffer += "
  var y = i32(global_invocation_id.y);
  if y >= face_dimensions.max[1] {{
    return;
  }}

  y += 1;
  var z = i32(global_invocation_id.z);
  if z >= face_dimensions.max[2] {{
    return;
  }}
";
    }

    pub fn add_lattice_constants(&mut self) {
        let directions = gen_d3q27_directions();
        self.buffer += "
const DIRECTIONS = array(
";
        for q_i in 0..27 {
            let dir = directions[q_i];
            self.buffer +=
                &format!("  vec3({}.0, {}.0, {}.0),\n", dir[0], dir[1], dir[2]);
        }
        self.buffer += ");";

        let offsets = gen_d3q27_offsets();
        self.buffer += "\n
const OFFSETS = array(
";
        for q_i in 0..27 {
            let offset = offsets[q_i];
            self.buffer += &format!(
                "  vec3({}, {}, {}),\n",
                offset[0], offset[1], offset[2]
            );
        }
        self.buffer += ");";

        self.buffer += "\n
const D3Q27_W = array(
";
        for q_i in 0..27 {
            let w = D3Q27_W[q_i];
            self.buffer += &format!("  {},\n", w);
        }
        self.buffer += ");";

        self.buffer += "\n
const D3Q27_OPP = array(
";
        for q_i in 0..27 {
            let o = D3Q27_OPP[q_i];
            self.buffer += &format!("  {},\n", o);
        }
        self.buffer += ");";
    }

    pub fn add_bounceback_fn(&mut self) {
        self.buffer += "\n
fn apply_bounce_back(index: i32) {
  var new_q: array<f32, 27>;
  let base = index * 27;
";
        for q_i in 0..27 {
            self.buffer += &format!(
                "
  {{
    let q = distributions[base + {qi}];
    let o_i = D3Q27_OPP[{qi}];
    new_q[o_i] = q;
  }}
",
                qi = q_i
            );
        }
        self.buffer += "
 add_qi_to_distributions(index, new_q);
}
";
    }

    pub fn add_equil_fn(&mut self) {
        self.buffer += shader_ops::wgsl_eq();

        self.buffer += "\n
 fn add_qi_to_distributions(index: i32, values: array<f32, 27>) {
    let base = index * 27;
";
        for q_i in 0..27 {
            self.buffer += &format!(
                "  distributions[base + {qi}] = values[{qi}];\n",
                qi = q_i
            );
        }
        self.buffer += "}\n";
    }

    pub fn add_init_main(
        &mut self,
        workgroup_size: [u32; 3],
        stream_figure: bool,
    ) {
        self.add_main_invocation_id_block(workgroup_size);
        self.buffer += "
  var velocity = bc_params.velocity;
";
        if stream_figure {
            self.buffer += "
  if x == 2 &&  y == 2 && z == 2 {
    velocity = vec3(1.0, 2.1, 3.2);
  }
";
        }
        self.buffer += "    
  let result = f_equilibrium(bc_params.density, velocity);
  let index = coord_to_linear(x, y, z);
  add_qi_to_distributions(index, result);
}
";
    }

    pub fn add_dirichlet_main(&mut self, workgroup_size: [u32; 3]) {
        self.add_xy_face_main_invocation_id_block(workgroup_size);
        self.buffer += "
  let result = f_equilibrium(bc_params.density, bc_params.velocity);
  let index_zmin = coord_to_linear(x, y, 0);
  let index_zmax = coord_to_linear(x, y, dimensions.max[2] - 1);
  add_qi_to_distributions(index_zmin, result);
  add_qi_to_distributions(index_zmax, result);
}
";
    }

    pub fn add_xz_bounce_main(&mut self, workgroup_size: [u32; 3]) {
        self.add_xz_face_main_invocation_id_block(workgroup_size);
        self.buffer += "
  let index_ymin = coord_to_linear(x, 0, z);
  apply_bounce_back(index_ymin); 

  let index_ymax = coord_to_linear(x, dimensions.max[1] - 1, z);
  apply_bounce_back(index_ymax);
}
";
    }

    pub fn add_xz_slip_main(&mut self, workgroup_size: [u32; 3]) {
        self.add_xz_face_main_invocation_id_block(workgroup_size);
        self.buffer += "
  let index_ymin = coord_to_linear(x, 0, z);
  let old_vel_ymin = get_velocity(index_ymin);
  let density_ymin = densities[index_ymin];
  let new_ymin_velocity = specular_reflect(old_vel_ymin, vec3(0.0, 1.0, 0.0));
  let result_ymin = f_equilibrium(density_ymin, new_ymin_velocity);
  add_qi_to_distributions(index_ymin, result_ymin);

  let index_ymax = coord_to_linear(x, dimensions.max[1] - 1, z);
  let old_vel_ymax = get_velocity(index_ymax);
  let density_ymax = densities[index_ymax];
  let new_ymax_velocity = specular_reflect(old_vel_ymax, vec3(0.0, -1.0, 0.0));
  let result_ymax = f_equilibrium(density_ymax, new_ymax_velocity);
  add_qi_to_distributions(index_ymax, result_ymax);
}
";
    }

    pub fn add_yz_bounce_main(&mut self, workgroup_size: [u32; 3]) {
        self.add_yz_face_main_invocation_id_block(workgroup_size);
        self.buffer += "
  let index_xmin = coord_to_linear(0, y, z);
  apply_bounce_back(index_xmin);

  let index_xmax = coord_to_linear(dimensions.max[0] - 1, y, z);
  apply_bounce_back(index_xmax);
}
";
    }

    pub fn add_yz_slip_main(&mut self, workgroup_size: [u32; 3]) {
        self.add_yz_face_main_invocation_id_block(workgroup_size);
        self.buffer += "
  let index_xmin = coord_to_linear(0, y, z);
  let old_vel_xmin = get_velocity(index_xmin);
  let density_xmin = densities[index_xmin];
  let new_xmin_velocity = specular_reflect(old_vel_xmin, vec3(1.0, 0.0, 0.0));
  let result_xmin = f_equilibrium(density_xmin, new_xmin_velocity);
  add_qi_to_distributions(index_xmin, result_xmin);

  let index_xmax = coord_to_linear(dimensions.max[0] - 1, y, z);
  let old_vel_xmax = get_velocity(index_xmax);
  let density_xmax = densities[index_xmax];
  let new_xmax_velocity = specular_reflect(old_vel_xmax, vec3(-1.0, 0.0, 0.0));
  let result_xmax = f_equilibrium(density_xmax, new_xmax_velocity);
  add_qi_to_distributions(index_xmax, result_xmax);
}
";
    }

    pub fn add_moments_main(&mut self, workgroup_size: [u32; 3]) {
        self.buffer += shader_ops::wgsl_moments();
        self.add_main_invocation_id_block(workgroup_size);
        self.buffer += "
  let index = coord_to_linear(x, y, z);
  moments(index);
}
";
    }

    pub fn add_stream_main(&mut self, workgroup_size: [u32; 3]) {
        self.add_main_invocation_id_block(workgroup_size);
        self.buffer += "
  let index = coord_to_linear(x, y, z);
  let base = index * 27;
  let coord = vec3(x, y, z);
";
        for q_i in 0..27 {
            self.buffer += &format!(
                "
  {{
    let n_x = x + OFFSETS[{qi}][0];
    let n_y = y + OFFSETS[{qi}][1];
    let n_z = z + OFFSETS[{qi}][2];
    let n_base = coord_to_linear(n_x, n_y, n_z);
    let d_index = n_base * 27 + {qi};
    let q = distributions[base + {qi}];
    distributions_scratch[d_index] = q;
  }}",
                qi = q_i
            );
        }
        self.buffer += "  
}
";
    }

    pub fn add_collision_main(
        &mut self,
        workgroup_size: [u32; 3],
        collision: CollisionType,
    ) {
        match collision {
            CollisionType::BGK(omega) => {
                self.buffer += &shader_ops::wgsl_bgk(omega);
            }
            CollisionType::MRT(riv) => {
                self.buffer += &shader_ops::wgsl_mrt(riv);
            }
            CollisionType::CMMRT(riv) => {
                self.buffer += &shader_ops::wgsl_cm_mrt(riv);
            }
        }
        self.add_main_invocation_id_block(workgroup_size);
        self.buffer += "
  let index = coord_to_linear(x, y, z);
  let base = index * 27;
  let coord = vec3(x, y, z);
  let velocity = get_velocity(index);
  let density = densities[index];

  // Collide
  if bb_flag[index] < 0 {
";

        match collision {
            CollisionType::BGK(_) => {
                self.buffer += "
    bgk(index);
";
            }
            CollisionType::MRT(_) => {
                self.buffer += "
    mrt(index); 
";
            }
            CollisionType::CMMRT(_) => {
                self.buffer += "
    cm_mrt(index);
";
            }
        }

        self.buffer += "
  } 

  // Specular slip
  else {
      apply_bounce_back(index);
      /*
      let n_index = bb_flag[index];
      let normal = get_normal(n_index);
      let new_velocity = specular_reflect(velocity, normal);
      let f_eq = f_equilibrium(density, new_velocity);
      add_qi_to_distributions(index, f_eq);
      */
  }
}
";
    }

    pub fn finish(self, debug_path: &str) -> String {
        let mut output = std::fs::File::create(debug_path).unwrap();
        write!(output, "{}", self.buffer).unwrap();
        self.buffer
    }
}
