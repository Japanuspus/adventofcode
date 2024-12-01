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

# %%
# ! aocprep

# %%
with open("input.txt") as f:
    lines = f.read().strip().split("\n")

# %%
a=np.array([[int(v) for v in ln.split()] for  ln in lines])

# %%
a[:,0].sort()
a[:,1].sort()

# %%
np.abs(a[:,0]-a[:,1]).sum()

# %%
prev=np.nan
n=0
bvals=dict()
for v in a[:,1].tolist():
   if v==prev:
       n+=1
   else:
       bvals[prev]=n
       n=1
       prev=v
bvals[prev]=n

# %%
sum(a*bvals.get(a,0) for a in a[:,0])

# %%
