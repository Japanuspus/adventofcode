#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use std::{fs, time::Instant};

fn race(time: usize, dist: usize) -> usize {
    // c*(T-c)=d
    // c c - c T + d = 0
    // (c - T/2)**2 -T**2/4 + d = 0
    // c = T/2 +/- sqrt(T**2/4-d)

    let sd = (time as f64 * time as f64 / 4.0 - dist as f64).sqrt();
    let cmin: usize = ((time as f64 / 2.0) - sd).ceil() as usize;
    let cmax: usize = ((time as f64 / 2.0) + sd).floor() as usize;
    cmax - cmin + 1
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<Vec<i32>> = input_s
        .trim_end()
        .split("\n")
        .map(|s| {
            s.split_ascii_whitespace()
                .skip(1)
                .map(|n| n.parse())
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()?;
    // time, dist
    let races: Vec<(i32, i32)> = input[0]
        .iter()
        .cloned()
        .zip(input[1].iter().cloned())
        .collect();

    let mut part1: usize = 1;
    for (time, dist) in races {
        let n = (0..time)
            .filter(|charge| charge * (time - charge) > dist)
            .count();
        // let n2 = race(time as usize, dist as usize);
        // println!("{}, {}", n, n2);
        part1 *= n;
    }

    let input2: Vec<usize> = input_s
        .trim_end()
        .split("\n")
        .map(|s| {
            if let Some((_, ns)) = s.split_once(':') {
                ns.chars()
                    .filter(|c| !c.is_ascii_whitespace())
                    .collect::<String>()
                    .parse::<usize>()
                    .context("parsing number")
            } else {
                Err(anyhow!("Could not split"))
            }
        })
        .collect::<Result<_, _>>()?;
    // println!("{:?}", input2);

    let part2 = race(input2[0], input2[1]);

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "288");
    assert_eq!(res[1], "71503");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    for _ in 0..20 {
        solution(&input)?;
    } //warmup
    println!("Running");
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
