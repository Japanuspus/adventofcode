# ---
# jupyter:
#   jupytext:
#     formats: ipynb,py:percent
#     text_representation:
#       extension: .py
#       format_name: percent
#       format_version: '1.3'
#       jupytext_version: 1.16.1
#   kernelspec:
#     display_name: Python 3 (ipykernel)
#     language: python
#     name: python3
# ---

# %%
import re
import numpy as np
import itertools
import operator

# %%
with open("input.txt") as f:
    eqs = [(a,b) for a,*b in ([int(v) for v in re.split(":? ", ln)] for ln in f.read().strip().split("\n"))]


# %%
def check_eq(eq, ops = (operator.add, operator.mul)):
    res, ns = eq
    for os in itertools.product(ops, repeat=len(ns)-1):
        v=ns[0]
        for b,op in zip(ns[1:], os):
            v=op(v,b)
        if v==res:
            return True
    return False


# %%
sum(res for res, _ in filter(check_eq, eqs))


# %%
def op_concat(a,b):
    return a*10**(len(str(b))) + b

op_concat(11, 123)

# %%
sum(res for res, ns in eqs if check_eq((res, ns), ops=(operator.add, operator.mul, op_concat)))

# %%
