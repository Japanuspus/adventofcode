#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use vecmath::vec2_add;
use std::{fs, time::Instant, collections::BTreeSet};
use itertools::Itertools;

type V=[i16;2];

fn solution(input_s: &str) -> Result<[String; 2]> {
    let mut pmax: V = [0;2];
    let mut p0: V = [0;2];
    let mut rocks = BTreeSet::<V>::new();
    for (y,ln) in input_s.trim_end().split("\n").enumerate() {
        for (x, c) in ln.chars().enumerate() {
            pmax = [x as i16, y as i16];
            match c {
                'S' => {p0=pmax;},
                '#' => {rocks.insert(pmax);},
                _ => {},
            }
        }
    }
    let ds = [[0, -1], [-1, 0], [0, 1], [1,0]]; //nwse

    let mut v = BTreeSet::<V>::from_iter([p0,]);
    for _ in 0..64 {
        let mut v2 = BTreeSet::new();
        for p2 in v.iter().flat_map(|p| ds.iter().map(|d| vec2_add(*p, *d))) {
            if p2.iter().any(|v| *v<0) || p2.iter().zip(pmax.iter()).any(|(v, vmax)| v>vmax) {continue};
            if !rocks.contains(&p2) {
                v2.insert(p2);
            };
        };
        v=v2;
    }
    let part1 = v.len();
    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "0");
    assert_eq!(res[1], "0");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    let start = Instant::now();
    let (res, time) = loop { // run warmup for 100ms
        let lap = Instant::now();
        let res = solution(&input)?;
        if start.elapsed().as_millis()>100 {break (res, lap.elapsed())};
    };
    println!( "({} us)\nPart 1: {}\nPart 2: {}", time.as_micros(), res[0], res[1]);
    Ok(())
}
