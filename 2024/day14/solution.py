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
import math
import collections
import operator
import functools

# %%
ifile, bounds = [["input.txt",(101,103)], ["test00.txt",(11,7)]][0]
with open(ifile) as f:
    ipv0s = [[int(v) for v in re.findall(r"-?\d+", ln)] for ln in  f.read().strip().split("\n")]
pv0s = [(y+1j*x, vy+1j*vx) for x,y,vx,vy in ipv0s]

# %%
half_bounds = [(b-1)//2 for b in bounds]
def sign(v):
    return 0 if v==0 else (1 if v>0 else -1)

ct = collections.defaultdict(lambda: 0)
for p0,v in pv0s:
    p = p0+100*v
    pxy = [a%b for a,b in zip([p.imag, p.real], bounds)]
    pxy = [a-s for a,s in zip(pxy, half_bounds)]
    sgn = tuple([sign(a) for a in pxy])
    ct[sgn]+=1

kv=[n for (x,y),n in ct.items() if x !=0 and y!=0]
print(functools.reduce(operator.mul, kv))


# %%
def prop(n):
    return [
        [int((pi+n*vi)%bi-((bi-1)//2)) for pi,vi,bi in zip([p.imag, p.real],[pv.imag, pv.real], bounds)]
        for p,pv in pv0s]

pxy100 = prop(100)
ct = collections.defaultdict(lambda: 0)
for pxy in pxy100:
    sgn = tuple([sign(a) for a in pxy])
    ct[sgn]+=1
kv=[n for (x,y),n in ct.items() if x !=0 and y!=0]
functools.reduce(operator.mul, kv)

# %%
hx = (bounds[0]-1)//2
hy = (bounds[1]-1)//2
fy = bounds[1]
xmas_allowed = set((x,y-hy) for xmax, y in ((math.ceil(2+hx*y/fy), y) for y in range(fy)) for x in range(xmax))

n=0
while True:
    n+=1
    pop = set(tuple(int((pi+n*vi)%bi-((bi-1)//2)) for pi,vi,bi in zip([p.imag, p.real],[pv.imag, pv.real], bounds))
        for p,pv in pv0s)
    if pop.issubset(xmas_allowed):
        print(f"candiate at: {n}")
    elif len(pop.intersection(xmas_allowed))==0:
        print(f"No overlap for n={n}")



# %%
def check_sym(n):
    quads = collections.defaultdict(set)
    pxys = prop(n)
    left = set((abs(px), py) for px,py in pxys if px<0)
    right = set((abs(px), py) for px,py in pxys if px>0)
    #print( next(v for v in left), next(v for v in right))
    return left==right

check_sym(2)

# %%
n = 0
while True:
    n+=1
    if n%10000 ==0:
        print(n)
    if check_sym(n):
        print("Solution at", n)
        break


# %% [markdown]
# ## Part 2 restart

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
# check part 1 replication
def safety(pts):
    ct = collections.defaultdict(lambda: 0)
    for pxy in pts:
        sgn = tuple([sign(a) for a in pxy])
        ct[sgn]+=1
    kv=[n for (x,y),n in ct.items() if x !=0 and y!=0]
    return functools.reduce(operator.mul, kv)

print(safety(prop(100)))


# %%
# try categorizing by min assym
by_asym = sorted((abs(sum(x for x,y in prop(n))), n) for n in range(bounds[0]*bounds[1]))
by_asym[:10]

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
    

# %%
