import cm_mrt
import sym_gen_util as util
from sympy.matrices import Matrix
from sympy import Symbol
from sympy.simplify.simplify import simplify

def eqhigh_rust_header():
    return '''\
pub fn eqhigh(
    ux: f32,
    uy: f32,
    uz: f32,
    density: f32,
) -> [f32; 27] {
    let mut result = [0.0; 27];
'''

def eqhigh_rust_footer():
    return '''\
  result
}\n\n
'''

def eqhigh_shader_header():
    return '''\
pub fn wgsl_eqhigh() -> &'static str {
    &"
fn f_equilibrium(density: f32, velocity: vec3<f32>) -> array<f32, 27> {
    let ux = velocity[0];
    let uy = velocity[1];
    let uz = velocity[2];
    var result: array<f32, 27>;
'''

def eqhigh_shader_footer():
    return '''\
    return result;
}
"
}
'''

def gen_eqhigh_ops(rust_src_dir, shader_src_dir, debug_dir):
    print("Generating eqhigh_op")
    name = "eqhigh"

    u = cm_mrt.u()
    density = Symbol("density")

    m = cm_mrt.M(u)
    m1 = m.inv()
    eqhigh_op = m1 * cm_mrt.MBar(density)

    (source_body, debug_raw) = util.rust_generate_op(simplify(eqhigh_op))
    util.write_ops_debug(name, debug_raw, debug_dir)

    rust_source = eqhigh_rust_header()
    rust_source += source_body
    rust_source += eqhigh_rust_footer()
    util.write_rust_ops(name, rust_source, rust_src_dir)

    shader_source = eqhigh_shader_header()
    shader_source += source_body
    shader_source += eqhigh_shader_footer()
    util.write_rust_ops(name, shader_source, shader_src_dir) 
