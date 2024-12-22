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
import itertools
import functools
import collections

# %%
with open(["input.txt", "test01.txt"][0]) as f:
    seeds = [int(ln) for ln in f.read().strip().split("\n")]


# %%
def step(n):
    n=(n^(n*64))%16777216
    n=(n^(n//32))%16777216
    n=(n^(n*2048))%16777216
    return n

assert step(123)==15887950


# %%
def multi_step(n,reps):
    for _ in range(reps):
        n=step(n)
    return n

print(sum(multi_step(s, 2000) for s in seeds))


# %%
def sequence(n, reps):
    last_price = n%10
    for _ in range(reps):
        n = step(n)
        price = n%10
        yield (price, price-last_price)
        last_price = price


# %%
def sequence_values(seed, reps):
    s = list(sequence(seed, reps))
    signals = [((d1,d2,d3,d4), v) for (_,d1), (_,d2),(_,d3), (v, d4) in zip(s,s[1:], s[2:], s[3:])]
    return {k: v for k,v in reversed(signals)}


# %%
v0, *vals = [sequence_values(s, 2000) for s in seeds]

# %%
comb = collections.defaultdict(lambda: 0, v0)
for vv in vals:
    for k,v in vv.items():
        comb[k]+=v

# %%
print(max(comb.values()))

# %%
