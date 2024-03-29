# Advent of code 2021 in rust

These solutions were all done on the days they were released, but this was primarily to avoid using the whole day trying to find the perfect solution before implementing, and to be sure I found a solution before inadvertently stumbling over a solution on reddit.

Most solutions are the same version I made on the day of the calendar, but I have marked some days with **possible extension** if I ever get the time to go back and clean up.

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

## Day 19: Beacon Scanner

Spent forever on this one and ended up with a monstrosity. 

This was my first time really using `ndarray`. In general, ergonomics were good, but when trying to compute a determinant I was required to install the linear algebra extension which has external dependencies.
Seems there might be a need for a "basic linear algebra without external dependencies"-package.

Performance was quite bad: Even with `--release`, the run took several seconds.

When matching up the maps, I used a lookup from pair-distance vectors which required me to compute all the rotations to rule out a match. A better approach would have been to use Manhatten distance or some other rotation invariant for key. Also, I could have used `ndarray` broadcast to build the entire distance set as an `Array3`, which would have also allowed me to transform it.
Probably would have made sense to make a struct with all the gunk related to a sensor reading.

**Possible extension**: Use pair-distance for excluding matches.

## Day 20: Trench Map

Only checked whether 0 mapped to 0 for the test input, not for my own -- so the initial solution was bad.
A good day for `bitvec` and `ndarray`, although I could have probably done something really funky with bitvecs for everything.

## Day 21: Dirac Dice

Tried a recursive solution which timed out. Did not think about simply caching it, so I ended up tracking the full game state across rounds.
Was actually quite concise, and should be quite efficient.

    let mut wins = [0usize;2];
    while states.len() > 0 {
        for i in 0..2 {
            let mut states_next: Counts = HashMap::new();
            for (s0, ct) in states.into_iter() {
                for roll in 3..=9 {
                    let n = ct * roll_count[roll as usize];
                    let s1 = s0.play(i, roll);
                    if s1.score[i] >= 21 {
                        wins[i]+=n;
                    } else {
                        *states_next.entry(s1).or_default() += n;
                    }
                }
            }
            states = states_next;
        }
    }


## Day 22: Reactor Reboot

The examples gave that splitting into regions on all three axes and then scanning over the cartesian product of these would be feasible. 
It did work, but runtime was terrible. Looking back, the right solution would have been recursive scanlines.
Still, the brute-force solution was pretty concise: 

    let break_after: Vec<Vec<i32>> = (0..3).map(|i| {
        input
        .iter()
        .flat_map(|s| [s.range[i][0]-1, s.range[i][1]].into_iter())
        .collect::<BTreeSet<i32>>()
        .apply(|b_set| b_set.into_iter().collect())
    }).collect();
    break_after.iter()
    .map(|breaks| breaks.windows(2))
    .multi_cartesian_product()
    .map(|rs| 
        if input.iter().rev().filter_map(|step| step.contains_range(&rs)).nth(0).unwrap_or(false) {
            rs.iter().map(|ab| (ab[1]-ab[0]) as usize).product()
        } else {
            0usize
        })
    .sum()

**Possible extension**: Use scanlines.

## Day 23: Amphipod

My only global leaderboard position -- by solving part 1 manually :blush:.

Solution was a simple Dijkstra, but could be extended to A* by adding the cost to move everything home without interactions.

One thing that annoyed me was this part:

    if r {
        work.extend(moves_in(&b, i).map(|(move_cost, new_board)| Reverse((cost+move_cost, new_board))));
    } else {
        work.extend(moves_out(&b, i).map(|(move_cost, new_board)| Reverse((cost+move_cost, new_board))));
    }

It would have been nice to do something like this - should look into `dyn`.

    work.extend(
        if r {moves_in(&b, i)} else {move_out(&b, i)}
        .map(|(move_cost, new_board)| Reverse((cost+move_cost, new_board)))
    );



**Possible extensions**: Use A*.


## Day 24: Arithmetic Logic Unit

I cheated and converted the code to an algebraic expression manually....

After that I used a depth first search, making use of the facts that this would give the desired solution by choosing the right order of the visits to the digits. 
This solution would also work for a generic state machine, but it would not be able to take advantage of the fact that the `x` and `y`-registers were ignored, so the cache would be less efficient. But more fun...

**Possible extension**: Solve day 24 without hand-parsing the code.

## Day 25: Sea Cucumber

Kept each species in a separate ndarray and used a transposed view to make the same move operation work for both. The move itself was surprisingly clunky, but all in all the code very concise -- 67 lines in total.

**Possible extension**: Try cleaning up the move logic.

