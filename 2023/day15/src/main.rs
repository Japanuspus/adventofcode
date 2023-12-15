#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use std::{fs, time::Instant, collections::HashMap};

fn hash(v: &[u8]) -> u8 {
    v.iter()
    .fold(0u8, |hash, c| (hash.wrapping_add(*c)).wrapping_mul(17))
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let part1: usize = input_s
        .trim_end()
        .split(",")
        .map(|v| hash(v.as_bytes()) as usize) 
        .sum();

    let mut boxes: Vec<HashMap<&str, (u32, u8)>> = (0..256).map(|_| HashMap::new()).collect();
    for (op_idx, op) in input_s.trim_end().split(",").enumerate() {
        let (label, lens) = op.split_once(['-', '=']).ok_or_else(|| anyhow!("Bad opstr: {}", op))?;
        let bb = &mut boxes[hash(label.as_bytes()) as usize];
        if let Some(lens_idx)=lens.chars().next().and_then(|c| c.to_digit(10)) {
            //insert/replace
            bb.entry(label)
            .or_insert((op_idx as u32, 0)).1=lens_idx as u8;
        } else {
            //remove
            bb.remove(label);
        }
    }

    let part2: usize = boxes
    .iter()
    .enumerate()
    .flat_map(|(ib, b)| 
        b
        .values()
        .sorted()
        .enumerate()
        .map(move |(il, (_, lens))| (ib+1)*(il+1)*(*lens as usize))
    ).sum();

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "1320");
    assert_eq!(res[1], "145");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    for _ in 0..20 {
        solution(&input)?;
    } //warmup
    let start = Instant::now();
    let res = solution(&input)?;
    println!(
        "({} us)\nPart 1: {}\nPart 2: {}",
        start.elapsed().as_micros(),
        res[0],
        res[1],
    );
    Ok(())
}
