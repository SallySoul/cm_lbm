import cm_mrt
import sym_gen_util as util
from sympy.matrices import Matrix
from sympy import Symbol
from sympy.simplify.simplify import simplify

def bgk_rust_header():
    return '''\
pub fn bgk(
    f: [f32; 27],
    ux: f32,
    uy: f32,
    uz: f32,
    density: f32,
    omega: f32,
) -> [f32; 27] {
    let mut result = [0.0; 27];
'''

def bgk_rust_footer():
    return '''\
  result
}\n\n
'''

def gen_bgk_ops(rust_src_dir, shader_src_dir, debug_dir):
    print("Generating BGK")
    name = "bgk"

    u = cm_mrt.u()
    f = cm_mrt.f()
    density = Symbol("density")
    omega = Symbol("omega")
    eq_op = cm_mrt.f_eq(density, u)

    eq_op = cm_mrt.f_eq(density, u)
    bgk_op = f + omega * (f - eq_op)
     
    (rust_source_body, debug_raw) = util.rust_generate_op(bgk_op)
    util.write_ops_debug(name, debug_raw, debug_dir)

    rust_source = bgk_rust_header()
    rust_source += util.rust_fi_def()
    rust_source += rust_source_body
    rust_source += bgk_rust_footer()
    util.write_rust_ops(name, rust_source, rust_src_dir)

