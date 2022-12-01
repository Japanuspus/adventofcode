#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use itertools::Itertools;
use std::fs;

fn solution(input_s: &str) -> Result<()> {
    let input: Vec<Vec<i32>> = input_s
        .trim()
        .split("\n\n")
        .map(|s| s.split("\n").map(|l| l.parse()).collect::<Result<_, _>>())
        .collect::<Result<_, _>>()?;

    let part1: i32 = input.iter().map(|e| e.iter().sum()).max().unwrap();
    println!("Part 1: {}", part1);

    let part2: i32 = input
        .iter()
        .map(|e| e.iter().sum::<i32>())
        .sorted()
        .rev()
        .take(3)
        .sum();
    println!("Part 2: {}", part2);
    Ok(())
}

fn solution2(input_s: &str) -> Result<()> {
    let elfs: Vec<i32> = itertools::unfold(
        input_s.trim().split("\n").map(|l| l.parse::<i32>().ok()),
        |lines| lines.map_while(|v| v).reduce(|acc, v| acc + v),
    )
    .sorted()
    .rev()
    .collect();

    println!("Part 1: {}", elfs[0]);
    println!("Part 2: {}", elfs.iter().take(3).sum::<i32>());
    Ok(())
}

fn main() -> Result<()> {
    solution(&fs::read_to_string("input.txt")?)?;
    println!("*** Take 2***");
    solution2(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
