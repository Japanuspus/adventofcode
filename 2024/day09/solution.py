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
with open(["input.txt", "test00.txt"][0]) as f:
    disk_map = [int(c) for c in f.read().strip()]

len(disk_map)

# %%
p, bfree, bocc = 0, [], []
for ib, v in enumerate(disk_map):
    if ib%2 == 0:
        bocc.append((p, ib//2, v))
    else:
        bfree.append((p, v))
    p+=v
bfree.reverse()


# %%

p, bfree, bocc = 0, [], []
for ib, v in enumerate(disk_map):
    if ib%2 == 0:
        bocc.append((p, ib//2, v))
    else:
        bfree.append((p, v))
    p+=v
bfree.reverse()

bmov = []
s = 0
sf = 0
while True:
    if s==0:
        p, id, s = bocc.pop()
    if sf==0:
        pf, sf = bfree.pop()
        if pf>p:
            bmov.append((p, id, s))
            break
    if sf==0:
        continue
    sm = min(s, sf)
    #print(pf, id, s, sf, sm)
    bmov.append((pf, id, sm))
    s-=sm
    sf-=sm
    pf+=sm

# %%
cs = 0
for (pf, id, s) in (b for bl in [bocc, bmov] for b in bl):
    for _ in range(s):
        cs+=pf*id
        pf+=1
print(cs)

# %%
p, bfree, bocc = 0, [], []
for ib, v in enumerate(disk_map):
    if ib%2 == 0:
        bocc.append((p, ib//2, v))
    else:
        bfree.append((p, v))
    p+=v

bocc.reverse()
bmov = []
ib_max=len(bocc)-1
for p, id, s in bocc:
    (i, pf, sf) = next(
        ((i, pf, sf) for i, (pf, sf) in enumerate(bfree) if sf>=s),
        (-1, 10*len(disk_map), -1))
    if pf>p:
        bmov.append((p, id, s))
    else:
        bmov.append((pf,id, s))
        bfree[i]=(pf+s, sf-s)

# %%
cs = 0
for (pf, id, s) in bmov:
    for _ in range(s):
        cs+=pf*id
        pf+=1
print(cs)
