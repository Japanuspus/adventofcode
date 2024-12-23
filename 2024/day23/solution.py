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
import collections
import networkx as nx

# %%
with open(["input.txt", "test00.txt"][0]) as f:
    edges = [ln.split('-') for ln in  f.read().strip().split("\n")]

# %%
neighbors = collections.defaultdict(set)
for a,b in edges:
    neighbors[a].add(b)
    neighbors[b].add(a)
    
triples = set()
for a, others in neighbors.items():
    for b in others:
        for c in others.intersection(neighbors[b]):
            triples.add(tuple(sorted([a,b,c])))

print(len([t for t in triples if any(k.startswith('t') for k in t)]))

# %% [markdown]
# ## First time to break out networkx

# %%
g = nx.Graph(edges)
nodes, size = nx.algorithms.max_weight_clique(g, weight=None)
print(','.join(sorted(nodes)))

# %%
