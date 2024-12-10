![PurpleTurbulence](render.png)

# `cm_lbm`: Introduction

This project implements a lattice Boltzmann fluid solver
with the central moments multiple relaxation time collision operator (CM-MRT).
It was my final project for computer graphics (CSE528).
The core applications is written in rust and targets WGPU compute shaders.
It generates VTK files that can be post-processed in Paraview.
In addition, python is used for code-generation.

## Contents

### `sympy_codegen`

This directory contains a python script `codegen.py` and supporting modules 
to generate shader code for several kernels needed by LBM.
The operations are represented symbolically using SymPy.
In particular, checkout the `sympy_codegen/gen_*.py`, there's one for each operation.

### `cm_lbm_generated`

This is a rust crate that strictly contains the generated operators.
There are two to three versions of each operator.
In `cm_lbm_generated/src/shader_ops` we have rust functions
that contains a WGSL function as a string which will get templated into a shader at runtime.
`cm_lbm_generates/src/rust_ops` contains the operators single and some-times double precision
rust functions.

### `cm_lbm`

This contains the actual runtime library needed to run a simulation.
`cm_lbm/examples` that contains several applications that 
hardcode different simulations and generate the figures for my report.
`cm_lbm/examples/example_1.rs` would be a good place to start exploring the code base.
`cm_lbm/src/solver/solver_state.rs` defines the solver state and its various operations.

## Commands

### Code Generation

The generated functions are checked into git, but to re-generated them run the following:
```
python sympy_codegen/codegen.py
```

### Run Example 1

The header image was generated with `example_1.rs`. 
This can be run with
```
cargo run --release --example example_1 -- --output-dir <dir-for-output-files>
```
Note that this example will generate over 300gb of data.
