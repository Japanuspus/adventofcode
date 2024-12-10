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

# %%
with open(["input.txt", "test04.txt"][0]) as f:
    topo = np.array([[int(c) for c in ln] for ln in f.read().strip().split("\n")])
dtopo = {y+1j*x: int(v) for ((y,x),v) in np.ndenumerate(topo)}
theads = [p for p, v in dtopo.items() if v==0]
dirs = [1,1j,-1,-1j]


# %%
# %%time
#16ms
def trails_conts(p,v):
    #print(p,v)
    #nbs w/ value v+1
    pv = dtopo.get(p)
    if pv is None or not pv==v:
        return set()
    if pv==9:
        return {p}
    return {vn 
            for nbset in (trails_conts(p+dir, v+1) for dir in dirs)
            for vn in nbset}

print(sum(len(trails_conts(th, 0)) for th in theads))


# %%
# %%time 
#15ms
def full_trails_conts(trail: tuple[int]):
    n = len(trail)
    p = trail[-1]
    pv = dtopo.get(p)
    if pv is None or not pv==n-1:
        return set()
    if n==10:
        return {trail}
    return {vn 
            for nbset in (full_trails_conts(tuple([*trail, p+dir])) for dir in dirs)
            for vn in nbset}

print(sum(len(full_trails_conts((th,))) for th in theads))
