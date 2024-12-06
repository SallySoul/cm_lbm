from gen_bgk import gen_bgk_ops 
from gen_cm_mrt import gen_cm_mrt_ops
from gen_moments import gen_moments_ops 
from gen_mrt import gen_mrt_ops
from gen_eq import gen_eq_ops

def main():
    rust_source_dir = "../cm_lbm_generated/src/rust_ops"
    shader_source_dir = "../cm_lbm_generated/src/shader_ops"
    debug_dir = "../codegen_debug"
    gen_bgk_ops(rust_source_dir, shader_source_dir, debug_dir)
    gen_cm_mrt_ops(rust_source_dir, shader_source_dir, debug_dir)
    gen_mrt_ops(rust_source_dir, shader_source_dir, debug_dir)
    gen_moments_ops(rust_source_dir, shader_source_dir, debug_dir)
    gen_eq_ops(rust_source_dir, shader_source_dir, debug_dir)

main()
