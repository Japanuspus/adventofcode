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
