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
with open(["input.txt", "test00.txt"][0]) as f:
    blocks = f.read().strip().split("\n\n")

# %%
locks = []
keys = []
for b in blocks:
    lns = b.split('\n')
    pos = [None]*5
    for i,(la,lb) in enumerate(zip(lns, lns[1:])):
        for k in (k for k,(a,b) in enumerate(zip(la,lb)) if a!=b):
            pos[k]=i
    (locks if lns[0].startswith('#') else keys).append(pos)

# %%
print(sum(1 for lock in locks for key in keys 
        if all(l<=k for l,k in zip(lock, key))))

# %%
