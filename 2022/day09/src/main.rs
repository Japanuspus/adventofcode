#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use itertools::Itertools;
use std::{fs, time::Instant, collections::HashSet};
use vecmath::{vec2_add, vec2_scale, vec2_len, vec2_sub};
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

fn step(rope: &mut [[i32;2]], direction: &Direction) {
    let dh = match direction {
        Direction::U => [0,  1], 
        Direction::D => [0, -1], 
        Direction::L => [-1, 0], 
        Direction::R => [ 1, 0], 
    };
    rope[0] = vec2_add(rope[0], dh);
 
    // Move tails
    for h in 0..(rope.len()-1) {
        let t = h+1; 
        let d = vec2_sub(rope[h], rope[t]);
        if (0..2).any(|i| d[i] < -1 || d[i]>1) {
            for i in 0..2 {
                rope[h+1][i]+=d[i].signum();
            }    
        }    
    }
}

fn solve<const N: usize>(input: &Vec<Step>) -> usize {
    input.iter()
    .flat_map(|mv| (0..mv.distance).map(|_| &mv.direction))
    .scan([[0;2];N], |rope, dir| {
        step(rope, dir);
        Some(rope[N-1])
    }).unique().count()
}

fn solution(input_s: &str) -> Result<(String, String)> {
    let input: Vec<Step> = input_s
        .trim()
        .split("\n")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_,_>>()?;

    let part1 = solve::<2>(&input);
    let part2 = solve::<10>(&input);
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
