from collections import defaultdict
import functools
import re

def check_update(u):
    umap = {v:i for i, v in enumerate(u)}
    return not any( (i:=umap.get(a)) and (j:=umap.get(b)) and i>j for a,b in rules)


with open("input.txt") as f:
    rules, updates =[[[int(v) for v in re.split("[,|]",ln)] 
                      for ln in bl.split("\n")] for bl in f.read().strip().split("\n\n")]

cmp_map = {ab: s for a,b in rules for ab,s in [((a,b),1),((b,a),-1)]}

print(
    sum(u[len(u)//2] for u in updates if check_update(u)),
    sum(u[len(u)//2] for u in (sorted(u, key=functools.cmp_to_key(lambda a,b: cmp_map.get((a,b),0))) for u in updates if not check_update(u)))
)