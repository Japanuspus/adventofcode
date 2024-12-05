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
