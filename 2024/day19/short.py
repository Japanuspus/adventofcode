import functools
import collections


with open(["input.txt", "test00.txt"][0]) as f:
    ts, ds = f.read().strip().split("\n\n")
    towels = [[c for c in w] for w in ts.split(", ")]
    designs = [[c for c in w] for w in ds.split("\n")]

ts_by_start = collections.defaultdict(list)
for t in towels:
    ts_by_start[t[0]].append(t)


def possible(design):
    if len(design)==0:
        return True
    return any(possible(design[len(t):]) for t in ts_by_start[design[0]] 
               if len(t)<=len(design) and all(a==b for a,b in zip(design, t)))


@functools.lru_cache(maxsize=1_000_000)
def ways(design):
    if len(design)==0:
        return 1
    return sum(ways(design[len(t):]) for t in ts_by_start[design[0]] 
               if len(t)<=len(design) and all(a==b for a,b in zip(design, t)))


print(len(list(filter(possible, designs))))
print(sum(ways(tuple(design))  for design in designs))
