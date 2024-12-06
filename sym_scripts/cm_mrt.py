from sympy.matrices import Matrix
from sympy import Symbol

# The lattice directions
c = [
  Matrix([[0.0], [0.0], [0.0]]),
  Matrix([[1.0], [0.0], [0.0]]),
  Matrix([[-1.0], [0.0], [0.0]]),
  Matrix([[0.0], [1.0], [0.0]]),
  Matrix([[0.0], [-1.0], [0.0]]),
  Matrix([[0.0], [0.0], [1.0]]),
  Matrix([[0.0], [0.0], [-1.0]]),
  Matrix([[1.0], [1.0], [0.0]]),
  Matrix([[1.0], [-1.0], [0.0]]),
  Matrix([[-1.0], [1.0], [0.0]]),
  Matrix([[-1.0], [-1.0], [0.0]]),
  Matrix([[1.0], [0.0], [1.0]]),
  Matrix([[1.0], [0.0], [-1.0]]),
  Matrix([[-1.0], [0.0], [1.0]]),
  Matrix([[-1.0], [0.0], [-1.0]]),
  Matrix([[0.0], [1.0], [1.0]]),
  Matrix([[0.0], [1.0], [-1.0]]),
  Matrix([[0.0], [-1.0], [1.0]]),
  Matrix([[0.0], [-1.0], [-1.0]]),
  Matrix([[1.0], [1.0], [1.0]]),
  Matrix([[1.0], [1.0], [-1.0]]),
  Matrix([[1.0], [-1.0], [1.0]]),
  Matrix([[-1.0], [1.0], [1.0]]),
  Matrix([[1.0], [-1.0], [-1.0]]),
  Matrix([[-1.0], [-1.0], [1.0]]),
  Matrix([[-1.0], [1.0], [-1.0]]),
  Matrix([[-1.0], [-1.0], [-1.0]]),
]

# Moments
# Eqn 35 from Li 2020 Supplemental
# Eqn 9 from De2017 
# This block was regexed into latex for report
# and code below
"""
0  1
1  cx
2  cy
3  cz
4  cx * cy
5  cx * cz
6  cy * cz
7  cx**2 - cy**2
8  cx**2 - cz**2
9  cx**2 + cy**2 + cz**2
10 cx * cy**2 + cx * cz**2
11 cx**2 * cy + cy * cz**2
12 cx**2 * cz + cy**2 * cz
13 cx * cy**2 - cx * cz**2
14 cx**2 * cy - cy * cz**2
15 cx**2 * cz - cy**2 * cz
16 cx * cy * cz
17 cx**2 * cy**2 + cx**2 * cz**2 + cy**2 * cz**2
18 cx**2 * cy**2 + cx**2 cz**2 - cy**2 * cz**2
19 cx**2 * cy**2 - cx**2 * cz**2
20 cx**2 * cy * cz
21 cx * cy**2 * cz
22 cx * cy * cz**2
23 cx * cy**2 * cz**2
24 cx**2 * cy * cz**2
25 cx**2 * cy**2 * cz
26 cx**2 * cy**2 * cz**2
"""

def m0(j, u):
  return 1
def m1(j, u):
  return (c[j] - u)[0]
def m2(j, u):
  return (c[j] - u)[1]
def m3(j, u):
  return (c[j] - u)[2]
def m4(j, u):
  return (c[j] - u)[0] * (c[j] - u)[1]
def m5(j, u):
  return (c[j] - u)[0] * (c[j] - u)[2]
def m6(j, u):
  return (c[j] - u)[1] * (c[j] - u)[2]
def m7(j, u):
  return (c[j] - u)[0] * (c[j] - u)[0] - (c[j] - u)[1] * (c[j] - u)[1]
def m8(j, u):
  return (c[j] - u)[0] * (c[j] - u)[0] - (c[j] - u)[2] * (c[j] - u)[2]
def m9(j, u):
  return (c[j] - u)[0] * (c[j] - u)[0] + (c[j] - u)[1] * (c[j] - u)[1] + (c[j] - u)[2] * (c[j] - u)[2]
def m10(j, u):
  return (c[j] - u)[0] * (c[j] - u)[1] * (c[j] - u)[1] + (c[j] - u)[0] * (c[j] - u)[2] * (c[j] - u)[2]
def m11(j, u):
  return (c[j] - u)[0] * (c[j] - u)[0] * (c[j] - u)[1] + (c[j] - u)[1] * (c[j] - u)[2] * (c[j] - u)[2]
def m12(j, u):
  return (c[j] - u)[0] * (c[j] - u)[0] * (c[j] - u)[2] + (c[j] - u)[1] * (c[j] - u)[1] * (c[j] - u)[2]
def m13(j, u):
  return (c[j] - u)[0] * (c[j] - u)[1] * (c[j] - u)[1] - (c[j] - u)[0] * (c[j] - u)[2] * (c[j] - u)[2]
