@group(0) @binding(0) 
var<storage, read_write> data: array<u32>;

@compute
@workgroup_size(128)
fn main(@builtin(global_invocation_id) global_invocation_id: vec3<u32>) {
  let index = global_invocation_id.x;
  data[index] = index;
}
