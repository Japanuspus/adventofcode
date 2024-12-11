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

# %%
with open(["input.txt", "test00.txt"][0]) as f:
    stones = [int(n) for n in f.read().strip().split(" ")]

# %%
stones


# %%
def nblink(st, n):
    if n==0:
        return 1
    if st==0:
        return nblink(1, n-1)
    elif (sval:=str(st)) and (lsval:=len(sval))%2==0:
        return nblink(int(sval[0:lsval//2]), n-1)+nblink(int(sval[lsval//2:]), n-1)
    else:
        return nblink(st*2024, n-1)


# %%
sum(nblink(st,25) for st in stones)


# %%

def blink(st):
    if st==0:
        yield 1
    elif (sval:=str(st)) and (lsval:=len(sval))%2==0:
        yield from [int(sval[0:lsval//2]), int(sval[lsval//2:])]
    else:
        yield st*2024


def mblink(first_stone, n):
    stones = [first_stone]
    for _ in range(n):
        stones = [s  for stone in stones for s in blink(stone)]
    return stones



# %%
@functools.lru_cache(maxsize=None)
def blink5(s,m:int):
    return mblink(s, m)

def process_stone(stone):
    res = 0
    plist = [(70,stone)]
    m=10

    while plist:
        n, s = plist.pop()
        print(n, len(plist))
        if n==5:
            res+=len(blink5(s, m))
        else:
            plist.extend((n-5, sn) for sn in blink5(s, m))

process_stone(1)



# %% [markdown]
# new take

# %%
#@functools.lru_cache(maxsize=1_000_000)
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

tot, work = 0, [(25, s) for s in stones]
bcache = dict()
while work:
    n, s = work.pop()
    if s in bcache:
        bif = bcache[s]
    else:
        bif = bifurcate(s)
        bcache[s] = bif
    if bif:
        nbif, a, b = bif
        if nbif>n:
            tot+=1
        elif nbif==n:
            tot+=2
        else:
            work.extend((n-nbif, v) for v in [a,b])
    else:
        tot+=1

tot


# %%
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
    if bif:=bifurcate(s):
        nbif, a, b = bif
        if nbif>n:
            return 1
        elif nbif==n:
            return 2
        else:
            return stones_from(a, n-nbif)+stones_from(b, n-nbif)
    else:
        return 1

sum(stones_from(s, 25) for s in stones)

# %%

sum(stones_from(s, 75) for s in stones)

# %%
