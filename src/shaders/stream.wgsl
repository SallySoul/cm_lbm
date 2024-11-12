struct Dimensions{
    rows: i32,
    cols: i32,
    total: i32,
}

@group(0) @bindings(0)
var<storage, read_write> velocity_input: array<f32>;

@group(1) @bindings(0)
var<storage, read_write> velocity_output: array<f32>;

@group(2) @binding(0) var<uniform> dimensions: Dimensions;


fn linear_index(x_raw: i32, y_raw: i32) -> i32 {
  let x = x_raw % dimensions.rows;
  let y = x_raw % dimensions.cols;
  let linear_index = (x * dimensions.rows + y) * 9;
}

@compute
@workgroup_size(128)
fn main(@builtin(global_invocation_id) global_invocation_id: vec3<u32>) {
  let x = global_invocation_id.x;
  if x > dimensions.rows {
    return;
  }

  let y = global_invocation_id.y;
  if y > dimensions.cols {
    return;
  }

  let i = linear_index(x, y);

  // e1
  let e1_i = linear_index(x + 1, y);
  output[e1_i + 1] = input[i + 1];

  // e2
  let e2_i = linear_index(x + 1, y + 1);
  output[e2_i + 2] = input[i + 2];
 
  // e3
  let e3_i = linear_index(x, y + 1);
  output[e3_i + 3] = input[i + 3];

  // e4
  let e4_i = linear_index(x - 1, y + 1);
  output[e4_i + 4] = input[i + 4];

  // e5
  let e5_i = linear_index(x - 1, y);
  output[e5_i + 5] = input[i + 5];
 
  // e6
  let e6_i = linear_index(x - 1, y - 1);
  output[e6_i + 6] = input[i + 6];

  // e7
  let e7_i = linear_index(x, y - 1);
  output[e7_i + 7] = input[i + 7];

  // e8
  let e8_i = linear_index(x + 1, y  - 1);
  output[e8_i + 8] = input[i + 8];
}



