#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use apply::Apply;
use itertools::Itertools;
use std::{fs, iter::once, collections::{HashSet, BTreeSet}, os::windows};

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
    range: [[i32;2];3],
}

impl Step {
    fn new(r: &ReadStep) -> Self {
        let state = match r.state {
            OnOff::On => true,
            OnOff::Off => false
        };
        let range = [
            [r.x.a, r.x.b],
            [r.y.a, r.y.b],
            [r.z.a, r.z.b],
        ];
        Step{state, range}
    }

    fn contains(&self, v: &[i32]) -> Option<bool> {
        if self.range.iter().zip(v).all(|([a, b], u)| a<=u && u<=b) {
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
        .map(|s| 
            s.parse::<ReadStep>()
            .and_then(|r| Ok(Step::new(&r)))
            .with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_,_>>()
}

fn part1(input: &Vec<Step>) -> usize {
    (0..3).map(|_| (-50i32..=50i32))
    .multi_cartesian_product()
    .filter(|v| input.iter().filter_map(|s| s.contains(&v)).last().unwrap_or(false))
    .count()
}


fn filter(v: Vec<&Step>, idx: usize) -> impl Iterator<Item=(usize, Vec<&Step>)> {
    let break_after: Vec<i32> = v
        .iter()
        .flat_map(|s| [s.range[idx][0]-1, s.range[idx][1]].into_iter())
        .collect::<BTreeSet<i32>>()
        .apply(|b_set| b_set.into_iter().collect());
    assert!(idx<3);
    // break_after.windows(2)
    (0..break_after.len().checked_sub(1).unwrap_or(0))
    .map(move |k| [break_after[k], break_after[k+1]])
    .map(move |u| (
        (u[1]-u[0]) as usize,
        v.iter().cloned().filter(|s| {let [a, b] = s.range[idx]; (a-1<=u[0]) && (b>=u[1])}).collect::<Vec<&Step>>()
    ))
}

fn part2(input: &Vec<Step>) -> usize {
    let mut p2 = 0usize;
    let v: Vec<&Step> = input.iter().collect();
    for (nx, vx) in filter(v, 0) {
        for (ny, vxy) in filter(vx, 1) {
            for (nz, vxyz) in filter(vxy, 2) {
                if vxyz.iter().rev().nth(0).and_then(|s| Some(s.state)).unwrap_or(false) {p2+=nx*ny*nz} else {}
            }
        }
    };
    p2
}


fn solve(n: &str) -> Result<()> {
    println!("** {} **", n);
    let input = parse(&fs::read_to_string(n)?)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn main() -> Result<()> {
    solve("test01.txt")?;
    solve("test02.txt")?;
    // runtime is really bad without --release
    solve("input.txt")?;
    Ok(())
}
