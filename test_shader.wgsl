struct LatticeDimensions {
    rows: i32,
    cols: i32,
    total: i32,
    q: i32,
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
    x = dimensions.rows - x_raw;
  }

  var y = y_raw;
  if y_raw >= dimensions.rows {
    y = y_raw % dimensions.rows;
  }
  if y_raw < 0 {
    y = dimensions.rows - y_raw;
  }

  return x * dimensions.rows + y;
}
@compute
@workgroup_size(2, 3, 4)
fn main(@builtin(global_invocation_id) global_invocation_id: vec3<u32>) {
  let x = i32(global_invocation_id.x);
  if x >= dimensions.rows {
    return;
  }

  let y = i32(global_invocation_id.y);
  if y >= dimensions.cols {
    return;
  }

  let output_index = coord_to_linear(x, y);
  let input_index = coord_to_linear(x + 0, y + -1);
  output[output_index] = input[input_index];
}