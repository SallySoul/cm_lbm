import re

buffer = "(-1.0*ux - 1.0*uy)**2**2"

output = re.sub(r'(u.|\([a-z*+\- \d\.]*\))\*\*2', r'\1 * \1', buffer)

print(output)

