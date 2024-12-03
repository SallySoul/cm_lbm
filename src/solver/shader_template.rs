use crate::*;
use std::io::prelude::*;

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

    pub fn add_distributions(&mut self, distributions_group: u32) {
        self.buffer += &format!(
            "
@group({dg}) @binding(0) 
var<storage, read_write> distributions: array<f32>;
",
            dg = distributions_group,
        );
    }

    pub fn add_moments_bindgroup(&mut self, group: u32) {
        self.buffer += &format!(
            "
@group({g}) @binding(0) 
var<storage, read_write> densities: array<f32>;
@group({g}) @binding(1) 
var<storage, read_write> velocities: array<vec3<f32>>;
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
}";
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
    }

    pub fn add_equil_fn(&mut self) {
        self.buffer += "\n
fn f_equilibrium(density: f32, velocity: vec3<f32>) -> array<f32, 27> {
    var result: array<f32, 27>;
";
        for q_i in 0..27 {
            self.buffer += &format!(
                "
  {{
     // Calculate equilibrium {qi}
     let dir = DIRECTIONS[{qi}];
     let dir_u = dot(dir, velocity);
     let w_i = D3Q27_W[{qi}];

     let t1 = 3.0 * dir_u;
     let t2 = 9.0 * dir_u * dir_u;
     let t3 = -(3.0 * dot(velocity, velocity));

     result[{qi}] = w_i * density * (1.0 + t1 + t2 + t3);
  }}
",
                qi = q_i
            );
        }
        self.buffer += "
  return result;
}\n";

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

    pub fn add_init_main(&mut self, workgroup_size: [u32; 3]) {
        self.add_main_invocation_id_block(workgroup_size);

        self.buffer += "
  let result = f_equilibrium(bc_params.density, bc_params.velocity);
  let index = coord_to_linear(x, y, z);
  add_qi_to_distributions(index, result);
  densities[index] = bc_params.density;
  velocities[index] = bc_params.velocity;
";

        self.buffer += "}";
    }

    pub fn add_moments_main(&mut self, workgroup_size: [u32; 3]) {
        self.add_main_invocation_id_block(workgroup_size);
        self.buffer += "
  let index = coord_to_linear(x, y, z);
  let base = index * 27;
  var density = 0.0;
  var velocity = vec3(0.0, 0.0, 0.0);\n
";
        for q_i in 0..27 {
            self.buffer += &format!(
                "  density += distributions[base + {qi}];\n",
                qi = q_i
            );
            self.buffer += &format!(
                "  velocity += DIRECTIONS[{qi}] * distributions[base + {qi}];\n",
                qi = q_i
            );
        }

        self.buffer += "  
  velocity /= density;
  densities[index] = density;
  velocities[index] = velocity;
}
";
    }

    pub fn finish(self, debug_path: &str) -> String {
        let mut output = std::fs::File::create(debug_path).unwrap();
        write!(output, "{}", self.buffer).unwrap();
        self.buffer
    }
}
