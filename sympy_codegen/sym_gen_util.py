import re

# I don't have time to learn sympy fully
# But the raw strings include an '**'
# exponent operator.
# I want to replace that with multiplication
def remove_exponent(source_buffer):
    # For now we only see power 2
    return re.sub(r'(u.|\([a-z*+\- \d\.]*\))\*\*2', r'\1 * \1', source_buffer)

def write_rust_ops(name, source, output_dir):
    print(f"write rust_op, name: {name}, output_dir: {output_dir}")
    with open(f"{output_dir}/{name}.rs", 'w') as output:
        output.write(source)

def write_ops_debug(name, debug, output_dir):
    with open(f"{output_dir}/{name}.txt", 'w') as output:
        output.write(debug)

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
    let q0 = f[0];
    let q1 = f[1];
    let q2 = f[2];
    let q3 = f[3];
    let q4 = f[4];
    let q5 = f[5];
    let q6 = f[6];
    let q7 = f[7];
    let q8 = f[8];
    let q9 = f[9];
    let q10 = f[10];
    let q11 = f[11];
    let q12 = f[12];
    let q13 = f[13];
    let q14 = f[14];
    let q15 = f[15];
    let q16 = f[16];
    let q17 = f[17];
    let q18 = f[18];
    let q19 = f[19];
    let q20 = f[20];
    let q21 = f[21];
    let q22 = f[22];
    let q23 = f[23];
    let q24 = f[24];
    let q25 = f[25];
    let q26 = f[26];
'''

def shader_fi_def():
    return '''\
    let base = index * 27; 
    let q0 = distributions[base + 0];
    let q1 = distributions[base + 1];
    let q2 = distributions[base + 2];
    let q3 = distributions[base + 3];
    let q4 = distributions[base + 4];
    let q5 = distributions[base + 5];
    let q6 = distributions[base + 6];
    let q7 = distributions[base + 7];
    let q8 = distributions[base + 8];
    let q9 = distributions[base + 9];
    let q10 = distributions[base + 10];
    let q11 = distributions[base + 11];
    let q12 = distributions[base + 12];
    let q13 = distributions[base + 13];
    let q14 = distributions[base + 14];
    let q15 = distributions[base + 15];
    let q16 = distributions[base + 16];
    let q17 = distributions[base + 17];
    let q18 = distributions[base + 18];
    let q19 = distributions[base + 19];
    let q20 = distributions[base + 20];
    let q21 = distributions[base + 21];
    let q22 = distributions[base + 22];
    let q23 = distributions[base + 23];
    let q24 = distributions[base + 24];
    let q25 = distributions[base + 25];
    let q26 = distributions[base + 26];
'''
