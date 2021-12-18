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


## Day 4

First part 1 implementation was nester for loops checking for subsets.
Then changed everything to have mutable board implementation.

The nicer take would have been to map from drawn numbers to `(&board, line_idx)` and then count up number of matches.

## Day 5
Almost tempted for `ndarray`, but `HashMap<(i32, i32), usize>` was super. 

## Day 8

Very clunky solution using `BTreeSet`s. Was surprised that `BTreeSet` has no in-place set operations. Should have probably stayed with initial thought of using bit vectors.

Also: complete missed the clever solution of mapping activation groups directly to digits, skipping the step of mapping segments to segments.

## Day 9
Another good day for `HashMap<(i32, i32), _>`.

## Day 14
First day where a clever algorithm was needed: brute force went belly up on part 2.

My implementation missed the fact that there will always be a matching rule, which causes some extra complexity. Found a really nice solution by [timvisee](https://github.com/timvisee/advent-of-code-2021/blob/master/day14a/src/main.rs) using
- `binary_search_by_key` instead of some fancy map to lookup rules
- precomputes rule outcomes to avoid further lookups
- uses `split-once` for efficient parsing


## Day 15

Used `ndarray` for the maps for the first time. Was quite nice to work with.
Major downside compared to using a map is that there is no `entry` -- so I have to do bounds checks (and cry that more bounds check is being done by `ndarray`). Came up with this model:

    struct Bounds {
        b: (usize, usize),
    }

    impl Bounds {
        fn add(&self, p: (usize, usize), dp: (i32, i32)) -> Option<(usize, usize)> {
            let i = p.0 as isize + dp.0 as isize;
            let j = p.1 as isize + dp.1 as isize;
            if i >= 0 && i < (self.b.0 as isize) && j >= 0 && j < (self.b.1 as isize) {
                Some((i as usize, j as usize))
            } else {
                None
            }
        }
    }

The `.add` implementation would have benefitted from [`checked_add_signed`](https://doc.rust-lang.org/std/primitive.usize.html#method.checked_add_signed), but no nightly here. Works nicely like so

    dirs.iter().filter_map(|d| bounds.add(position, *d))


## Day 16

Used the `bitvec` crate, but was initially frustrated trying to use it with `nom`. Issue turned out to be that `nom` `Err` results maintain a reference to the input. 


## Day 17: Trick Shot

Spent a lot of time finding a correct closed form solution for the possible intersects given y-velocity. Once that was working, things were nice (using part 2 as example to remind myself of `scan`)

    let p2:usize = (vy_min..=vy_max)
    .filter_map(|vy| y_range(vy, &t.y))
    .map(|(n1, n2)| 
        (vx_min..vx_max).filter(|vx0| 
            (1..=n2)
            .scan((*vx0, 0), |(vx, x), i| {if *vx>0 {*x+=*vx; *vx-=1;}; Some((i, *x))})
            .any(|(i, x)| i>=n1 && x>=t.x.a && x<=t.x.b)
        ).count()
    )
    .sum();

## Day 18: Snailfish (recursive pairs)

Tried doing a visitor-pattern, but got stuck in fighting the borrow checker.
The recursive solution was ok-ish, but a stateful visitor would probably have been as well. Looking forward to seeing other solutions for this.

The recursive descent parser was nice (didn't use nom..m)

    fn parse_node(i: &[u8]) -> Result<(&[u8], Node)> {
        let c = i[0];
        if c==b'[' {
            let (i, a) = parse_node(&i[1..])?;
            let (i, b) = parse_node(&i[1..])?; //skipping ,
            Ok((&i[1..], Node::Pair{a: Box::new(a), b: Box::new(b)}))
        } else {
            Ok((&i[1..], Node::Number{a: (c-b'0') as isize}))
        } 
    }

... wrapped this in a `FromStr`-impl: 

    impl FromStr for Node {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            parse_node(s.as_bytes())
            .and_then(|(_, v)| Ok(v)).with_context(|| format!("Parsing {}", s))
        }
    }