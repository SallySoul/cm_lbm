import re

buffer = "(3.0 - uz - 4.0)**2"

output = re.sub(r'(u.|\([a-z+\- \d\.]*\))\*\*2', r'\1 * \1', buffer)

print(output)

