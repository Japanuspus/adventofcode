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
with open(["input.txt", "test03.txt"][0]) as f:
    plots = np.array([[c for c in ln] for ln in f.read().strip().split("\n")])

# %%
dplots = {(i+1j*j): c for (i,j), c in np.ndenumerate(plots)}

# %%
import collections

# %%
letters = collections.defaultdict(set)
for p,c in dplots.items():
    letters[c].add(p)

regions = []
dirs = [1,1j,-1,-1j]
for c, ps in letters.items():
    visited = set()
    while len(visited)<len(ps):
        #print('>', ps, visited)
        work = [next(v for v in ps if v not in visited)]
        region = set()
        while work:
            p = work.pop()
            region.add(p)
            visited.add(p)
            for d in dirs:
                work.extend(pd for pd in (p+d for d in dirs) if pd in ps and pd not in visited)
        regions.append((c, region))

print(regions)


# %%
tot = 0
dirs = [1,1j,-1,-1j]
for rc, ps in regions:
    perim = sum(1 for p in ps for d in dirs if p+d not in ps)
    # print(rc, len(ps), perim)
    tot += len(ps)*perim
print(tot)

# %%
tot = 0
dirs = [1,1j,-1,-1j]
for rc, ps in regions:
    perim = set((p, d) for p in ps for d in dirs if p+d not in ps)
    ct = sum(1 for p,d in perim if (p+(-1j)*d, d) not in perim)
    print(rc, len(ps), ct)
    tot += len(ps)*ct
print(tot)

# %%
