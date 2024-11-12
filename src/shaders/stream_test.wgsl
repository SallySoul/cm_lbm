struct LatticeDimensions{
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

fn linear_index(x_raw: i32, y_raw: i32) -> i32 {
  let x = x_raw % dimensions.rows;
  let y = x_raw % dimensions.cols;
  let linear_index = (x * dimensions.rows + y);
}

@compute
@workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_invocation_id: vec3<u32>) {
  let x = i32(global_invocation_id.x);
  if x > dimensions.rows {
    return;
  }

  let y = i32(global_invocation_id.y);
  if y > dimensions.cols {
    return;
  }

  let i = linear_index(x, y);
  let q_f = f32(dimensions.q);
  output[i] = input[i] * q_f;
}
