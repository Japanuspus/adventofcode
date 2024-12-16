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
import heapq

import collections

# %%
with open(["input.txt", "test00.txt"][0]) as f:
    lines = f.read().strip().split("\n")
    pc = ((y+1j*x,c) for y,ln in enumerate(lines) for x, c in enumerate(ln))
    walls = set()
    for p,c in pc:
        if c=='#':
            walls.add(p)
        if c=='S':
            start = p
        if c=='E':
            end = p
    walls = frozenset(walls)
east = 1j


# %%
# %%time
# 3.1s
PBase = collections.namedtuple("P", "p h")
class P(PBase):
    def __lt__(self, other):
        return (self.p.real, self.p.imag) < (other.p.real, other.p.imag)


work = [
    (0, P(start, east)),
    (1000, P(start, east*1j)),
    (1000, P(start, east*-1j))
]
heapq.heapify(work)
visited = dict()
while len(work)>0:
    d, (p,h) = heapq.heappop(work)
    if (dv:=visited.get((p,h))) is not None and dv<d:
        continue
    visited[(p,h)]=d
    if p+h not in walls:
        heapq.heappush(work, (d+1, P(p+h, h)))
        heapq.heappush(work, (d+1001, P(p+h, h*1j)))
        heapq.heappush(work, (d+1001, P(p+h, h*-1j)))

print(min(d for d in (visited.get((end, h)) for h in [1, 1j, -1, -1j]) if d is not None))

# %%
# %%time
# 150ms
PBase = collections.namedtuple("P", "p h ancestor")
class P(PBase):
    def __lt__(self, other):
        return (self.p.real, self.p.imag) < (other.p.real, other.p.imag)


work = [
    (0, P(start, east, None)),
    (1000, P(start, east*1j, None)), 
    (1000, P(start, east*-1j, None))
    ]
heapq.heapify(work)
visited = dict()
while len(work)>0:
    d, (p, h, ancestor) = heapq.heappop(work)
    if (prev_d_ancestors := visited.get((p,h), None)) is not None:
        prev_d, ancestors = prev_d_ancestors
        if prev_d==d:
            ancestors.append(ancestor)
            continue
        if prev_d<d:
            continue
    visited[(p,h)]=(d, [ancestor])
    if p+h not in walls:
        heapq.heappush(work, (d+1, P(p+h, h, (p,h))))
        heapq.heappush(work, (d+1001, P(p+h, h*1j, (p,h))))
        heapq.heappush(work, (d+1001, P(p+h, h*-1j, (p,h))))

dmin = min(visited.get((end, h), (1000*len(walls), None))[0] for h in [1, 1j, -1, -1j])
print(dmin)

# %%
ps = set([(end, east)])
work = [a for da in (visited.get((end, h)) for h in [1, 1j, -1, -1j]) if da is not None and da[0]==dmin for a in da[1]]
while work:
    p, h = work.pop()
    if (p,h) in ps:
        continue
    ps.add((p,h))
    _, ancestors = visited.get((p,h))
    work.extend(a for a in ancestors if a is not None)

print(len(set(p for p,h in ps)))
