#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use std::{fs, time::Instant};

fn sensor_next(s: &Vec<i64>) -> i64 {
    if s.iter().all(|v| *v==0) {
        0
    } else {
        s[s.len()-1]+sensor_next(&s.windows(2).map(|ss| ss[1]-ss[0]).collect::<Vec<_>>())
    }
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<Vec<i64>> = input_s
        .trim_end()
        .split("\n")
        .map(|s| s.split_ascii_whitespace()
            .map(|n| n.parse())
            .collect::<Result<_,_>>()
            .with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_, _>>()?;

    let part1: i64 = input.iter().map(sensor_next).sum();
    let part2: i64 = input.iter().map(|v| sensor_next(&v.iter().rev().cloned().collect::<Vec<_>>())).sum();

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "114");
    assert_eq!(res[1], "2");
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
