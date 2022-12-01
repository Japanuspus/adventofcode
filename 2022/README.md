# Advent of code 2022 in rust

This year I was tempted to have a go at array-programming with APL/[BQN](https://mlochbaum.github.io/BQN/), but I will at least start out with rust. Have not built much in rust over the last year, so this will be a good opportunity to freshen up.

As for the previous years, I will try to timebox my solutions were all done on the days they are released.

## General tools

Will use my hand-rolled [`aocprep`](https://github.com/Japanuspus/aocprep) from 2020.

## Day 1

Had to google `rust parse str to int`, but other than that it felt pretty natural coming back to Rust.

Same as the other years, I am thinking about ways to make the parts testable without defining too many types.
Maybe using owned strings for output and defining an `Input`-type for the day could work.
Also, the test-data functionality of `aocprep` seems to be broken.

Ended up doing a variation without the double split inspired by a post on [the solution megathread](https://www.reddit.com/r/adventofcode/comments/z9ezjb/2022_day_1_solutions/iyho95s/). 
The no-op `map_while` doesn't feel quite right though.
```
let elfs: Vec<i32> = itertools::unfold(
    input_s.trim().split("\n").map(|l| l.parse::<i32>().ok()),
    |lines| lines.map_while(|v| v).reduce(|acc, v| acc + v),
)
```