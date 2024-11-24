pub struct PressureShader2DBuilder {
    buffer: String,
}

impl PressureShader2DBuilder {
    pub fn new() -> Self {
        PressureShader2DBuilder {
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

    pub fn add_input_output(&mut self, input_group: u32, output_group: u32) {
        self.buffer += &format!(
            "
@group({}) @binding(0) 
var<storage, read_write> input: array<f32>;
@group({}) @binding(0) 
var<storage, read_write> output: array<f32>;",
            input_group, output_group
        );
    }

    pub fn add_index_ops_periodic(&mut self) {
        self.buffer += "
fn coord_to_linear(x_raw: i32, y_raw: i32) -> i32 {
  var x = x_raw;
  if x_raw >= dimensions.rows {
    x = x_raw % dimensions.rows;
  }
  if x_raw < 0 {
    x = dimensions.rows + x_raw;
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

    pub fn add_main(&mut self, workgroup_size: [u32; 3], first: bool) {
        self.buffer += &format!(
            "
@compute
@workgroup_size({}, {}, {})",
            workgroup_size[0], workgroup_size[1], workgroup_size[2]
        );

        self.buffer += "
fn main(@builtin(global_invocation_id) global_invocation_id: vec3<u32>) {
  let x = i32(global_invocation_id.x);
  if x >= dimensions.rows {{
    return;
  }}

  let y = i32(global_invocation_id.y);
  if y >= dimensions.cols {{
    return;
  }}

  let input_index = coord_to_linear(x, y);
";

        if first {
            self.buffer += "output[input_index] = input[input_index];\n";
        } else {
            self.buffer += "output[input_index] += input[input_index];\n";
        }

        self.buffer += "}";
    }

    pub fn finish(self) -> String {
        self.buffer
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use std::io::*;

    #[test]
    fn pressure_shader_2d_builder_test() {
        let mut b = PressureShader2DBuilder::new();
        b.add_dimensions_uniform(0);
        b.add_input_output(1, 2);
        b.add_index_ops_periodic();
        b.add_main([2, 3, 4], true);
        let result = b.finish();
        assert_eq!(
            result,
            "struct LatticeDimensions {
    rows: i32,
    cols: i32,
    total: i32,
    q: i32,
    size: f32,
}
@group(0) @binding(0) 
var<uniform> dimensions: LatticeDimensions;
@group(1) @binding(0) 
var<storage, read_write> input: array<f32>;
@group(2) @binding(0) 
var<storage, read_write> output: array<f32>;
fn coord_to_linear(x_raw: i32, y_raw: i32) -> i32 {
  var x = x_raw;
  if x_raw >= dimensions.rows {
    x = x_raw % dimensions.rows;
  }
  if x_raw < 0 {
    x = dimensions.rows + x_raw;
  }

  var y = y_raw;
  if y_raw >= dimensions.rows {
    y = y_raw % dimensions.rows;
  }
  if y_raw < 0 {
    y = dimensions.rows + y_raw;
  }

  return x * dimensions.rows + y;
}
@compute
@workgroup_size(2, 3, 4)
fn main(@builtin(global_invocation_id) global_invocation_id: vec3<u32>) {
  let x = i32(global_invocation_id.x);
  if x >= dimensions.rows {{
    return;
  }}

  let y = i32(global_invocation_id.y);
  if y >= dimensions.cols {{
    return;
  }}

  let input_index = coord_to_linear(x, y);
output[input_index] = input[input_index];
}"
        );
    }
}
