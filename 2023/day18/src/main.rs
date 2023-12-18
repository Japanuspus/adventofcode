#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use vecmath::{vec2_add, vec2_scale};
use std::{fs, time::Instant, collections::HashSet};
use itertools::Itertools;

use parse_display::{Display, FromStr};

// L 6 (#2d8140)
#[derive(Debug, Display, FromStr)]
#[display("{direction} {distance} (#{hex})")]
struct Edge {
    direction: char,
    distance: i64,
    hex: String,
}

type V=[i64;2];

fn greens_area(input: impl Iterator<Item=(V, i64)>) -> i64 {
    // Using greens theorem 2*a = sum x dy - y dx

    let (a2, _) = input
    .fold((0, [0,0]), |(a2, p), (d, n)| (
        //includes `n` to count the half unit outside edge
        a2 + n + n*(p[0]*d[1]-p[1]*d[0]), 
        vec2_add(p, vec2_scale(d, n)),
    ));

    // There are 4 more outside than inside corners:
    // exactly one unit is not counted if adding 1/2 * edge len
    1+a2/2
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<Edge> = input_s
        .trim_end()
        .split("\n")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_, _>>()?;

    let part1 = greens_area(input.iter().map(|e| (
        match e.direction {
            'U' => [ 0, -1], 'D' => [ 0,  1], 'L' => [-1,  0], 'R' => [ 1,  0],
            _ => panic!("Bad direction"),
        },
        e.distance,
    )));

    //0 means R, 1 means D, 2 means L, and 3 means U.
    let dirs = [[1,0],[0,1],[-1,0],[0,-1]];
    let part2 = greens_area(input.iter().map(|e| {
        let s=&e.hex;
        let d=dirs[(s.as_bytes()[5]-b'0') as usize];
        let n=i64::from_str_radix(&s[..5], 16).unwrap();
        (d, n)
    }));

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "62");
    assert_eq!(res[1], "952408144115");
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
