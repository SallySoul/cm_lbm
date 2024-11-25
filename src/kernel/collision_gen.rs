pub struct Collision2DBuilder {
    buffer: String,
}

impl Collision2DBuilder {
    pub fn new() -> Self {
        Collision2DBuilder {
            buffer: String::new(),
        }
    }

    pub fn add_dimensions_uniform(&mut self, group: u32) {
        self.buffer += &format!(
            "struct LatticeDimensions {{
    rows: i32,
    cols: i32,
    total: i32,
    q: i32,
    size: f32,
}}
@group({}) @binding(0) 
var<uniform> dimensions: LatticeDimensions;",
            group
        );
    }

    pub fn add_collision_params_uniform(&mut self, group: u32) {
        self.buffer += &format!(
            "struct CollisionParams {{
    c_s: f32,
    delta_t: f32,
    tau: f32,
}}
@group({}) @binding(0) 
var<uniform> params: CollisionParams;",
            group
        );
    }

    pub fn add_input_output(
        &mut self,
        densities_group: u32,
        pressure_group: u32,
        output_group: u32,
    ) {
        self.buffer += &format!(
            "
@group({}) @binding(0) 
var<storage, read_write> densities: array<f32>;
@group({}) @binding(0) 
var<storage, read_write> pressure: array<f32>;
@group({}) @binding(0) 
var<storage, read_write> ux: array<f32>;
@group({}) @binding(1) 
var<storage, read_write> uy: array<f32>;
",
            densities_group, pressure_group, output_group, output_group
        );
    }

    pub fn add_index_ops_periodic(&mut self) {
        self.buffer += "
fn coord_to_linear(x_raw: i32, y_raw: i32) -> i32 {
  var x = x_raw;
  if x_raw >= dimensions.cols{
    x = x_raw % dimensions.cols;
  }
  if x_raw < 0 {
    x = dimensions.cols + x_raw;
  }

  var y = y_raw;
  if y_raw >= dimensions.rows {
    y = y_raw % dimensions.rows;
  }
  if y_raw < 0 {
    y = dimensions.rows + y_raw;
  }

  return x * dimensions.rows + y;
}";
    }

    pub fn add_main(&mut self, workgroup_size: [u32; 3], wi: f32, dir: [i32; 2], first: bool) {
        self.buffer += &format!(
            "
@compute
@workgroup_size({}, {}, {})",
            workgroup_size[0], workgroup_size[1], workgroup_size[2]
        );

    // page 64
        self.buffer += &format!(
            "
fn main(@builtin(global_invocation_id) global_invocation_id: vec3<u32>) {{
  let x = i32(global_invocation_id.x);
  if x > dimensions.cols {{
    return;
  }}

  let y = i32(global_invocation_id.y);
  if y > dimensions.rows {{
    return;
  }}

  let input_index = coord_to_linear(x, y);
  let p = pressure[input_index];
  let u_x = ux[input_index] / p;
  let u_y = uy[input_index] / p;
   
  let cs_2 = params.cs * params.cs;
  let c_dot_u = u_x * {c_x} + u_y * {c_y};
  let t1 = c_dot_u / cs_2;
  let t2 = (c_dot_u * c_dot_u) / (cs_2 * cs_2);
  let t3 = - (u_x * u_x + u_y * u_y) / (2.0 * cs_2);
  let eq = {w_i} * p * (1 + t1 + t2 + t3);

  let d_i = densities[input_index];
  let new_d_i = params.delta_t * (eq - d_i) / params.tau
  densities[input_index] = new_d_i;
",
            w_i = wi, c_x = dir[0], c_y = dir[1]
        );
        self.buffer += "}";
    }

    pub fn finish(self) -> String {
        self.buffer
    }
}

/*
#[cfg(test)]
mod unit_tests {
    use super::*;
    use std::io::*;

    #[test]
    fn collision_shader_2d_builder_test() {
        let mut b = Collision2DBuilder::new();
        b.add_dimensions_uniform(0);
        b.add_collision_params_uniform(1)
        b.add_input_output(2, 3, 4);
        b.add_index_ops_periodic();
        b.add_main([2, 3, 4], 1.0 / 9.0, [-1, 1], false);
        let result = b.finish();
    }
}
*/
