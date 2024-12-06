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

def eq_op_header():
    return '''\
pub fn eq_op(
    ux: f32,
    uy: f32,
    uz: f32,
    density: f32,
) -> [f32; 27] {
    let mut result = [0.0; 27];
'''

def eq_op_footer():
    return '''\
  result
}\n\n
'''

def moments_op_header():
    return '''\
pub fn moments_op(f: [f32; 27]) -> (f32, [f32;3]) {
'''

def moments_op_footer():
    return '''\
    (density, [ux/density, uy/density, uz/density]) 
}\n\n
'''

def gen_ops(output_dir):
    # These variables will be defined in the function 
    # header
    u = cm_mrt.u()
    f = cm_mrt.f()
    density = Symbol("density")
    riv = Symbol("riv")

    m = cm_mrt.M(u)
    print(f"gen_ops: M shape: {m.shape}")

    m1 = m.inv()
    print(f"gen_ops: M1 shape: {m1.shape}")

    r = cm_mrt.R(riv)
    print(f"gen_ops: R shape: {r.shape}")

    f = cm_mrt.f()
    print(f"gen_ops: f shape: {f.shape}")

    mbar = cm_mrt.MBar(density)
    print(f"gen_ops: M Bar shape: {mbar.shape}")

    moment_op = cm_mrt.M(Matrix([[0.0], [0.0], [0.0]])) * f
    print(f"gen_ops: moment_op shape: {moment_op.shape}")

    eq_op = cm_mrt.f_eq(density, u)
    print(f"gen_ops: eq_op shape: {eq_op.shape}")

    cm_mrt_op = m1 * r * m * (f - eq_op)
    print(f"gen_ops: cm_mrt_op shape: {cm_mrt_op.shape}")

    print("Generating cm_mrt")
    (cm_mrt_source_body, cm_mrt_raw) = util.rust_generate_op(cm_mrt_op)
    cm_mrt_source = cm_mrt_op_header()
    cm_mrt_source += util.rust_fi_def()
    cm_mrt_source += cm_mrt_source_body
    cm_mrt_source += cm_mrt_op_footer()
    util.write_results("cm_mrt", cm_mrt_source, cm_mrt_raw, output_dir)

    print("Generating eq_op")
    (eq_body, eq_raw) = util.rust_generate_op(simplify(eq_op))
    eq_source = eq_op_header()
    eq_source += eq_body
    eq_source += eq_op_footer()
    util.write_results("eq_op", eq_source, eq_raw, output_dir)

    print("Generating moments_op")
    (moments_body, moments_raw) = util.rust_generate_moment_op(simplify(moment_op))
    moments_source = moments_op_header()
    moments_source += util.rust_fi_def()
    moments_source += moments_body
    moments_source += moments_op_footer()
    util.write_results("moments_op", moments_source, moments_raw, output_dir)

def main():
    gen_ops("../example_output/sym_gen")

main()
