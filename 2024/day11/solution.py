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
import functools
import collections

# %%
with open(["input.txt", "test00.txt"][0]) as f:
    stones = [int(n) for n in f.read().strip().split(" ")]


# %%
# %%time
# 60ms
@functools.lru_cache(maxsize=1_000_000)
def bifurcate(s):
    # watch stone until it splits
    for n in range(75):
        if s==0:
            s=1
        elif len(str(s))%2==0:
            sval = str(s)
            lsval = len(sval)
            return (n+1, int(sval[0:lsval//2]), int(sval[lsval//2:]))
        else:
            s*=2024
    return None


@functools.lru_cache(maxsize=1_000_000)
def stones_from(s, n):
    match bifurcate(s):
        case None:
            return 1
        case (nbif,_,_) if nbif>n:
            return 1
        case (nbif,_,_) if nbif==n:
            return 2
        case (nbif, a, b):
            return stones_from(a, n-nbif)+stones_from(b, n-nbif)

print([sum(stones_from(s, n) for s in stones) for n in [25, 75]])


# %% [markdown]
# ## Alternative approach without recursion
#
# Just wanted to check my intuition on this one. The observation is that the order of stones do not matter, and that the scaling problem is due to a large number of identical stones -- which is why the lru-cache solved the problem in the recursive formulation.
# Here we just propagate the counts for each stone type

# %%
# %%time
#90ms with generator, 70ms without cache, 40ms with cache

def blink(st):
    if st==0:
        yield 1
    elif (sval:=str(st)) and (lsval:=len(sval))%2==0:
        yield from [int(sval[0:lsval//2]), int(sval[lsval//2:])]
    else:
        yield st*2024


#@functools.lru_cache(maxsize=1_000_000)
def blink(st):
    if st==0:
        return (1,)
    elif (sval:=str(st)) and (lsval:=len(sval))%2==0:
        return (int(sval[0:lsval//2]), int(sval[lsval//2:]))
    else:
        return (st*2024,)


def stone_prop(n_prop):
    counts = {s: 1 for s in stones} # no duplicates in input
    for _ in range(n_prop):
        new_counts = collections.defaultdict(lambda: 0)
        for s, n in counts.items():
            for new_s in blink(s):
                new_counts[new_s]+=n
        counts = new_counts
    return counts

print([sum(stone_prop(n).values()) for n in [25, 75]])

# %%

# %%
