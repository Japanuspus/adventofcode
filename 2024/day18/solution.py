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
import itertools
import collections
import heapq

# %%
[ifile, size, count1] = [["input.txt", 70, 1024], ["test00.txt", 6, 12]][0]
with open(ifile) as f:
    bytes = lines = f.read().strip().split("\n")
    # real, y, down -- imag, x, right
    bytes = [(int(y)+1j*int(x)) for x,y in [ln.split(',') for ln in lines]]
    

# %%
obstacles = frozenset({v for v in bytes[:count1]})

class P(collections.namedtuple("PBase", "p")):
    def __lt__(self, other):
        return False

smap = dict()
dirs = [1,1j, -1, -1j]
work = [(0, P(0))]
heapq.heapify(work)
while work:
    s, pp = heapq.heappop(work)
    p = pp.p
    if p in smap:
        continue
    smap[p]=s
    for d in dirs:
        pd = p+d
        if (0<=pd.real<=size) and (0<=pd.imag<=size) and pd not in obstacles:
            heapq.heappush(work, (s+1, P(pd)))

#smap
smap[(size+1j*size)]




# %%
class P(collections.namedtuple("PBase", "p")):
    def __lt__(self, other):
        return False
dirs = [1,1j, -1, -1j]

def check_count(count):
    obstacles = frozenset({v for v in bytes[:count]})
    smap = dict()
    work = [(0, P(0))]
    heapq.heapify(work)
    while work:
        s, pp = heapq.heappop(work)
        p = pp.p
        if p in smap:
            continue
        smap[p]=s
        for d in dirs:
            pd = p+d
            if (0<=pd.real<=size) and (0<=pd.imag<=size) and pd not in obstacles:
                heapq.heappush(work, (s+1, P(pd)))
    return size+1j*size in smap   



# %%
lb=0 #known good
ub=len(bytes) #above known bad
while ub-lb>1:
    count=(ub+lb)//2
    v = check_count(count)
    print(f"{count=} -> {v}")
    if v:
        lb=count
    else:
        ub=count

print(lb, ub, bytes[lb])
b=bytes[lb]
print(f"{int(b.imag)},{int(b.real)}")

# %%
