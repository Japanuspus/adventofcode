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
import functools
import collections

# %%
with open(["input.txt", "test00.txt"][0]) as f:
    ts, ds = f.read().strip().split("\n\n")
    towels = [[c for c in w] for w in ts.split(", ")]
    designs = [[c for c in w] for w in ds.split("\n")]

# %%
# ? filter

# %%
# %%time
#100ms
ts_by_start = collections.defaultdict(list)
for t in towels:
    ts_by_start[t[0]].append(t)
def possible(design):
    if len(design)==0:
        return True
    return any(possible(design[len(t):]) for t in ts_by_start[design[0]] 
               if len(t)<=len(design) and all(a==b for a,b in zip(design, t)))

print(len(list(filter(possible, designs))))


# %%
# %%time
#677ms
@functools.lru_cache(maxsize=1_000_000)
def ways(design):
    if len(design)==0:
        return 1
    return sum(ways(design[len(t):]) for t in ts_by_start[design[0]] 
               if len(t)<=len(design) and all(a==b for a,b in zip(design, t)))

ws = ([ways(tuple(design))  for design in designs])
print(sum(ws))


# %% [markdown]
# ## Without the first-letter lookup

# %%
# %%time
# 410ms compared to 100ms for the original solution
def possible(design):
    if len(design)==0:
        return True
    return any(possible(design[len(t):]) for t in towels if len(t)<=len(design) and all(a==b for a,b in zip(design, t)))

print(len([design for design in designs if possible(tuple(design))]))


# %%
# %%time
#3000ms vs 700ms for the original solution
@functools.lru_cache(maxsize=1_000_000)
def ways(design):
    if len(design)==0:
        return 1
    return sum(ways(design[len(t):]) for t in towels
               if len(t)<=len(design) and all(a==b for a,b in zip(design, t)))

ws = ([ways(tuple(design))  for design in designs])
print(sum(ws))


# %%
