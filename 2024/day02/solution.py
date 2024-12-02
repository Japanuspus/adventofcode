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
