struct LatticeDimensions{
    rows: i32,
    cols: i32,
    total: i32,
    q: i32,
}

@group(0) @binding(0) var<uniform> dimensions: LatticeDimensions;

@group(1) @binding(0) 
var<storage, read_write> input: array<u32>;

@group(1) @binding(0) 
var<storage, read_write> output: array<u32>;

@compute
@workgroup_size(128)
fn main(@builtin(global_invocation_id) global_invocation_id: vec3<u32>) {
  
  let index = global_invocation_id.x;
  data[index] = index;
}
