#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use apply::Apply;
use itertools::Itertools;
use std::{fs, iter::{once, Scan}, collections::{HashSet, BTreeSet}, os::windows};

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

#[derive(Debug, Display, FromStr)]
#[display("{state} x={x},y={y},z={z}")]
struct ReadStep {
    state: OnOff,
    x: Range,
    y: Range,
    z: Range,
}

#[derive(Debug, Clone)]
struct Step {
    state: bool,
    order: u16,
    range: [[i32;2];3],
}

impl Step {
    fn new(r: &ReadStep, order: u16) -> Self {
        let state = match r.state {
            OnOff::On => true,
            OnOff::Off => false
        };
        let range = [
            [r.x.a, r.x.b],
            [r.y.a, r.y.b],
            [r.z.a, r.z.b],
        ];
        Step{state, range, order}
    }

    fn contains(&self, v: &[i32]) -> Option<bool> {
        if self.range.iter().zip(v).all(|([a, b], u)| a<=u && u<=b) {
            Some(self.state)
        } else {
            None
        }
    }

    // v is a windows from breaks_after: perfect overlap is a-1, b
    fn contains_range(&self, v: &Vec<&[i32]>) -> Option<bool> {
        if self.range.iter().zip(v).all(|([a, b], u)| 
            (a-1<=u[0]) && (b>=&u[1])
        ) {
            Some(self.state)
        } else {
            None
        }
    }
}

fn parse(input_s: &str) -> Result<Vec<Step>> {
    input_s
        .trim()
        .split("\n")
        .enumerate()
        .map(|(i, s)| 
            s.parse::<ReadStep>()
            .and_then(|r| Ok(Step::new(&r, i as u16)))
            .with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_,_>>()
}

fn part1(input: &Vec<Step>) -> usize {
    (0..3).map(|_| (-50i32..=50i32))
    .multi_cartesian_product()
    .filter(|v| input.iter().filter_map(|s| s.contains(&v)).last().unwrap_or(false))
    .count()
}

struct Scanline <'a> {
    idx: usize,
    all: Vec<&'a Step>,
    active: Vec<&'a Step>,
}

impl <'a> Scanline<'a> {
    fn new(idx: usize, v: &'a [Step]) -> Self {
        let all: Vec<&'a Step>=v.iter().sorted_by_key(|s| s.range[idx][0]).collect();
        let active = Vec::new();
        Self{all, active, idx}
    }
}

impl <'a> Iterator for Scanline<'a> {
    type Item = ((i32, i32), Vec<&'a Step>);
    fn next(&mut self) -> Option<Self::Item> {
        let next_close = self.active.iter().nth(0).and_then(|a| Some(a.range[self.idx][1]));
        let next_open = self.all.iter().nth(0).and_then(|a| Some(a.range[self.idx][0]));
        match (next_close, next_open) {
            (None, None) => None,
            _ => None,
        }
    }
}

fn part2_brute(input: &Vec<Step>) -> usize {
    let break_after: Vec<Vec<i32>> = (0..3).map(|i| {
        input
        .iter()
        .flat_map(|s| [s.range[i][0]-1, s.range[i][1]].into_iter())
        .collect::<BTreeSet<i32>>()
        .apply(|b_set| b_set.into_iter().collect())
    }).collect();
    break_after.iter()
    .map(|breaks| breaks.windows(2))
    .multi_cartesian_product()
    .map(|rs| 
        if input.iter().rev().filter_map(|step| step.contains_range(&rs)).nth(0).unwrap_or(false) {
            rs.iter().map(|ab| (ab[1]-ab[0]) as usize).product()
        } else {
            0usize
        })
    .sum()
}

fn solve(n: &str) -> Result<()> {
    println!("** {} **", n);
    let input = parse(&fs::read_to_string(n)?)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2 brute: {}", part2_brute(&input));
    Ok(())
}

fn main() -> Result<()> {
    solve("test01.txt")?;
    solve("test02.txt")?;
    // runtime is really bad!
    solve("input.txt")?;
    Ok(())
}
