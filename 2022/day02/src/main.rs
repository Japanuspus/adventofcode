#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use std::fs;

fn solution(input_s: &str) -> Result<()> {
    let input: Vec<(i32, i32)> = input_s
        .trim()
        .split("\n")
        .map(|s| (
            (s.as_bytes()[0] as i32)-(b'A' as i32), 
            (s.as_bytes()[2] as i32)-(b'X' as i32)
        ))
        .collect();

    // Rock: 0, Paper: 1, Scissor: 2
    // (b-a+1).rem_euclid(3)-1 : 0 on tie, 1 if b wins, -1 if a wins

    let part1:i32 = input.iter().map(|(a, b)| (b+1) + 3*(b-a+1).rem_euclid(3)).sum();
    let part2:i32 = input.iter().map(|(a, x)| ((a+x-1).rem_euclid(3)+1) + 3*x).sum();
    
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
