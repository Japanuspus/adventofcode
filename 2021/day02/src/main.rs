use anyhow::{Result, Context};
use std::fs;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display(style="lowercase")]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug, FromStr)]
#[display("{direction} {distance}")]
struct Step {
    direction: Direction,
    distance: i32,
}

fn main() -> Result<()> {
    let input: Vec<Step> = fs::read_to_string("input.txt")?
        .split("\n")
        .filter(|s| s.len()>0)
        .map(|s| s.parse().with_context(|| format!("Failed to parse >{}<", s)))
        .collect::<Result<_,_>>()?;

    let mut x = 0i32;
    let mut y = 0i32;
    for s in input.iter() {
        match s.direction {
            Direction::Down => {y+=s.distance;},
            Direction::Up => {y-=s.distance;},
            Direction::Forward => {x+=s.distance;},
        }
    }
    println!("Part 1: {}", x*y);

    let mut aim = 0i32;
    let mut depth = 0i32;
    let mut x = 0i32;
    for s in input.iter() {
        match s.direction {
            Direction::Down => {aim+=s.distance;},
            Direction::Up => {aim-=s.distance;},
            Direction::Forward => {
                x+=s.distance;
                depth+=aim*s.distance;
            },
        }
    }
    println!("Part 1: {}", depth*x);

    Ok(())
}