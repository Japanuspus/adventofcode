#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use itertools::Itertools;
use std::{fs, time::Instant, collections::HashSet};
use vecmath::{vec2_add, vec2_scale, vec2_len};
use parse_display::{Display, FromStr};
 
#[derive(Display, FromStr, PartialEq, Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug, Display, FromStr)]
#[display("{direction} {distance}")]
struct Step {
    direction: Direction,
    distance: i32,
}

#[derive(Debug, Clone)]
struct Rope {
    head: [i32;2],
    tail: [i32;2],
}

impl Rope {
    fn new() -> Self {
        Rope {head: [0,0], tail: [0,0]}
    }
    fn step(&mut self, direction: &Direction) {
        let dh = match direction {
            Direction::U => [0,  1], 
            Direction::D => [0, -1], 
            Direction::L => [-1, 0], 
            Direction::R => [ 1, 0], 
        };
        self.head = vec2_add(self.head, dh);
        if (0..2).any(|i| (self.head[i]-self.tail[i]).abs()>1) {
            for i in 0..2 {
                let d = self.head[i]-self.tail[i];
                self.tail[i]+=d.signum();
            }    
        }
    }
}

fn solution(input_s: &str) -> Result<(String, String)> {
    let input: Vec<Step> = input_s
        .trim()
        .split("\n")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_,_>>()?;

    let tails: HashSet<[i32;2]> = input.iter()
    .flat_map(|mv| (0..mv.distance).map(|_| &mv.direction))
    .scan(Rope::new(), |rope, dir| {
        rope.step(dir);
        Some(rope.tail)
    }).collect();
    //.unique().count();
    for r in 0..5 {
        for c in 0..5 {
            print!("{}", if tails.contains(&[c, 4-r]) {'#'} else {'.'});
        }
        println!();
    }
    let part1 = tails.len();
    let part2 = 0;

    Ok((part1.to_string(), part2.to_string()))
}

#[test]
fn test_solution() -> Result<()> {
    let res=solution(&fs::read_to_string("test03.txt")?)?;
    println!("Part 1: {}\nPart 2: {}", res.0, res.1);
    assert!(res.0=="13");
    assert!(res.1=="1");

    let res=solution(&fs::read_to_string("test07.txt")?)?;
    println!("Part 1: {}\nPart 2: {}", res.0, res.1);
    assert!(res.1=="36");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    let start = Instant::now();
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}\nRuntime: {}us", res.0, res.1, start.elapsed().as_micros());
    Ok(())
}
