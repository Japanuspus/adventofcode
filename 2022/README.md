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

## Day 3: Rucksack Reorganization

Not very pretty with bytes in hash sets.
In particular, for part two I split into tuples to iterate over chunks, so that the chunk size had to be hard coded. Would have been nicer to build the iterator and then iterate over the chunks in a for loop.

## Day 4: Camp Cleanup

My initial morning solution had the inelegant contains/overlap check -- had to go back and change it.
Day 4 was the first day of using `parse_display`, which always feels like magic. 


## Day 5: Supply Stacks

Was bitten by a `trim` on the input in my skeleton-code: This is apparently the first AOC in the last two years with significant leading whitespace...
Used [`VecDeque`](https://doc.rust-lang.org/std/collections/vec_deque/struct.VecDeque.html) for storing the stacks. For part 2 I could probaly have used `.split_off` to avoid building a buffer of the items to be moved.

## Day 6: Tuning Trouble 

Learned about `.find_map` from the rust-analyzer hints:

```rust
    s.as_bytes().windows(n).enumerate().find_map(|(i, grp)|{
        if grp.iter().collect::<HashSet<_>>().len()==n {Some(i+n)} else {None}
    })
```

**extension**: Implement with mutating set of elements in window instead of building a new set for each window.

Update: Tried some rewrites in this direction. Speedup was from 800us to around 250us.

Update 2: Learned about `.all_unique` from reddit megathread. This is more concise -- and runs faster than building the set (since it can abort as soon as it sees the collision?). 

```rust
    s.as_bytes().windows(n).enumerate().find_map(|(i, grp)|{
        if grp.iter().all_unique() {Some(i+n)} else {None}
    })
```

## Day 7: No Space Left On Device

Wrote a very very ugly parser. But then at least I stopped myself before doing a full recursive depth first traversal, when I realized I could just sort the paths by length and then compute the sizes starting from the longest.

```rust
let mut sizes: HashMap<&FolderPath, usize> = HashMap::new();
for (folder_path, entry) in dirs.iter().sorted_by_key(|(v, _)| -(v.len() as isize)) {
    let tot: usize = entry.files_size
        + entry.children.iter().map(|c| sizes.get(c).unwrap()).sum::<usize>();
    sizes.insert(folder_path, tot);
}
let part1: usize = sizes.values().filter(|&v| *v <= 100000).sum();
```

Cleanup: Nice [parser trick on the megathread](https://www.reddit.com/r/adventofcode/comments/zesk40/2022_day_7_solutions/iz8f2r7/): Split on `$`. I implemented this because my first parser was awful.

## Day 8: Treetop Tree House

Lost quite a bit of time because I misunderstood part 2 to include trees visible trees shadowing each other. Also, my first implementation used all manual index handling...
```rust
let a1 = p0.0+d1.0*(i1 as i32)+d2.0*(i2 as i32);
let a2 = p0.1+d1.1*(i1 as i32)+d2.1*(i2 as i32);
```

For the cleanup, I picked up [`vecmath`](https://docs.rs/vecmath/latest/vecmath/) from the piston team, which allowed me to write:
```rust
let a = vec2_add(*p0, vec2_add(vec2_scale(*d1, i1), vec2_scale(*d2, i2)));
```

## Day 9: Rope Bridge

Forgot the first rule of AOC: Never use methods - always functions.
My initial part 1 solution used a struct with `head` and `tail` to represent the rope, and then it was too easy to add a `.step`-method to the implementation. 

Good thing was that most of the code carried over nicely to part 2, and I got to play with const-generics for the first time:

```rust
fn step(rope: &mut [[i32;2]], direction: &Direction) {
    let dh = match direction {
        Direction::U => [0,  1], 
        Direction::D => [0, -1], 
        Direction::L => [-1, 0], 
        Direction::R => [ 1, 0], 
    };
    rope[0] = vec2_add(rope[0], dh);
 
    // Move tails
    for h in 0..(rope.len()-1) {
        let t = h+1; 
        let d = vec2_sub(rope[h], rope[t]);
        if (0..2).any(|i| d[i] < -1 || d[i]>1) {
            for i in 0..2 {
                rope[h+1][i]+=d[i].signum();
            }    
        }    
    }
}
```

It would have been less verbose to use a `char` to encode the directions: Would save the enum-definition in return for a default clause. 

## Day 10: Cathode-Ray Tube (34us)

Worked on first try. Spent most time on getting `parse_display` to accept my enum formatting, and on inserting newlines into the part 2 result...