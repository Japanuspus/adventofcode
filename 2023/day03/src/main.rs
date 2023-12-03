#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs, iter,
    time::Instant,
};

fn solution(input_s: &str) -> Result<[String; 2]> {
    let symbols: HashMap<(i32, i32), char> = input_s
        .trim_end()
        .split("\n")
        .enumerate()
        .flat_map(|(ln, s)| {
            s.chars()
                .enumerate()
                .filter(|(_, c)| !(*c == '.' || c.is_ascii_digit()))
                .map(move |(i, c)| ((i as i32, ln as i32), c))
        })
        .collect();

    let re = Regex::new(r"\d+")?;
    let mut gears: HashMap<(i32, i32), isize> = HashMap::new();
    let mut part1: isize = 0;
    // negative: one nb. positive: product. zero: more than two nbs
    for (ln, m) in input_s
        .trim_end()
        .split("\n")
        .enumerate()
        .flat_map(|(ln, s)| re.find_iter(s).map(move |m| (ln as i32, m)))
    {
        let mut has_nb: bool = false;
        let val = m.as_str().parse::<isize>()?; //? how to make lazy?
        for nb_key in ((ln - 1)..(ln + 2)).flat_map(move |nln| {
            ((m.start() as i32 - 1)..(m.end() as i32 + 1)).map(move |x| (x, nln))
        }) {
            if let Some(c) = symbols.get(&nb_key) {
                has_nb = true;
                if c == &'*' {
                    gears
                        .entry(nb_key)
                        .and_modify(|v| *v = if *v < 0 { -*v * val } else { 0 })
                        .or_insert(-val);
                }
            }
        }
        if has_nb {
            part1 += val
        }
    }
    let part2: isize = gears.values().filter(|&v| *v > 0).sum();

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "4361");
    assert_eq!(res[1], "467835");
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
