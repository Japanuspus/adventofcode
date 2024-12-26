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
import functools
import operator
import collections
import graphlib

# %%
with open(["input.txt", "test01.txt"][0]) as f:
    defines, props = f.read().strip().split("\n\n")

Rule = collections.namedtuple('Rule', 'a op b c')

defines = {k: int(v) for k,v in (ln.split(': ') for ln in defines.split('\n'))}
rules = [Rule(*[v for v in ln.split(' ') if v !='->']) for ln in props.split('\n')]

# %%
graph_def = {r.c: {r.a, r.b} for r in rules}
ts = graphlib.TopologicalSorter(graph_def)
order = list(ts.static_order())

# %%
vals = defines.copy()
opmap = {'XOR': operator.xor, 'AND': operator.and_, 'OR': operator.or_}
rule_map = {r.c: r for r in rules}
for c in order:
    if c in vals:
        continue
    rule = rule_map[c]
    vals[c] = opmap[rule.op](vals[rule.a], vals[rule.b])


# %%
sum(v*2**i for i,(k,v) in enumerate(sorted((k,v) for k,v in vals.items() if k.startswith('z'))))

# %%
