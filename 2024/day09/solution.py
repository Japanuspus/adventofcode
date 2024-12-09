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
from heapq import heappush, heappop, heapreplace

# %%
with open(["input.txt", "test00.txt"][0]) as f:
    disk_map = [int(c) for c in f.read().strip()]


# %%
def blocks(disk_map):
    p, bfree, bocc = 0, [], []
    for ib, v in enumerate(disk_map):
        if ib%2 == 0:
            bocc.append((p, ib//2, v))
        else:
            bfree.append((p, v))
        p+=v
    return bfree, bocc


# %%
bfree, bocc = blocks(disk_map)
bfree.reverse()
bmov, s, sf = [], 0, 0
while True:
    if s==0:
        p, id, s = bocc.pop()
    if sf==0:
        pf, sf = bfree.pop()
        if pf>p:
            bmov.append((p, id, s))
            break
    sm = min(s, sf)
    bmov.append((pf, id, sm))
    s-=sm
    sf-=sm
    pf+=sm
print(sum(id*(s*p+s*(s-1)//2) for (p, id, s) in bocc+bmov))

# %%
# %%time
bfree, bocc = blocks(disk_map)
bocc.reverse()
bmov = []
for p, id, s in bocc:
    (i, pf, sf) = next(
        ((i, pf, sf) for i, (pf, sf) in enumerate(bfree) if sf>=s),
        (-1, 10*len(disk_map), -1))
    if pf>p:
        bmov.append((p, id, s))
    else:
        bmov.append((pf,id, s))
        bfree[i]=(pf+s, sf-s)
print(sum(id*(s*p+s*(s-1)//2) for (p, id, s) in bmov))

# %% [markdown]
# The code above runs in 1.14s. Using heapq for tracking first space of a given size,
# we can reduce runtime to 24ms:

# %%
# %%time
bfree, bocc = blocks(disk_map)
heap_free = [[] for _ in range(10)]
for (p,s) in bfree:
    heappush(heap_free[s], p)
    
bocc.reverse()
cs = 0
for p, id, s in bocc:
    if (pfsf:=min((heap_free[sf][0], sf) for sf in range(s,10) if heap_free[sf])) and pfsf[0]<p:
        pf, sf = pfsf
        heappop(heap_free[sf])
        heappush(heap_free[sf-s], pf+s)
        p=pf
    cs += id*(s*p+s*(s-1)//2)
print(cs)

# %%
