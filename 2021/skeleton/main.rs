use anyhow::Result;
use std::fs;

// use parse_display::{Display, FromStr};

// #[derive(Display, FromStr, PartialEq, Debug)]
// enum Direction {
//     #[display("forward")]
//     Forward,
// }

// #[derive(Debug, FromStr)]
// #[display("{direction} {distance}")]
// struct Step {
//     direction: Direction,
//     distance: i32,
// }

fn main() -> Result<()> {
    let input: Vec<i32> = fs::read_to_string("input.txt")?
        .split("\n")
        .map(|s| s.parse())
        .collect::<Result<_,_>>()?;

    println!("Part 1: {}", input.len());
    println!("Part 2: {}", input.len());
    Ok(())
}    