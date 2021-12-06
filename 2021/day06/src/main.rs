use anyhow::{Result, Context};
use std::fs;


fn part1(input: &[usize], steps: usize) -> usize {
    let mut counts: [usize;9] = [0; 9];
    for f in input.iter() {counts[*f]+=1;}
    for _ in 0..steps {
        counts.rotate_left(1);
        counts[6]+=counts[8];
    }
    counts.iter().sum::<usize>()
}

fn main() -> Result<()> {
    let input_s = fs::read_to_string("input.txt")?;
    let input: Vec<usize> = input_s
        .trim()
        .split(",")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_,_>>()?;

    println!("Part 1: {}", part1(&input, 80));
    println!("Part 2: {}", part1(&input, 256));
    Ok(())
}    