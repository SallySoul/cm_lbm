import cm_mrt
import sym_gen_util as util
from sympy.matrices import Matrix
from sympy import Symbol
from sympy.simplify.simplify import simplify

def moments_op_header():
    return '''\
pub fn moments_op(f: [f32; 27]) -> (f32, [f32;3]) {
'''

def moments_op_footer():
    return '''\
    (density, [ux/density, uy/density, uz/density]) 
}\n\n
'''

def gen_moments_ops(rust_src_dir, shader_src_dir, debug_dir):
    print("Generating moments_op")
    name = "moments"

    f = cm_mrt.f()
    moment_op = cm_mrt.M(Matrix([[0.0], [0.0], [0.0]])) * f

    (rust_source_body, debug_raw) = util.rust_generate_moment_op(simplify(moment_op))
    util.write_ops_debug(name, debug_raw, debug_dir)

    rust_source = moments_op_header()
    rust_source += util.rust_fi_def()
    rust_source += rust_source_body
    rust_source += moments_op_footer()
    util.write_rust_ops(name, rust_source, rust_src_dir)
