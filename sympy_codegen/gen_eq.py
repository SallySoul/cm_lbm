import cm_mrt
import sym_gen_util as util
from sympy.matrices import Matrix
from sympy import Symbol
from sympy.simplify.simplify import simplify

def eq_rust_header():
    return '''\
pub fn eq(
    ux: f32,
    uy: f32,
    uz: f32,
    density: f32,
) -> [f32; 27] {
    let mut result = [0.0; 27];
'''

def eq_rust_f64_header():
    return '''\
pub fn eq_f64(
    ux: f64,
    uy: f64,
    uz: f64,
    density: f64,
) -> [f64; 27] {
    let mut result = [0.0; 27];
'''

def eq_rust_footer():
    return '''\
  result
}\n\n
'''

def eq_shader_header():
    return '''\
pub fn wgsl_eq() -> &'static str {
    &"
fn f_equilibrium(density: f32, velocity: vec3<f32>) -> array<f32, 27> {
    let ux = velocity[0];
    let uy = velocity[1];
    let uz = velocity[2];
    var result: array<f32, 27>;
'''

def eq_shader_footer():
    return '''\
    return result;
}
"
}
'''

def gen_eq_ops(rust_src_dir, shader_src_dir, debug_dir):
    print("Generating eq_op")
    name = "eq"

    u = cm_mrt.u()
    density = Symbol("density")

    eq_op = cm_mrt.f_eq(density, u)

    (source_body, debug_raw) = util.rust_generate_op(simplify(eq_op).evalf())
    util.write_ops_debug(name, debug_raw, debug_dir)

    rust_source = eq_rust_header()
    rust_source += source_body
    rust_source += eq_rust_footer()
    util.write_rust_ops(name, rust_source, rust_src_dir)

    f64_rust_source = eq_rust_f64_header()
    f64_rust_source += source_body
    f64_rust_source += eq_rust_footer()
    f64_name = f"{name}_f64"
    util.write_rust_ops(f64_name, f64_rust_source, rust_src_dir)

    shader_source = eq_shader_header()
    shader_source += source_body
    shader_source += eq_shader_footer()
    util.write_rust_ops(name, shader_source, shader_src_dir) 
