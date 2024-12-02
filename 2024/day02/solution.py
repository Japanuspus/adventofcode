# ---
# jupyter:
#   jupytext:
#     formats: py:percent
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

# %%
with open("input.txt") as f:
    lines = f.read().strip().split("\n")


# %%
reports = [np.array([int(b) for b in ln.split()]) for ln in lines]

# %%
diffs = [np.diff(r) for r in reports]


# %%
def check_report(r):
    d = np.diff(r)
    ad = np.abs(d)
    return (np.all(d>0) or np.all(d<0)) and np.all(ad>=1) and np.all(ad<=3)

sum(1 for r in reports if check_report(r))


# %%
def check_report_damper(ra):
    r=ra.tolist()
    for p in range(len(r)+1):
        rsub = r[:max(p-1,0)]+r[p:]
        if check_report(np.array(rsub)):
            return True
    return False


# %%
sum(1 for r in reports if check_report_damper(r))
