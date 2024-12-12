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
import numpy as np
import collections
import itertools

# %%
with open(["input.txt", "test03.txt"][0]) as f:
    dplots = {int(y)+1j*int(x): c for (y,ln) in enumerate(f.read().strip().split("\n")) for (x,c) in enumerate(ln)}

# %%
letters = {c: set(p for p,c in g) for c, g in itertools.groupby(dplots.items(), key=lambda pc: pc[1])}

# %%
dirs = [1,1j,-1,-1j]

# %%
regions = []
for c, ps in letters.items():
    visited = set()
    while len(visited)<len(ps):
        work = [next(v for v in ps if v not in visited)]
        region = set()
        while work:
            p = work.pop()
            region.add(p)
            visited.add(p)
            for d in dirs:
                work.extend(pd for pd in (p+d for d in dirs) if pd in ps and pd not in visited)
        regions.append((c, region))

# %%
# %%time
# 21ms
tot1, tot2 = 0,0
for rc, ps in regions:
    perim = set((p, d) for p in ps for d in dirs if p+d not in ps)
    ct = sum(1 for p,d in perim if (p+(-1j)*d, d) not in perim)
    # print(rc, len(ps), ct)
    tot1 += len(ps)*len(perim)
    tot2 += len(ps)*ct
print(tot1, tot2)

# %%
