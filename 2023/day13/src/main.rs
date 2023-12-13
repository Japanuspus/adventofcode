#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use ndarray::{Array2, ArrayView2, Zip};
use std::{fs, time::Instant};

fn row_sym(m: ArrayView2<u8>) -> Option<usize> {
    (1..m.nrows()).find(|&i| {
        (0..i)
            .rev()
            .zip(i..m.nrows())
            .all(|(i1, i2)| m.row(i1) == m.row(i2))
    })
}

fn smudged_row_sym(m: ArrayView2<u8>) -> Option<usize> {
    (1..m.nrows()).find(|&i| {
        let mut smudged = false;
        for (i1, i2) in (0..i).rev().zip(i..m.nrows()) {
            let n_diff: usize = Zip::from(m.row(i1))
                .and(m.row(i2))
                .fold(0, |acc, a, b| acc + if a != b { 1 } else { 0 });
            if n_diff == 0 {
                continue;
            }
            if smudged || n_diff > 1 {
                return false;
            }
            smudged = true;
        }
        smudged
    })
}

fn part1(m: &Array2<u8>) -> usize {
    row_sym(m.view())
        .and_then(|v| Some(100 * v))
        .or_else(|| row_sym(m.t()))
        .unwrap_or(0)
}

fn part2(m: &Array2<u8>) -> usize {
    smudged_row_sym(m.view())
        .and_then(|v| Some(100 * v))
        .or_else(|| smudged_row_sym(m.t()))
        .unwrap_or(0)
}

fn parse_array(s: &str) -> Result<Array2<u8>> {
    let mut nrows: usize = 0;
    let cs: Vec<u8> = s
        .split("\n")
        .inspect(|_| nrows += 1)
        .flat_map(|ln| ln.as_bytes().iter())
        .cloned()
        .collect();
    let ncols = cs.len() / nrows;
    Array2::from_shape_vec((nrows, ncols), cs).context("Parsing map array")
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<Array2<_>> = input_s
        .trim_end()
        .split("\n\n")
        .map(|s| parse_array(s))
        .collect::<Result<_>>()?;

    let part1: usize = input.iter().map(|m| part1(m)).sum();
    let part2: usize = input.iter().map(|m| part2(m)).sum();

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "405");
    assert_eq!(res[1], "400");
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
