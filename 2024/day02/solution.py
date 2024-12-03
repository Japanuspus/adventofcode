# ---
# jupyter:
#   jupytext:
#     formats: py:percent
#     text_representation:
#       extension: .py
#       format_name: percent
#       format_version: '1.3'
#       jupytext_version: 1.11.2
#   kernelspec:
#     display_name: aoc
#     language: python
#     name: python3
# ---

# %%
import re
import numpy as np
import itertools

# %%
with open("input.txt") as f:
    lines = f.read().strip().split("\n")


# %%
reports = [[int(b) for b in ln.split()] for ln in lines]


# %%
def check_report(r):
    d = np.diff(np.array(r))
    ad = np.abs(d)
    return (np.all(d>0) or np.all(d<0)) and np.all(ad>=1) and np.all(ad<=3)
len(list(filter(check_report, reports)))


# %%
def check_report_damper(r):
    # leave out index p which may be outside list in which case whole list
    return any(check_report(r[:p]+r[p+1:]) for p in range(len(r)+1))
len(list(filter(check_report_damper, reports)))

# %% [markdown]
# ## Cleaned up version
#
# Think this will be my thing for this year: do a nice cleaned up version.
# Taking som hints from reddit, including [this idea](https://old.reddit.com/r/adventofcode/comments/1h4ncyr/2024_day_2_solutions/m0041k3/) for using subsets for the first check, we can get to this compact solution: 

# %%
with open("input.txt") as f:
    reports = [[int(b) for b in ln.split()] for ln in f.read().strip().split("\n")]

def check_report(r):
    diffs = {b-a for a,b in zip(r, r[1:])}
    return diffs<={-1,-2,-3} or diffs<={1,2,3}

def check_report_damper(r):
    return any(check_report(r[:p]+r[p+1:]) for p in range(len(r)+1))

[len(list(filter(c, reports))) for c in [check_report, check_report_damper]]

# %%
