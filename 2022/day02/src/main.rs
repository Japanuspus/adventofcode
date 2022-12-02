#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use std::fs;

// use parse_display::{Display, FromStr};

// #[derive(Display, FromStr, PartialEq, Debug)]
// enum Direction {
//     #[display("forward")]
//     Forward,
// }

// #[derive(Debug, Display, FromStr)]
// #[display("{direction} {distance}")]
// struct Step {
//     direction: Direction,
//     distance: i32,
// }

// Rock: 0, Paper: 1, Scissor: 2
// (b-a+1).rem_euclid(3)-1 : 0 on tie, 1 if b wins, -1 if a wins

fn game(a: i32, b: i32) -> i32 {
    (b-a+1).rem_euclid(3)-1
}

fn solution(input_s: &str) -> Result<()> {
    let input: Vec<(i32, i32)> = input_s
        .trim()
        .split("\n")
        .map(|s| (
            (s.as_bytes()[0] as i32)-(b'A' as i32), 
            (s.as_bytes()[2] as i32)-(b'X' as i32)
        ))
        .collect();

    let part1:i32 = input.iter().map(|(a, b)| (b+1) + 3*(game(*a, *b)+1)).sum();
    let part2:i32 = input.iter().map(|(a, x)| 3*x + 1+(a+x-1).rem_euclid(3)).sum();
    
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

fn main() -> Result<()> {
    println!("** TEST **");
    solution(&fs::read_to_string("test00.txt")?)?;
    println!("\n** INPUT **");
    solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
