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
import operator


# %%
with open(["input.txt", "test00.txt", "test01.txt"][0]) as f:
    wares, movesc = f.read().strip().split("\n\n")

# x, im, right
# y, re, down
ware = {(y+1j*x): c for y,ln in enumerate(wares.split('\n')) for x,c in enumerate(ln) if c!='.'}
walls, boxes = set(), set()
for p,c in ware.items():
    match c:
        case '#':
            walls.add(p)
        case 'O':
            boxes.add(p)
        case '@':
            robot=p

dir_cs = {'>': 1j, '^': -1, '<': -1j, 'v': 1}
moves = [dir_cs[c] for c in movesc.strip() if c !='\n']
boxes0=boxes


# %%
def show(walls, boxes, robot):
    w = int(max(p.imag for p in walls))
    h = int(max(p.real for p in walls))

    def cval(p):
        if p in walls:
            return('#')
        elif p in boxes:
            return('O')
        elif p==robot:
            return('@')
        else:
            return('.')

    for y in range(h+1):
        print(''.join(cval(y+1j*x) for x in range(w+1)))


# %%
r=robot
boxes = boxes0.copy()
#show(walls, boxes, r)
for m in moves:
    #print(f"Move: {m}")
    r2 = r+m
    while r2 in boxes:
        r2+=m
    if r2 in walls:
        continue
    boxes.add(r2)
    boxes.remove(r+m)
    r=r+m
    #show(walls, boxes, r)


sum(100*p.real+p.imag for p in boxes)


# %%
def stretch(p):
    y,x = p.real, p.imag
    return [y+1j*2*x, y+1j*(2*x+1)]

r = stretch(robot)[0]
swalls = set(ws for w in walls for ws in stretch(w))
boxes = set(stretch(b)[0] for b in boxes0) #only left side in set

for m in moves:
    bs = set()
    to_clear = [r+m]
    #flag = False
    while to_clear:
        if to_clear[-1] in swalls:
            break
        p = to_clear.pop()
        if m.imag==0:
            # up/down
            for pc in [p, p-1j]: 
                if pc in boxes:
                    bs.add(pc)
                    to_clear.append(pc+m)
                    to_clear.append(pc+m+1j)
        elif m.imag==1:
            # right
            if p in boxes:
                bs.add(p)
                to_clear.append(p+2*m)
        elif m.imag==-1:
            #left
            if p+m in boxes:
                bs.add(p+m)
                to_clear.append(p+2*m)
        else:
            print(f"BAD MOVE: {m}")
    if to_clear:
        continue
    boxes = boxes-bs
    boxes = boxes.union(b+m for b in bs)
    r+=m

int(sum(100*p.real+p.imag for p in boxes))
