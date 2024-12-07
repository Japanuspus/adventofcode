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
import itertools
import operator
import math

# %%
with open("input.txt") as f:
    eqs = [(a,b) for a,*b in ([int(v) for v in re.split(":? ", ln)] for ln in f.read().strip().split("\n"))]


# %%
def check_eq(eq, ops):
    res, ns = eq
    for os in itertools.product(ops, repeat=len(ns)-1):
        v=ns[0]
        for b,op in zip(ns[1:], os):
            v=op(v,b)
        if v==res:
            return True
    return False


# %%
# %%time
ops = (operator.add, operator.mul)
sum(res for res, ns in eqs if check_eq((res, ns), ops=ops))


# %%
def op_cc(a,b):
    return a*10**len(str(b))+b

def op_ccm(a,b):
    return b+a*10**math.ceil(math.log10(b+1))
# %%
# %%time
ops = (operator.add, operator.mul, op_cc)
sum(res for res, ns in eqs if check_eq((res, ns), ops=ops))


# %% [markdown]
# * * *
# Faster solution with branch and bound:

# %%
def check_rec(res, val, rem, ops) -> bool:
    if len(rem)==0:
        return val==res
    if val>res:
        # all operations increase or maintain value - cut if we are past result
        return False
    return any(check_rec(res, op(val, rem[0]), rem[1:], ops) for op in ops)

def check_eq_rec(eq, ops) -> bool:
    res, ns = eq
    return check_rec(res, val=ns[0], rem=ns[1:], ops=ops)


# %%
ops1 = (operator.add, operator.mul, op_cc)
ops2 = (operator.add, operator.mul, op_ccm)
for i,eq in enumerate(eqs):
    if not check_eq_rec(eq, ops1) == check_eq_rec(eq, ops2):
        print((i, eq))

# %%
# %%time
ops = (operator.add, operator.mul)
sum(res for res, ns in eqs if check_rec(res, val=ns[0], rem=ns[1:], ops=ops))

# %%
# %%time
ops = (operator.add, operator.mul, op_cc)
sum(res for res, ns in eqs if check_rec(res, val=ns[0], rem=ns[1:], ops=ops))
