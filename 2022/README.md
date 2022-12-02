# Advent of code 2022 in rust

This year I was tempted to have a go at array-programming with APL/[BQN](https://mlochbaum.github.io/BQN/), but I will at least start out with rust. Have not built much in rust over the last year, so this will be a good opportunity to freshen up.

As for the previous years, I will try to timebox my solutions were all done on the days they are released.

## General tools

Will use my hand-rolled [`aocprep`](https://github.com/Japanuspus/aocprep) from 2020.

## Day 1: Calorie Counting

Had to google `rust parse str to int`, but other than that it felt pretty natural coming back to Rust.

Same as the other years, I am thinking about ways to make the parts testable without defining too many types.
Maybe using owned strings for output and defining an `Input`-type for the day could work.
Also, the test-data functionality of `aocprep` seems to be broken.

Ended up doing a variation without the double split inspired by a post on [the solution megathread](https://www.reddit.com/r/adventofcode/comments/z9ezjb/2022_day_1_solutions/iyho95s/). 
The no-op `map_while` doesn't feel quite right though.

```rust
let elfs: Vec<i32> = itertools::unfold(
    input_s.trim().split("\n").map(|l| l.parse::<i32>().ok()),
    |lines| lines.map_while(|v| v).reduce(|acc, v| acc + v),
)
```

## Day 2: Rock Paper Scissors

Somehow decided that I wanted to do the Rock-Paper-Scissor games with only modular arithmetic, which took way too long for my morning brain. It ended up working in first go, but took som paper and pencil.

```rust
    // Rock: 0, Paper: 1, Scissor: 2
    // (b-a+1).rem_euclid(3)-1 : 0 on tie, 1 if b wins, -1 if a wins
    let part1:i32 = input.iter().map(|(a, b)| (b+1) + 3*(b-a+1).rem_euclid(3)).sum();
    let part2:i32 = input.iter().map(|(a, x)| ((a+x-1).rem_euclid(3)+1) + 3*x).sum();
```

The annoyance of the day: treating strings as bytes is noisy, instead of `s[0]-'A'` we get:
```rust
    (s.as_bytes()[0] as i32)-(b'A' as i32) 
```
