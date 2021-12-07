use anyhow::{Context, Result};
use std::fs;

fn main() -> Result<()> {
    let input_s = fs::read_to_string("input.txt")?;
    let input: Vec<i64> = input_s
        .trim()
        .split(",")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_, _>>()?;

    let x1 = input.iter().min().unwrap();
    let x2 = input.iter().max().unwrap();
    let p1 = (*x1..=*x2)
        .map(|x| input.iter().map(|c| (c - x).abs()).sum::<i64>())
        .min()
        .unwrap();
    println!("Part 1: {}", p1);

    let p2 = (*x1..=*x2)
        .map(|x| {
            input
                .iter()
                .map(|c| {
                    let n = (c - x).abs();
                    n * (n + 1) / 2
                })
                .sum::<i64>()
        })
        .min()
        .unwrap();
    println!("Part 2: {}", p2);
    Ok(())
}
