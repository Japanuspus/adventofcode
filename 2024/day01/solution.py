# ---
# jupyter:
#   jupytext:
#     formats: py:percent
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

# %% [markdown]
# This python file is (the input part of) a jupyter notebook stored in "percent" format by `jupytext`. Install `jupytext` to view as notebook in jupyter.

# %%
import re
import numpy as np
import itertools

# %%
# ! aocprep

# %%
with open("input.txt") as f:
    lines = f.read().strip().split("\n")

# %%
m=np.array([[int(v) for v in ln.split()] for  ln in lines])

# %%
l=np.sort(m[:,0])
r=np.sort(m[:,1])
np.abs(l-r).sum()

# %%
bvals = {v: len(list(g)) for v,g in itertools.groupby(r.tolist(), lambda v: v)}
sum(a*bvals.get(a,0) for a in l)
