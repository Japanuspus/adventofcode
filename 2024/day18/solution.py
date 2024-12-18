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
import heapq

# %%
[ifile, size, count1] = [["input.txt", 70, 1024], ["test00.txt", 6, 12]][0]
with open(ifile) as f:
    # real, y, down -- imag, x, right
    lines = f.read().strip().split("\n")
    bytes = [(int(y)+1j*int(x)) for x,y in [ln.split(',') for ln in lines]]


# %%
class SP(collections.namedtuple("PBase", "s p")):
    "Noop wrapper because complex-encoded p's cannot be compared as required by heapq"
    def __lt__(self, other):
        return self.s < other.s

dirs = [1,1j, -1, -1j]
endpoint = size+1j*size

def distance_map(count):
    obstacles = frozenset({v for v in bytes[:count]})
    smap = dict()
    work = [SP(0, 0)]
    heapq.heapify(work)
    while work:
        s, p = heapq.heappop(work)
        if p in smap:
            continue
        smap[p]=s
        for d in dirs:
            pd = p+d
            if (0<=pd.real<=size) and (0<=pd.imag<=size) and pd not in obstacles:
                heapq.heappush(work, SP(s+1, pd))
    return smap


# %%
print(distance_map(count1)[endpoint])

# %%
lb=0 #known good
ub=len(bytes) #above known bad
while ub-lb>1:
    count=(ub+lb)//2
    v = endpoint in distance_map(count)
    if v:
        lb=count
    else:
        ub=count

b=bytes[lb]
print(f"{int(b.imag)},{int(b.real)}")
