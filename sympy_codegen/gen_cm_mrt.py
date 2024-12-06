import cm_mrt
import sym_gen_util as util
from sympy.matrices import Matrix
from sympy import Symbol
from sympy.simplify.simplify import simplify

def cm_mrt_op_header():
    return '''\
pub fn cm_mrt(
    f: [f32; 27],
    ux: f32,
    uy: f32,
    uz: f32,
    density: f32,
    riv: f32,
) -> [f32; 27] {
    let mut result = [0.0; 27];
'''

def cm_mrt_op_footer():
    return '''\
  result
}\n\n
'''

def gen_cm_mrt_ops(rust_src_dir, shader_src_dir, debug_dir):
    print("Generating cm_mrt")
    name = "cm_mrt"

    u = cm_mrt.u()
    f = cm_mrt.f()
    density = Symbol("density")
    riv = Symbol("riv")

    m = cm_mrt.M(u)
    m1 = m.inv()
    r = cm_mrt.R(riv)
    f = cm_mrt.f()
    mbar = cm_mrt.MBar(density)
    eq_op = cm_mrt.f_eq(density, u)
    cm_mrt_op = m1 * r * m * (f - eq_op)

    (rust_source_body, debug_raw) = util.rust_generate_op(cm_mrt_op)
    util.write_ops_debug(name, debug_raw, debug_dir)

    rust_source = cm_mrt_op_header()
    rust_source += util.rust_fi_def()
    rust_source += rust_source_body
    rust_source += cm_mrt_op_footer()
    util.write_rust_ops(name, rust_source, rust_src_dir)
