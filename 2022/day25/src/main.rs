#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use std::{fs, time::Instant};

fn parse_char(c: char) -> isize {
    match c {
        '-' => -1,
        '=' => -2,
        o => ((o as u8)-b'0') as isize,
    }
}

fn parse_snafu(s: &str) -> isize {
    s.chars().fold(0, |acc, c| acc*5 + parse_char(c))
}

const SNAFU_DIGITS: [char;5] = ['=', '-', '0', '1', '2'];

fn snafu_to_string(v: isize) -> String {
    let digit_indices_rev = itertools::unfold(v, |va| {
        if *va>0 {
            let vp = *va+2;
            *va = vp / 5;
            Some(vp % 5)
        } else {
            None
        }
    }).collect::<Vec<isize>>();
    digit_indices_rev.iter().rev().map(|i| SNAFU_DIGITS[*i as usize]).collect()
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let part1_num: isize = input_s.trim_end()
        .split("\n")
        .map(|s| parse_snafu(s))
        .sum();

    let part1 = snafu_to_string(part1_num);
    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "2=-1=0");
    assert!(res[1] == "0");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    for _ in 0..20 {solution(&input)?;} //warmup
    let start = Instant::now();
    let res = solution(&input)?;
    println!(
        "({} us)\nPart 1: {}\nPart 2: {}",
        start.elapsed().as_micros(), res[0], res[1],
    );
    Ok(())
}