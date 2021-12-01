use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input_s = fs::read_to_string("input.txt")?;
    let input: Vec<usize> = input_s
        .split("\n")
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    let p1: usize = input.windows(2).filter(|ab| ab[1] > ab[0]).count();
    println!("Part 1: {}", p1);

    // `.window` is a slice function, so collect the windows sums to buffer
    let input_w: Vec<usize> = input.windows(3).map(|w| w.iter().sum()).collect();
    let p2: usize = input_w.windows(2).filter(|ab| ab[1] > ab[0]).count();

    println!("Part 2: {}", p2);
    Ok(())
}
