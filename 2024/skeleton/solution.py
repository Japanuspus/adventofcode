

# %%
import re
import numpy as np
import itertools
import math
import functools
import operator
import collections

# %%
# ! aocprep

# %%
with open("input.txt") as f:
    lines = f.read().strip().split("\n")
    # real, y, down -- imag, x, right
    pc = ((y+1j*x,c) for y,ln in enumerate(lines) for x, c in enumerate(ln))
