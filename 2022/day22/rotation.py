# ---
# jupyter:
#   jupytext:
#     formats: ipynb,py:light
#     text_representation:
#       extension: .py
#       format_name: light
#       format_version: '1.5'
#       jupytext_version: 1.14.0
#   kernelspec:
#     display_name: Python 3 (ipykernel)
#     language: python
#     name: python3
# ---

import sympy

a, b = sympy.symbols('a b')
A = sympy.Matrix([[a, -b, 0], [b, a, 0], [0, 0, 1]])
A

Rd = sympy.Matrix([[0, 0, 1], [0, 1, 0], [-1, 0,0]])
Rd

A * Rd * A.T


