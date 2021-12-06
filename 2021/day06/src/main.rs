use anyhow::{Result, Context};
use std::fs;


fn main() -> Result<()> {
    let input_s = fs::read_to_string("input.txt")?;
    let input: Vec<usize> = input_s
        .trim()
        .split(",")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_,_>>()?;


    let mut counts: [usize;9] = [0; 9];
    for f in input.iter() {counts[*f]+=1;}
    for _ in 0..80 {
        let mut new_counts: [usize;9] = [0; 9];
        new_counts[0] = counts[1];
        for v in 0usize..8usize {
            new_counts[v] = counts[v+1];
        }
        new_counts[6]+=counts[0];
        new_counts[8]+=counts[0];
        counts=new_counts;
    }
    println!("Part 1: {}", counts.iter().sum::<usize>());


    let mut counts: [usize;9] = [0; 9];
    for f in input.iter() {counts[*f]+=1;}
    for _ in 0..256 {
        let mut new_counts: [usize;9] = [0; 9];
        new_counts[0] = counts[1];
        for v in 0usize..8usize {
            new_counts[v] = counts[v+1];
        }
        new_counts[6]+=counts[0];
        new_counts[8]+=counts[0];
        counts=new_counts;
    }
    println!("Part 2: {}", counts.iter().sum::<usize>());
    Ok(())
}    