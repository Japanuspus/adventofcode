# Advent of code 2021 in rust

## General tools

Will be using `aocprep` from 2020 again.

## Day 1

Happily remembered about [windows](https://doc.rust-lang.org/std/primitive.slice.html#method.windows).
Tried to unpack the result of `.windows(2)` directly into `[a,b]`, but even with `[a, b, ..]` I was missing the length 1 and 2 cases so this of course did not work. 

    let p1: usize = input.windows(2).filter(|ab| ab[1] > ab[0]).count();

For part 2, I collected into an intermediate buffer since windows is a slice function.

    let input_w: Vec<usize> = input.windows(3).map(|w| w.iter().sum()).collect();
    let p2: usize = input_w.windows(2).filter(|ab| ab[1] > ab[0]).count();
