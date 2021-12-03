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


## Day 2

[`parse-display-derive`](https://crates.io/crates/parse-display-derive) is totally overkill for this, but so nice to work with.
First, define display form of inputs:

    #[derive(Display, FromStr, PartialEq, Debug)]
    enum Direction {
        #[display("forward")]
        Forward,
        #[display("down")]
        Down,
        #[display("up")]
        Up,
    }

    #[derive(Debug, FromStr)]
    #[display("{direction} {distance}")]
    struct Step {
        direction: Direction,
        distance: i32,
    }

And then parse as

    let input: Vec<Step> = fs::read_to_string("input.txt")?
        .split("\n")
        .map(|s| s.parse())
        .collect::<Result<_,_>>()?;


## Day 3

Was in doubt about the choise of `Vec<Vec<bool>>` as input datastructure -- would this have been a good day to play with `ndarray`?
Still, part 2 came out ok, although it probably has a massive bounds checking overhead:

    fn bit_rate(v: &Vec<Vec<bool>>, tgt: bool) -> &Vec<bool> {
        let mut buf: Vec<&Vec<bool>> = v.iter().collect();
        let mut idx = 0usize;
        while buf.len()>1 {
            let count: isize = buf.iter().filter(|r| r[idx]==tgt).count() as isize;
            let rem = (buf.len() as isize) - count;
            let mark = if count == rem {tgt} else {count>rem};
            buf = buf.into_iter().filter(|r| r[idx]==mark).collect();
            idx+=1;
        }
        buf[0]
    }

My natural tendency is totally towards loop with external mutable state. But I did use `fold` for the bit vector decoding...

    v.iter().fold(0, |s, &c| (s<<1)+if c {1} else {0})

