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

# %%
#with open("test01.txt") as f:
with open("input.txt") as f:
    lines = f.read().strip().split("\n")

# %%
cmap = {(x,y): c for (y, ln) in enumerate(lines) for (x, c) in enumerate(ln)}

# %%
p0s = [(x,y) for (y, ln) in enumerate(lines) for (x, c) in enumerate(ln) if c=='X']
dirs = [(1,0),(1,-1),(0,-1),(-1,-1),(-1,0),(-1,1),(0,1),(1,1)]
cand = [(p, d) for p in p0s for d in dirs]
#cand = [((4, 0), (1, 1))]
for c, s in zip("MAS", range(1, 4)):
    cand = [((px,py),(dx,dy)) for ((px,py),(dx,dy)) in cand if cmap.get((px+s*dx, py+s*dy), None)==c]
len(cand)

# %%
p0s = [(x,y) for (y, ln) in enumerate(lines) for (x, c) in enumerate(ln) if c=='A']
dirs = [(1,1), (-1,1)]
ct=0
for (px,py) in p0s:
    if all({cmap.get((px+s*dx, py+s*dy), None) for s in [1, -1]}=={'M','S'} for (dx,dy) in dirs):
        ct+=1
ct


# %% [markdown]
# ## Cleaner versions after submission

# %%
dirs = [(1,0),(1,-1),(0,-1),(-1,-1),(-1,0),(-1,1),(0,1),(1,1)]
cand = [(p, d) for p in cmap.keys() for d in dirs]
for s, c in enumerate("XMAS"):
    cand = [((px,py),(dx,dy)) for ((px,py),(dx,dy)) in cand if cmap.get((px+s*dx, py+s*dy), None)==c]
len(cand)

# %%
# complete pos/dir pair in one go:
print(sum(1 for (px,py) in cmap.keys() for (dx,dy) in [(1,0),(1,-1),(0,-1),(-1,-1),(-1,0),(-1,1),(0,1),(1,1)] 
          if all(cmap.get((px+s*dx, py+s*dy), None)==c for s,c in enumerate("XMAS"))))

# %%
# same for part 2:
print(sum(1 for (px,py),c0 in cmap.items() 
          if c0=='A' and all({cmap.get((px+s*dx, py+s*dy), None) for s in [1, -1]}=={'M','S'} for (dx,dy) in [(1,1), (-1,1)])))

# %% [markdown]
# ## Compact form:

# %%
with open("input.txt") as f:
    cmap = {(x,y): c for (y, ln) in enumerate(f.read().strip().split("\n")) for (x, c) in enumerate(ln)}
print(
    sum(1 for (px,py) in cmap.keys() for (dx,dy) in [(1,0),(1,-1),(0,-1),(-1,-1),(-1,0),(-1,1),(0,1),(1,1)] 
        if all(cmap.get((px+s*dx, py+s*dy), None)==c for s,c in enumerate("XMAS"))),
    sum(1 for (px,py),c0 in cmap.items() if c0=='A' 
        if all({cmap.get((px+s*dx, py+s*dy), None) for s in [1, -1]}=={'M','S'} for (dx,dy) in [(1,1), (-1,1)])))

# %%
