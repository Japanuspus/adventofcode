use anyhow::{Result, Context};
use std::fs;

fn tri(v: i64) -> i64 {
    v*(v+1)/2
}

fn main() -> Result<()> {
    let input_s = fs::read_to_string("input.txt")?;
    let input: Vec<i64> = input_s
        .trim()
        .split(",")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_,_>>()?;

    let x1 = input.iter().min().unwrap();
    let x2 = input.iter().max().unwrap();
    let xp1 = (*x1..*x2).map(|x| (input.iter().map(|c| (c-x).abs()).sum::<i64>(), x)).min().unwrap();
    println!("Part 1: {}", xp1.0);

    let xp2 = (*x1..*x2).map(|x| (input.iter().map(|c| tri((c-x).abs())).sum::<i64>(), x)).min().unwrap();
    println!("Part 2: {}", xp2.0);
    Ok(())
}    