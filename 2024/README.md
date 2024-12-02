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