#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use std::fs;

use parse_display::{Display, FromStr};

// #[derive(Display, FromStr, PartialEq, Debug)]
// enum Direction {
//     #[display("forward")]
//     Forward,
// }

// #[derive(Debug, Display, FromStr)]
// #[display("{direction} {distance}")]
// struct Step {
//     direction: Direction,
//     distance: i32,
// }

#[derive(Debug, Display, FromStr)]
#[display("{from}-{to}")]
struct Assignment {
    from: i32,
    to: i32,
}
#[derive(Debug, Display, FromStr)]
#[display("{a},{b}")]
struct Pair {
    a: Assignment,
    b: Assignment,
}


fn solution(input_s: &str) -> Result<(String, String)> {
    let input: Vec<Pair> = input_s
        .trim()
        .split("\n")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_,_>>()?;

    let part1 = input.iter().filter(|p| {
        (p.a.from<=p.b.from && p.a.to>=p.b.to) 
        || (p.b.from<=p.a.from && p.b.to>=p.a.to)
    }).count();
    let part2 = input.iter().filter(|p| {
        (p.a.to >= p.b.from) && (p.a.from <= p.b.to)
        || (p.b.to >= p.a.from) && (p.b.from <= p.a.to)
    }).count();

    Ok((part1.to_string(), part2.to_string()))
}

#[test]
fn test_solution() -> Result<()> {
    let res=solution(&fs::read_to_string("test00.txt")?)?;
    assert!(res.0=="2");
    assert!(res.1=="4");
    Ok(())
}

fn main() -> Result<()> {
    let res=solution(&fs::read_to_string("input.txt")?)?;
    println!("Part 1: {}\nPart 2: {}", res.0, res.1);
    Ok(())
}
