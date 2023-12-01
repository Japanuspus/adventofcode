# Advent of code 2023 in rust

Similar to last year, I have been tempted to go for array-programming with APL/[BQN](https://mlochbaum.github.io/BQN/) -- but I am not writing much code these days, so keeping my rust alive seems prudent.

## General tools

Will use my hand-rolled [`aocprep`](https://github.com/Japanuspus/aocprep) from 2020.

## Day 1: Trebuchet?!

Spent way too long looking at my tests failing before realizing that test for part 2 needed to run on other input. 

After getting everything working, I spend some time reacquainting myself with rust error handling and stumbled over the [`anyhow!`](https://docs.rs/anyhow/latest/anyhow/#details) macro in the anyhow package, which makes it easy to create one-off error objects.

