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

## Day 5: If You Give A Seed A Fertilizer (173 us)

Thought long and hard to avoid having to deal with interval intersections, but in the end this was the way.

Extension: clean up the four-way logic in the interval code.  
