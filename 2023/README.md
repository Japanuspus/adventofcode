# Advent of code 2023 in rust

Similar to last year, I have been tempted to go for array-programming with APL/[BQN](https://mlochbaum.github.io/BQN/) -- but I am not writing much code these days, so keeping my rust alive seems prudent.

## General tools

Will use my hand-rolled [`aocprep`](https://github.com/Japanuspus/aocprep) from 2020.

## Day 1: Trebuchet?!

Spent way too long looking at my tests failing before realizing that test for part 2 needed to run on other input. 

After getting everything working, I spend some time reacquainting myself with rust error handling and stumbled over the [`anyhow!`](https://docs.rs/anyhow/latest/anyhow/#details) macro in the anyhow package, which makes it easy to create one-off error objects.

Extensions: I really should do a trie-search implementation...

## Day 2: Cube Conundrum 

Should have used nom or parse-display for parsing, but managed to discover that mixing up "green" and "blue" index gave same answer for the test problem.

Good learning from my clunky parser: the `str`-primitive has a `.split_once`-method.

Extension: use nom for parsing.

## Day 3: Gear Ratios

Tried different variations between functional and procedural style and decided that explicit for loops over complex iterators and using exterior mutable state was the most readable.

## Day 4: Scratchcards

Decided that this was the day to try out `nom` again, and it ended up being somewhat painful.

I should note that ergonomics of `nom` have improved a lot Since I first used it. Most significantly, suggested best practice is now to avoid macros. 
Furthermore, the `IResult`-type has been replaced with `Result<(rest, res), Err>`, which is much more ergonomic.

The issue I saw was related to using parser-combinators exclusively: 

1: You cannot reuse a combinator construction.
2: Error type hinting is quite verbose.
3: Variable space separators are verbose.

As an example, consider this

```
  let win_have = nom::sequence::separated_pair(
        nom::multi::separated_list1(
            nom::character::complete::space1::<&str, nom::error::Error<_>>,
            nom::character::complete::u16::<&str, nom::error::Error<_>>,
        ),
        nom::sequence::pair(
            nom::bytes::complete::tag(" |"),
            nom::character::complete::space1::<&str, nom::error::Error<_>>,
        ),
        nom::multi::separated_list1(
            nom::character::complete::space1::<&str, nom::error::Error<_>>,
            nom::character::complete::u16::<&str, nom::error::Error<_>>,
        ),
    );
```

Here the two pair-entries would ideally be defined externally, but this results in an "already borrowed"-error.

### Followup: making nom less verbose

Did the following:
- define my own module to re-export all relevant nom symbols without hierarchy.
- use closure to generate the `u16list`-parser.
- move parser to separate function with `IResult` return type to avoid turbofishes.
- use the `ws`-combinator from the nom recipes page
```
mod nm {
    pub use nom::multi::*;
    pub use nom::sequence::*;
    pub use nom::character::complete::*;
    pub use nom::bytes::complete::*;
    pub use nom::error::*;
    pub use nom::IResult;

    /// A combinator that takes a parser `inner` and produces a parser that 
    /// also consumes both leading and trailing whitespace, 
    /// returning the output of `inner`.
    pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
        inner: F,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        F: Fn(&'a str) -> IResult<&'a str, O, E>,
    {
        delimited(multispace0, inner, multispace0)
    }
}
```

Then the mess above becomes:

```
fn parse(s: &str) -> nm::IResult<&str, Vec<(u16, (Vec<u16>, Vec<u16>))>> {
    let u16list = || nm::separated_list1(nm::space1,nm::u16);
    let win_have = nm::separated_pair(u16list(),nm::ws(nm::tag("|")),u16list());
    let id = nm::delimited(nm::ws(nm::tag("Card")), nm::u16, nm::ws(nm::tag(":")));
    let card = nm::pair(id, win_have);
    nm::separated_list1(nm::newline, card)(s)
}
```

Another simplification might be the `nom_regex` crate.

## Day 5: If You Give A Seed A Fertilizer (173 us)

Thought long and hard to avoid having to deal with interval intersections, but in the end this was the way.

Extension: clean up the four-way logic in the interval code.  

## Day 6: Wait For It (<1 us)

My double-based solution agreed with brute force for all my part 1 problems -- but not for the example part 1 problems, so I might be lucky on this one.


## Day 7: (517 us)

Did not read part two of the ranking properly and implemented normal poker ranking. Which incidentally gives the correct result for the example input...

## Day 8: Haunted Wasteland (6.3ms)

The closed-cycles but not Chinese remainder day.

I missed the fact that the cycles looped back to start and spent time implementing chinese remainder for non-coprime moduli. Which is always good to have. Quoting for history as I may remove it:

```rust
use num::Integer};

#[derive(Debug, Clone, PartialEq, Eq)]
struct RSpec<T: Clone> {
    n: T,
    a: T,
}

fn chinese_remainder<T: Integer + Clone + fmt::Debug>(n1: RSpec<T>, n2: RSpec<T>) 
    -> Option<RSpec<T>> {
    let ee = T::extended_gcd(&n1.n, &n2.n);
    if ee.gcd != T::one() && n1.a.mod_floor(&ee.gcd)!=n2.a.mod_floor(&ee.gcd) {
        return None
    }
    let n = n1.n.clone() * n2.n.clone() / ee.gcd.clone();
    let a = ((n1.a * ee.y * n2.n + n2.a * ee.x * n1.n) / ee.gcd.clone()).mod_floor(&n); 
    Some(RSpec { n, a })
}
```

Also, should have probably gone for `[u8;3]` keys in the hashmap. Strings and ownership always ends up being a hassle.


## Day 9: Mirage Maintenance (200us)

## Day 10: Pipe Maze (93ms) 

Awesome day - everything worked in first try, and I got to revive some old code.
I wrote a mathematica implementation of the code [found here](https://web.archive.org/web/20100430183237/http://www.ecse.rpi.edu/Homepages/wrf/Research/Short_Notes/pnpoly.html) 
[back in 2008](https://insignificancegalore.net/2008/10/implementing-fast-point-in-polygon/).

### Extension: Vectorize the poin-in-polygon code

This did not give any speedup.

### Extension: Get this trait to work without taking ownership:

```
trait PolyTester {
    fn new(edge: Vec<V>) -> Self;
    fn point_in_poly(&self, test: V) -> bool;
}
```

## Day 11: Cosmic Expansion (177us)

## Day 12: Hot Springs (35ms)
My final solution is maybe caching too much -- went in to split up into two part before I realized I was spuriously returning without updating my manually updated cache.

- [ ] Read up on automatic memoization in rust
- [ ] Read up on rust profiling options

## Day 13: Point of Incidence (91us)

My first successful use of `rust ndarray`. Overall a good experience: passing views worked without type insanity, and [Zip](https://docs.rs/ndarray/latest/ndarray/struct.Zip.html) fitted the problem perfectly. 

To be honest the, ndarray zip-fold with no `Option` or `Result`-wrapping felt somewhat unrusty, but much nicer ergonomics for AOC-style things. Maybe I should just use `Array` instead of `Vec` in general...

## Day 14: Parabolic Reflector Dish (7.6s)

Got worried when my idea for cycle-detection was hit by `HashSet` not being hashable:
```
error[E0599]: the method `insert` exists for struct `HashMap<HashSet<[i8; 2]>, usize>`, but its trait bounds were not satisfied
   --> src/main.rs:78:17
    |
78  |         if hist.insert(balls, hist.len()) {
    |                 ^^^^^^
    |
...
    = note: the following trait bounds were not satisfied:
            `HashSet<[i8; 2]>: Hash`    
```
Luckily, `BTreeSet` implements `Hash`, and all was good.

Was pretty happy about this COW-pattern:

```rust
fn tilt(board: &Board, balls: &BTreeSet<V>, d: V) -> BTreeSet<V> {
    let mut balls: Cow<BTreeSet<V>> = Cow::Borrowed(balls);
    ...
    balls = Cow::Owned(new_balls);
    ...
    balls.into_owned()
}

```

Runtime is so-so. Only obvious implementation inefficiency is repeating the spin cycles after finding the period, so I expect I am missing some algorithmic insight.

## Day 15: Lens Library (230us)

Used `let mut boxes: Vec<HashMap<&str, (u32, u8)>>` for storing boxes, with the first entry being the operation index to get correct front-to-back ordering.
Worked fine, but I feel I have overlooked a data structure that would achieve this in a more scalable way.

## Day 16: The Floor Will Be Lava (340ms)

The main dispatch was nice and compact, but I could not find a good way to avoid heap allocation for the outcome. 
```
let mut ds: Vec<V> = Vec::new();
match (pd.1, circuit.get(&pd.0)) {
    (d, None) => {ds.push(d);}
    (d @ [0,_], Some('|')) => {ds.push(d);}
    (d @ [_,0], Some('-')) => {ds.push(d);}
    ([_,0], Some('|')) => {ds.push([0, 1]); ds.push([ 0,-1]);}
    ([0,_], Some('-')) => {ds.push([1, 0]); ds.push([-1, 0]);}
    ([dx, dy], Some('/'))  => {ds.push([-dy, -dx]);}
    ([dx, dy], Some('\\')) => {ds.push([ dy,  dx]);}
    _ => {}
};
```

## Day 17: Clumsy Crucible (295ms)

Tried profiling with `cargo instruments` (macOS-specific), but results where pretty useless due to the iterator chains.

Installing and running the profiler was simple:

```
cargo install instruments
```

Then added this to `Cargo.toml`:
```
[profile.profiler]
inherits = "release"
debug = true
```

And ran a time-profile with `cargo instruments --profile profiler -t time`.

## Day 18: Lavaduct Lagoon (200us)

Was happy when I realized that it was easy to get the answer from a Green's integral. Had almost started writing code to count inside and outside corners, when I realized that only the difference mattered.

On the rust side, I could not get `parse-display-derive` to parse my hex code, so I ended up with some ugly string wrangling.

## Day 19: Aplenty (190us)

Went with `nom` for the parsing. Was probably an ok choice, but took a while...

Did the interval arithmetic manually. Considered using the [interval](https://docs.rs/intervallum/latest/interval/interval/index.html) crate, but is seems mainly aimed at [interval arithmetic](https://en.wikipedia.org/wiki/Interval_arithmetic), so may not be ergonomic for this.

## Day 20: : Pulse Propagation (37ms)
Positives - nom was nice for the parsing:
```
fn parse(s: &str) -> nm::IResult<&str, Vec<ModuleSpec>> {
    let modtype = nm::opt(nm::one_of("&%"));
    let modspec = nm::pair(modtype, nm::alpha1);
    let receivers = nm::separated_list1(nm::ws(nm::char(',')),nm::alpha1);
    let module = nm::separated_pair(modspec, nm::ws(nm::tag("->")), receivers);
    nm::separated_list1(nm::newline, module)(s)
}
```

Part 2 was a very hand-held process of finding the structure of the signaling system. I left the code in spite of wanting to delete it all. 

The modules were divided into disjunct networks that each fed one input of the final conjunction. Each part had a loop period, but the initial state was not on the stable attractor so it took a full hash-setup to find it.

Only after that did i look at the outputs from the relevant nodes, and found that they sent their signals just before returning to the first non-trivial state:

```
x0 --> x1 --> x2 --> x4 - .... xn
       ^                        |
       |--- (signal here) ------v
```
So that e.g. n=4 would correspond to cycle length of 4, and signal after 4 button presses (had an off-by-one on this...).

All in all, after finding the cycles, part 2 answer was the lowest common multiple.

Off course, it would have been easier to just look at the output of the four interesting nodes and look for cycles there. But what would be the fun of that.

On the rust-side, I gave up on handling the state mutations in a function -- luckily it all worked when inlined.

## Day 21: Step Counter

Part 2 was mostly manual, and only happened when I looked closely on the input:
- The number of rounds corresponded to a half-integer multiple of the input size
- The input pattern has vertical and horizontal clear lines, and large horizontal "firebreaks"

Together this ensures that each block will be populates the same. 

I used small inputs with same characteristics to fit a 2nd order poly, which then gave the solution.

Code is a mess, of my attempts at a brute force solver.

## Day 22: Sand Slabs (9.8ms)

Used a `BTreeMap<(i16,i16), usize>>` for each `[x,y]`-pair to get the `nb_below` dependency while also tracking distances.
Then a depth-first traversal in this graph, to find the blocks with no unsettled blocks below and find how they settled in order to build the `rests_on`-data which was enough to solve both part 1 and 2.

The code maps the initial point pairs to a `Block`-structure which does not add any value. Also, since all the data structures were dense, I ended up using `usize`-indices rather than block refs. Not very rusty, really.

- Extension: Remove the blocks-structure.

## Day 23: A Long Walk (4.5s)

Parsed directly to the contracted graph, but failed to realize I was adding most paths from both ends. This made the brute force for part 2 time out.

