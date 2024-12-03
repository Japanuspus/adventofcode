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
# ! aocprep

# %%
with open("input.txt") as f:
    input =f.read().strip()


# %%
matches=re.findall(r"mul\((\d+),(\d+)\)", input)

# %%
sum(int(a)*int(b) for (a,b) in matches)

# %%
matches=re.findall(r"(mul\((\d+),(\d+)\))|(don't)|(do)", input)

# %%
s = 0
d = True
for m in matches:
    if m[4]:
        d=True
        continue
    elif m[3]:
        d=False
        print("saw dont")
        continue
    elif d:
        s+=int(m[1])*int(m[2])
s

# %%
