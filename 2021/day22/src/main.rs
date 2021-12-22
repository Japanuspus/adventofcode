#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use apply::Apply;
use itertools::Itertools;
use std::fs;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display(style="lowercase")]
enum OnOff {
    On,
    Off,
}

#[derive(Debug, Display, FromStr)]
#[display("{a}..{b}")]
struct Range {
    a: i32,
    b: i32,
}

impl Range {
    fn contains(&self, v: i32) -> bool {
        v>=self.a && v<=self.b
    }
}

#[derive(Debug, Display, FromStr)]
#[display("{state} x={x},y={y},z={z}")]
struct Step {
    state: OnOff,
    x: Range,
    y: Range,
    z: Range,
}

impl Step {
    fn contains(&self, v: &[i32]) -> Option<bool> {
        if self.x.contains(v[0]) && self.y.contains(v[1]) && self.z.contains(v[2]) {
            match self.state {
                OnOff::On => true,
                OnOff::Off => false
            }
            .apply(Some)
        } else {
            None
        }
    }
}

fn solution(input_s: &str) -> Result<()> {
    let input: Vec<Step> = input_s
        .trim()
        .split("\n")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_,_>>()?;

    //brute force part 1
    let p1 = (0..3).map(|_| (-50i32..=50i32))
    .multi_cartesian_product()
    .filter(|v| input.iter().filter_map(|s| s.contains(&v)).last().unwrap_or(false))
    .count();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", 0);
    Ok(())
}

fn main() -> Result<()> {
    println!("** TEST **");
    //solution(&fs::read_to_string("test00.txt")?)?;
    println!("\n** INPUT **");
    solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
