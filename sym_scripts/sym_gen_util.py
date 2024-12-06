import re

# I don't have time to learn sympy fully
# But the raw strings include an '**'
# exponent operator.
# I want to replace that with multiplication
def remove_exponent(source_buffer):
    # For now we only see power 2
    return re.sub(r'(u.|\([a-z*+\- \d\.]*\))\*\*2', r'\1 * \1', source_buffer)

def write_results(name, source, raw, output_dir):
    print(f"write result, name: {name}, output_dir: {output_dir}")
    with open(f"{output_dir}/{name}.rs", 'w') as output:
        output.write(source)
    with open(f"{output_dir}/{name}.txt", 'w') as output:
        output.write(raw)

def rust_generate_op(op_expr):
    source_buffer = "" 
    raw_buffer = ""
    i = 0
    for q in op_expr:
        print(f"  rust_generate_op: Starting {i}")
        #s = simplify(q)
        source_buffer += f"result[{i}] = {q};\n\n"
        raw_buffer+= f"i: {i}, q: {q}\n\n"
        i += 1
    source_buffer = remove_exponent(source_buffer)
    return (source_buffer, raw_buffer)

def rust_generate_moment_op(op_expr):
    source_buffer = "" 
    raw_buffer = ""

    print(f"  rust_generate_moment_op: 0")
    source_buffer += f"let density = {op_expr[0]};\n\n"
    raw_buffer+= f"i: 0, q: {op_expr[0]}\n\n"

    print(f"  rust_generate_moment_op: 1")
    source_buffer += f"let ux = {op_expr[1]};\n\n"
    raw_buffer+= f"i: 1, q: {op_expr[1]}\n\n"

    print(f"  rust_generate_moment_op: 2")
    source_buffer += f"let uy = {op_expr[2]};\n\n"
    raw_buffer+= f"i: 2, q: {op_expr[2]}\n\n"

    print(f"  rust_generate_moment_op: 3")
    source_buffer += f"let uz = {op_expr[3]};\n\n"
    raw_buffer+= f"i: 3, q: {op_expr[3]}\n\n"

    source_buffer = remove_exponent(source_buffer)
    return (source_buffer, raw_buffer)


def rust_fi_def():
    return '''\
    let f0 = f[0];
    let f1 = f[1];
    let f2 = f[2];
    let f3 = f[3];
    let f4 = f[4];
    let f5 = f[5];
    let f6 = f[6];
    let f7 = f[7];
    let f8 = f[8];
    let f9 = f[9];
    let f10 = f[10];
    let f11 = f[11];
    let f12 = f[12];
    let f13 = f[13];
    let f14 = f[14];
    let f15 = f[15];
    let f16 = f[16];
    let f17 = f[17];
    let f18 = f[18];
    let f19 = f[19];
    let f20 = f[20];
    let f21 = f[21];
    let f22 = f[22];
    let f23 = f[23];
    let f24 = f[24];
    let f25 = f[25];
    let f26 = f[26];
'''