def m14(j, u):
  return (c[j] - u)[0] * (c[j] - u)[0] * (c[j] - u)[1] - (c[j] - u)[1] * (c[j] - u)[2] * (c[j] - u)[2]
def m15(j, u):
  return (c[j] - u)[0] * (c[j] - u)[0] * (c[j] - u)[2] - (c[j] - u)[1] * (c[j] - u)[1] * (c[j] - u)[2]
def m16(j, u):
  return (c[j] - u)[0] * (c[j] - u)[1] * (c[j] - u)[2]
def m17(j, u):
  return (c[j] - u)[0] * (c[j] - u)[0] * (c[j] - u)[1] * (c[j] - u)[1] + (c[j] - u)[0] * (c[j] - u)[0] * (c[j] - u)[2] * (c[j] - u)[2] + (c[j] - u)[1] * (c[j] - u)[1] * (c[j] - u)[2] * (c[j] - u)[2]
def m18(j, u):
  return (c[j] - u)[0] * (c[j] - u)[0] * (c[j] - u)[1] * (c[j] - u)[1] + (c[j] - u)[0] * (c[j] - u)[0] * (c[j] - u)[2] * (c[j] - u)[2] - (c[j] - u)[1] * (c[j] - u)[1] * (c[j] - u)[2] * (c[j] - u)[2]
def m19(j, u):
  return (c[j] - u)[0] * (c[j] - u)[0] * (c[j] - u)[1] * (c[j] - u)[1] - (c[j] - u)[0] * (c[j] - u)[0] * (c[j] - u)[2] * (c[j] - u)[2]
def m20(j, u):
  return (c[j] - u)[0] * (c[j] - u)[0] * (c[j] - u)[1] * (c[j] - u)[2]
def m21(j, u):
  return (c[j] - u)[0] * (c[j] - u)[1] * (c[j] - u)[1] * (c[j] - u)[2]
def m22(j, u):
  return (c[j] - u)[0] * (c[j] - u)[1] * (c[j] - u)[2] * (c[j] - u)[2]
def m23(j, u):
  return (c[j] - u)[0] * (c[j] - u)[1] * (c[j] - u)[1] * (c[j] - u)[2] * (c[j] - u)[2]
def m24(j, u):
  return (c[j] - u)[0] * (c[j] - u)[0] * (c[j] - u)[1] * (c[j] - u)[2] * (c[j] - u)[2]
def m25(j, u):
  return (c[j] - u)[0] * (c[j] - u)[0] * (c[j] - u)[1] * (c[j] - u)[1] * (c[j] - u)[2]
def m26(j, u):
  return (c[j] - u)[0] * (c[j] - u)[0] * (c[j] - u)[1] * (c[j] - u)[1] * (c[j] - u)[2] * (c[j] - u)[2]

# Used to construct M(u)
def row(mi, u):
    return [mi(j, u) for j in range(0, 27)]

# Create central moment space transform
def M(u):
    return Matrix([row(m0, u),row(m1, u),row(m2, u),row(m3, u),row(m4, u),row(m5, u),row(m6, u),row(m7, u),row(m8, u),row(m9, u),row(m10, u),row(m11, u),row(m12, u),row(m13, u),row(m14, u),row(m15, u),row(m16, u),row(m17, u),row(m18, u),row(m19, u),row(m20, u),row(m21, u),row(m22, u),row(m23, u),row(m24, u),row(m25, u),row(m26, u)])

# Derive relaxation rate from kinematic viscosity
def moment_relaxation_rate(v):
    return 1.0 / (3 * v + 0.5)

# Precompute value for r4 - r9, pass in as riv
# r9 is unclear, could be in either set from what I can find?
def relaxation_rates(riv):
    r0r8 = [0.0, 2.0, 2.0, 2.0, riv, riv, riv, riv, riv]
    r9r16 = [moment_relaxation_rate(0.005) for i in range(0,8)]
    r17r22 = [moment_relaxation_rate(0.007) for i in range(0, 6)]
    r23r25 = [moment_relaxation_rate(0.009) for i in range(0,3)]
    r26 = [moment_relaxation_rate(0.01)]
    return [*r0r8, *r9r16, *r17r22, *r23r25, *r26]


# Diagonal relaxation rates matrix
def R(riv):
    return Matrix.diag(relaxation_rates(riv))

# Equilibrium moments
def MBar(density):
    r = [0 for i in range(0, 27)]
    r[0] = density
    r[9] = density
    r[17] = 3.0 * density
    r[18] = 9.0 * density
    r[26] = 27.0 * density
    return Matrix([[i] for i in r])

# Utility to create vector of component symbols for f
def f():
    return Matrix([[Symbol(f"f{i}")] for i in range(0, 27)])

# Utility to create vector of component symbols for u
def u():
    ux = Symbol("ux")
    uy = Symbol("uy")
    uz = Symbol("uz")
    return Matrix([[ux], [uy], [uz]])

