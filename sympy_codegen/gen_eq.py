import cm_mrt
import sym_gen_util as util
from sympy.matrices import Matrix
from sympy import Symbol
from sympy.simplify.simplify import simplify

def eq_rust_header():
    return '''\
pub fn eq_op(
    ux: f32,
    uy: f32,
    uz: f32,
    density: f32,
) -> [f32; 27] {
    let mut result = [0.0; 27];
'''

def eq_rust_footer():
    return '''\
  result
}\n\n
'''

def gen_eq_ops(rust_src_dir, shader_src_dir, debug_dir):
    print("Generating eq_op")
    name = "eq"

    u = cm_mrt.u()
    density = Symbol("density")

    eq_op = cm_mrt.f_eq(density, u)

    (rust_source_body, debug_raw) = util.rust_generate_op(simplify(eq_op))
    util.write_ops_debug(name, debug_raw, debug_dir)

    rust_source = eq_rust_header()
    rust_source += rust_source_body
    rust_source += eq_rust_footer()
    util.write_rust_ops(name, rust_source, rust_src_dir)
