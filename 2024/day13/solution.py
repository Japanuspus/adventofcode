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

# %%
with open(["input.txt", "test00.txt"][0]) as f:
    machines = [[int(v) for v in re.findall(r"\d+", bl)] for bl in f.read().strip().split("\n\n")]

# %%
t = 0
for (x1,y1,x2,y2,xt,yt) in machines:
    m = np.array([[x1, x2], [y1, y2]])
    b = np.array([xt, yt])
    a=np.linalg.solve(m,b)
    aint = np.array([round(v) for v in a])
    res = m.dot(aint)
    if np.all(res==b):
        price = aint.dot([3,1])
        t+=price
    else:
        #print(m, b, a, aint, res, price)
        pass

print(t)

# %%
t = 0
shift = 10000000000000
for (x1,y1,x2,y2,xt,yt) in machines:
    m = np.array([[x1, x2], [y1, y2]])
    b = np.array([xt+shift, yt+shift])
    a=np.linalg.solve(m,b)
    aint = np.array([round(v) for v in a])
    res = m.dot(aint)
    if np.all(res==b):
        price = aint.dot([3,1])
        t+=price
    else:
        #print(m, b, a, aint, res, price)
        pass

print(t)

# %%
math.
