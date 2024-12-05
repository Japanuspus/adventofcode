from collections import defaultdict
import functools


with open("input.txt") as f:
    irules, iupdates = f.read().strip().split("\n\n")
    rules = [(int(a), int(b)) for a,b in (ab.split("|") for ab in irules.split("\n"))]
    updates = [[int(v) for v in ln.split(",")] for ln in iupdates.split("\n")]


def check_update(u):
    umap = {v:i for i, v in enumerate(u)}
    return not any( (i:=umap.get(a)) and (j:=umap.get(b)) and i>j for a,b in rules)


rule_map = defaultdict(set)
for (a,b) in rules:
    rule_map[a].add(b)


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