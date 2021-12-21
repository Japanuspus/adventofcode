#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use std::{fs, iter::Cycle};

fn solution(input_s: &str) -> Result<()> {
    let input: Vec<u32> = input_s
        .trim()
        .split("\n")
        .map(|s| {let (_, b) = s.split_once(": ").unwrap(); b.parse().with_context(|| format!("Parsing {}", s))})
        .collect::<Result<_>>()?;

    let mut die = (1..=100u32).cycle();
    let mut die_count: usize = 0;
    let mut pos_minus_1: [u32; 2] = [input[0]-1, input[1]-1];
    let mut scores: [usize; 2] = [0;2];
    'outer: loop {
        for i in 0..2 {
            let roll = die.by_ref().take(3).sum::<u32>();
            die_count+=3;
            let p = (pos_minus_1[i]+roll) % 10;
            pos_minus_1[i] = p;
            scores[i]+=(p+1) as usize;
            if scores[i]>=1000 {break 'outer}
            //println!("Player {} roll {} new_pos {} score{}", i, &roll, pos_minus_1[i]+1, scores[i]);
        }
    }
    println!("Part 1: {}", die_count*scores.iter().min().unwrap());
    println!("Part 2: {}", 0);
    Ok(())
}

fn main() -> Result<()> {
    println!("** TEST **");
    solution(&fs::read_to_string("test00.txt")?)?;
    println!("\n** INPUT **");
    solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
