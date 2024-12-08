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
import math
import collections

# %%
bounds = (0,0)
ant = collections.defaultdict(set)

with open(["input.txt", "test00.txt"][0]) as f:
    for y, ln in enumerate(f.read().strip().split("\n")):
        for x, c in enumerate(ln):
            bounds = tuple(max(a,b) for a,b in zip(bounds, (x,y)))
            if c=='.':
                continue
            ant[c].add(y+1j*x)

def check_bounds(p) -> bool:
    return all(0<=v<=b for v,b in zip((p.imag, p.real),bounds))


# %%
antinodes = set()
for c,ps in ant.items():
    anti_c = {p for pa in ps for pb in ps 
              for p in (2*pa-pb, 2*pb-pa) if (not pa==pb) and check_bounds(p)}
    #print(c, anti_c)
    antinodes = set.union(antinodes, anti_c)

len(antinodes)

# %%
antinodes = set()
max_dim = sum(b for b in bounds)
for c,ps in ant.items():
    for pa,pb in ((pa,pb) for pa in ps for pb in ps if not pa==pb):
        n = 1+math.ceil(max_dim/abs(pb-pa))
        dp = pb-pa
        # todo: check gcd == 1...
        antinodes.update(p for p in (pa+i*dp for i in range(-n,n+1)) if check_bounds(p))

len(antinodes)
