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
from typing import DefaultDict
from collections import defaultdict
import functools

# %%
with open("input.txt") as f:
    irules, iupdates = f.read().strip().split("\n\n")
    rules = [(int(a), int(b)) for a,b in (ab.split("|") for ab in irules.split("\n"))]
    updates = [[int(v) for v in ln.split(",")] for ln in iupdates.split("\n")]

# %%
rule_map = defaultdict(set)
for (a,b) in rules:
    rule_map[a].add(b)


# %%
def process_update(update):
    prev = set()
    for u in update:
        if rule_map[u].intersection(prev):
            return False
        prev.add(u)
    return True

sum(u[len(u)//2] for u in updates if process_update(u))


# %%
def fix_update(update):
    must_come_after={u: set() for u in update}
    for u in update:
        for n in rule_map[u]:
            if n in must_come_after:
                must_come_after[n].add(u)
    
    ps = set()
    p = []
    while must_come_after:
        drop = None
        for a,cs in must_come_after.items():
            if cs <= ps:
                ps.add(a)
                p.append(a)
                drop = a
                break
        if drop:
            del must_come_after[drop]
    
    return p

bad_updates = [u for u in updates if not process_update(u)]

sum(u[len(u)//2] for u in (fix_update(u) for u in bad_updates))


# %% [markdown]
# ## Niced up solution
#
# Less verbose `check_update` for part 1. This was the first idea, but somehow I didn't code it: Make a looup for the index of each entry in an update, and then check all applicable rules.
#
# Part 2 using custom comparison key for sort.
# This took me a little to find: you need to use [`functools.cmp_to_key`](https://docs.python.org/3/library/functools.html#functools.cmp_to_key).

# %%
def check_update(u):
    umap = {v:i for i, v in enumerate(u)}
    return not any( (i:=umap.get(a)) and (j:=umap.get(b)) and i>j for a,b in rules)
    
def rule_comp(a,b):
    if b in rule_map[a]:
        return 1
    elif a in rule_map[b]:
        return -1
    return 0

print(
    sum(u[len(u)//2] for u in updates if check_update(u)),
    sum(u[len(u)//2] for u in (sorted(u, key=functools.cmp_to_key(rule_comp)) for u in updates if not check_update(u)))
)

# %%

# %%
