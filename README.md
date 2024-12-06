# lbm3

## Code Generation

Code generation has two targets.
We can generate rust functions, which are easier to debug and unit test.
That can be run with
```
cd sym_scripts
python gen_cm_mrt.py 
cp ../example_output/sym_gen/*.rs ../src/op_test
```

To generate the test operations, 

## Examples

### `compute_shader_diagram`

Used to produce the collide compute shader passes diagram.
