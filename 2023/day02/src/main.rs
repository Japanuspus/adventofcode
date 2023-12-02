#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use std::{fs, time::Instant};

//Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

fn parse_reveal(s: &str) -> Result<[u8; 3]> {
    let mut res = [0; 3];
    for v in s.split(", ") {
        if let Some((vs, c)) = v.split_once(' ') {
            let v = vs.parse::<u8>()?;
            res[match c {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => return Err(anyhow!("Unknown color")),
            }] = v;
        }
    }
    Ok(res)
}

struct Game {
    id: usize,
    reveals: Vec<[u8; 3]>,
}

fn parse_game(s: &str) -> Result<Game> {
    if let Some((game, r)) = s.split_once(": ") {
        let id = game[5..].parse()?;
        let reveals = r
            .split("; ")
            .map(|s| parse_reveal(s))
            .collect::<Result<_, _>>()?;
        return Ok(Game { id, reveals });
    }
    Err(anyhow!("No colon in game"))
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<Game> = input_s
        .trim_end()
        .split("\n")
        .map(|s| parse_game(s).with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_, _>>()?;

    let max1 = [12, 13, 14];
    let part1: usize = input
        .iter()
        .filter_map(|g| {
            if g.reveals
                .iter()
                .all(|r| r.iter().zip(max1.iter()).all(|(l, r)| l <= r))
            {
                //println!("+ {}: {:?}", g.id, g.reveals);
                Some(g.id)
            } else {
                //println!("- {}: {:?}", g.id, g.reveals);
                None
            }
        })
        .sum();

    let part2: usize = input
        .iter()
        .map(|g| {
            let mut m = g.reveals[0];
            for r in &g.reveals[1..] {
                for i in 0..3 {
                    if r[i] > m[i] {
                        m[i] = r[i]
                    }
                }
            }
            m.iter().map(|v| *v as usize).product::<usize>()
        })
        .sum();

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "8");
    assert_eq!(res[1], "2286");
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

