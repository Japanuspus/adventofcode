The solutions in this repository are mostly in the form I initially wrote them.
Below I try to keep notes of things I learned while solving each day, as well as ideas for changes and interesting alternative approaches,
picked up mostly from the [/r/adventofcode](https://www.reddit.com/r/adventofcode) solution megathreads.

## Notes individual days

### Day 4

Found [parse-display-derive] which allows 

```
#[derive(Debug, FromStr)]
#[display("{key}:{value}")]
struct KeyVal {key: String, value: String}
...
"Foo: bar".parse::<KeyVal>()
```

### Day 6

Another good day for [parse-display-derive]:
```
#[derive(Debug, FromStr)]
#[from_str(regex = r"(?P<count>[0-9]+) (?P<bag_type>\w+ \w+) (bag|bags).?")]
struct HoldsSpec {
    bag_type: String,
    count: usize,
}
```

Also, re-learned that to get `collect` to collect `Result`s to `Result<Vec,_>` you [just need to ask: `.collect::<Result<Vec<_>, _>>()`](https://doc.rust-lang.org/rust-by-example/error/iter_result.html#fail-the-entire-operation-with-collect)

### Day 7

Days 1 through 6 used my `aoclib` crate to download inputs on demand, but this was not a good solution because
1. I was reminded how irritating it is to read other solutions which import boilerplate.
2. Including the automatics in each build caused pretty high build times (`reqwest` implies `tokio`, etc.)

Instead, starting from day 7 I am using [`aocprep`](https://github.com/Japanuspus/adventofcode/blob/master/2020/aocprep_src/src/main.rs) which is a small tool that
- instantiates a day-folder skeleton if the day-folder does not exist
- tries to download input to `<day-folder>/input.txt` if this file does not exist

The code is more verbose than my python-equivalent from last year, but not massively so, given all the edge cases.
Also, working with `StructOpt` for CLI arguments is such a joy.


### Day 8

By experience form 2019 I am keeping the VM as modular as possible. Was tempted to do an immutable register state, but in the end I didn't.
Was tempted to include the flag state in register state but decided to return a flag instead. If I end up needing it, I can compose flag and register state.

[mboehnke](https://gitlab.com/mboehnke/aoc-2020/-/blob/master/aoc-2020-08/src/solution.rs) has a couple of nice tricks:
- Implementing each part in a function returning `impl std::fmt::Display`
- Using `find_map(f)` as equivalent to `filter_map(f).next()`
- Using the [also](https://docs.rs/crate/also/0.1.0) crate to get `.also()` function mapping

[smmalis37](https://github.com/smmalis37/aoc2020/blob/main/src/days/day8.rs) avoids brute-forcing part 2.

[parse-display-derive]: https://crates.io/crates/parse-display-derive


### Day 11

Kept flip flopping between `[i16;2]` and `(i16, i16)` for coordinates, and wether to use hash directly on points or use vertex indices for the graph. Not very pretty.

### Day 12

One more round of writing component-equations for vector entities. Next time I should try out [vecmath] by the Piston developers, which seems exactly the minimal vector functionality I need.

[vecmath]: https://crates.io/crates/vecmath

### Day 13

A lot of unnecessary work on this one: couldn't find a chinese remainder implementation so I wrote one from scratch, only to realize that `isize` would not hold the result.
Started refactoring for `BigInt` before I realized that the `num-integer` crate (part of [`num`](https://crates.io/crates/num)-crate) has implements `extended_gcd` which is the heavy lifting of chinese remainder.

Main challenge working with the `BigInt`-objects is that I have not found how to run the mathematical operations from references, which causes a lot of clones and explicit intermediates.
Also, there is a bit of ecosystem fragmentation, i.e. `Integer` uses `mod_floor` as opposed to `rem_euclid` in the `std`-integers.


### Day 17

Ok experience with the [vecmath] crate. Const generics would have been nice sharing code between parts 1 and 2.
Missed opportunity for using `try_into` to go from vector to array.

### Day 18

Used `Regex::find_iter` as tokenizer and it worked a charm.
Part 1 is a mess, but the recursive descend parser for part 2 came out ok.

## The big [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html) - [`Result`](https://doc.rust-lang.org/stable/std/result/enum.Result.html) table

The things I always spend the most time searching for

<table>
<tr><td><th>Option</th><th>Result</th></tr>

<tr><th>Option</th>
<td><code><pre>
as_ref/as_mut
map(self, f: F) 
filter(self, f: F)
zip

and/and_then
or/or_else

take, replace

copied, cloned
as_deref

flatten
</pre></code></td>
<td><code><pre>
ok()
err()
</pre></code></td>

<tr><th>Result</th>
<td><code><pre>
ok_or/ok_or_else
transpose
</pre></code></td>
<td><code><pre>
map/map_or/map_or_else
map_err

and/and_then
or/or_else



</pre></code></td>

<tr><th>Any</th>
<td><code><pre>
⚠ expect(self, msg: &str) -> T
⚠ unwrap(self) -> T

unwrap_or(self, default: T) -> T
unwrap_or_else(self, default: f)

map_or(f, default: T) -> T
map_or_else(f, default: f) -> T

iter/iter_mut

get_or_insert/get_or_insert_with
</pre></code></td>
<td><code><pre>
⚠ expect(self, msg: &str) -> T
⚠ unwrap(self) -> T
⚠ expect_err(self, msg: &str) -> T
⚠ unwrap_err(self) -> T

unwrap_or(self, default: T) -> T
unwrap_or_else(self, default: f)

is_ok
iter
</pre></code></td>




