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
import collections
import operator
import functools

# %%
ifile, bounds = [["input.txt",(101,103)], ["test00.txt",(11,7)]][0]
half_bounds = [(b-1)//2 for b in bounds]
with open(ifile) as f:
    ipv0s = [[int(v) for v in re.findall(r"-?\d+", ln)] for ln in  f.read().strip().split("\n")]
pv0s = [((x,y), (vx,vy)) for x,y,vx,vy in ipv0s]

# %%
half_bounds = [(b-1)//2 for b in bounds]
def sign(v):
    return 0 if v==0 else (1 if v>0 else -1)
def prop(n):
    return [tuple([(pi+n*vi)%bi-hbi for pi,vi,hbi,bi in zip(p, v, half_bounds, bounds)]) for p,v in pv0s]


# %%
def safety(pts):
    ct = collections.defaultdict(lambda: 0)
    for pxy in pts:
        sgn = tuple([sign(a) for a in pxy])
        ct[sgn]+=1
    kv=[n for (x,y),n in ct.items() if x !=0 and y!=0]
    return functools.reduce(operator.mul, kv)

print(safety(prop(100)))

# %%
by_safety = sorted((safety(prop(n)), n) for n in range(bounds[0]*bounds[1]))
by_safety[:10]


# %%
def show(n):
    ps = set(prop(n))
    print(f">>> {n}")
    print('\n'.join(''.join('X' if (x,y) in ps else '.' for x in range(-half_bounds[0], half_bounds[0]+1)) 
                    for y in range(-half_bounds[1], half_bounds[1]+1)))

show(by_safety[0][1])
    
