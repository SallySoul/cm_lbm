import cm_mrt
import sym_gen_util as util
from sympy.matrices import Matrix
from sympy import Symbol
from sympy.simplify.simplify import simplify

def mrt_rust_header():
    return '''\
pub fn mrt(
    f: [f32; 27],
    ux: f32,
    uy: f32,
    uz: f32,
    density: f32,
    riv: f32,
) -> [f32; 27] {
    let mut result = [0.0; 27];
'''

def mrt_rust_footer():
    return '''\
  result
}\n\n
'''

def mrt_shader_header():
    return '''\
pub fn wgsl_mrt(riv: f32) -> String {
    format!("
fn mrt(index: i32) {{
    let velocity = get_velocity(index);
    let ux = velocity[0];
    let uy = velocity[1];
    let uz = velocity[2];
    let density = densities[index];
    let riv = {};
    var result: array<f32, 27>;
'''

def mrt_shader_footer():
    return '''\
    add_qi_to_distributions(index, result);
}}
", riv)
}
'''

def gen_mrt_ops(rust_src_dir, shader_src_dir, debug_dir):
    print("Generating mrt")
    name = "mrt"

    u = cm_mrt.u()
    f = cm_mrt.f()
    density = Symbol("density")
    riv = Symbol("riv")

    m = cm_mrt.M(Matrix([[0], [0], [0]]))
    m1 = m.inv()
    r = cm_mrt.R(riv)
    f = cm_mrt.f()
    mbar = cm_mrt.MBar(density)
    eq_op = cm_mrt.f_eq(density, u)
    mrt_op = f + m1 * r * m * (eq_op - f)

    (source_body, debug_raw) = util.rust_generate_op(simplify(mrt_op).evalf())
    util.write_ops_debug(name, debug_raw, debug_dir)

    rust_source = mrt_rust_header()
    rust_source += util.rust_fi_def()
    rust_source += source_body
    rust_source += mrt_rust_footer()
    util.write_rust_ops(name, rust_source, rust_src_dir)

    shader_source = mrt_shader_header()
    shader_source += util.shader_fi_def()
    shader_source += source_body
    shader_source += mrt_shader_footer()
    util.write_rust_ops(name, shader_source, shader_src_dir)
