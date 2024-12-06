# Advent of Code 2024 solutions

This year will be python notebooks: I have not been writing much python since the big changes in 3.10 and forward, so this should be a good chance to get back up to speed.
As previous years, speed is not a goal -- I want to take the to explore some of the newlanguage features.

I will be coding in jupyter notebooks, but store them as [jupytext](https://jupytext.readthedocs.io/en/latest/) files.

## Day 01 - Historian Hysteria

Happy to be back with numpy as a first class citizen :)
Only thought of `itertools.groupby` after having done the grouping manually.

## Day 02 - Red-Nosed Reports

Saw my first ever HTTP-500 error from the AOC server! They must be seeing a massive onslaught each morning.
Same as yesterday, I reflectively googled to see if `len(list(...))` is really the nicest way of counting an iterator in python, but I guess it is.
Still, after dropping numpy `diff` and cleaning up, the python solution is nice, compact and readable:

```python
with open("input.txt") as f:
    reports = [[int(b) for b in ln.split()] for ln in f.read().strip().split("\n")]

def check_report(r):
    diffs = {b-a for a,b in zip(r, r[1:])}
    return diffs<={-1,-2,-3} or diffs<={1,2,3}

def check_report_damper(r):
    return any(check_report(r[:p]+r[p+1:]) for p in range(len(r)+1))

[len(list(filter(c, reports))) for c in [check_report, check_report_damper]]
```

## Day 03 - Mull It Over

My newfound [jupytext for VS-code](https://github.com/notebookPowerTools/vscode-jupytext) was having issues, so I ended up in jupyter lab again.

Solved with `re` and learned that `re.findall` always returns tuples: no match objects.
Tried [structural pattern matching](https://peps.python.org/pep-0636/) but liked the if-chain better.

## Day 04 - Ceres Search

First day working in VS-Code jupyter. Debugger and variable view are nicer than jupyter lab, but having to do jupytext as a followup step is not very ergonomic.

Had fun trimming the final version down in size:
```python
with open("input.txt") as f:
    cmap = {(x,y): c for (y, ln) in enumerate(f.read().strip().split("\n")) for (x, c) in enumerate(ln)}
print(
    sum(1 for (px,py) in cmap.keys() for (dx,dy) in [(1,0),(1,-1),(0,-1),(-1,-1),(-1,0),(-1,1),(0,1),(1,1)] 
        if all(cmap.get((px+s*dx, py+s*dy), None)==c for s,c in enumerate("XMAS"))),
    sum(1 for (px,py),c0 in cmap.items() if c0=='A' 
        if all({cmap.get((px+s*dx, py+s*dy), None) for s in [1, -1]}=={'M','S'} for (dx,dy) in [(1,1), (-1,1)])))
```

## Day 05 - Print Queue

It was only after doing the initial solution that I had the calm to read on how to use custom comparators in python: turns out functools have a magical [`functools.cmp_to_key`](https://docs.python.org/3/library/functools.html#functools.cmp_to_key) that use memoization to build a key-function. This is almost cheating...

Edit: `cmp_to_key` is not doing any memoization, somehow it triggers the internal comparator functionality. Should really look at some source here.

## Day 06 - Guard Gallivant

Worked in first try with relatively performant code but naive algorithm (8s runtime).
Looking forward to seeing the clever solutions on this one.

