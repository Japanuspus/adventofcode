#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use itertools::{InterleaveShortest, iterate};
use std::collections::{HashMap, HashSet};
use std::{fs, time::Instant};
use vecmath::{vec2_add, vec2_cast, vec2_scale};
use rayon::prelude::*;

fn solution_0(input_s: &str) -> Result<(String, String)> {
    let input: Vec<Vec<i8>> = input_s
        .trim()
        .split("\n")
        .map(|s| s.as_bytes().iter().map(|b| (b-b'0') as i8).collect())
        .collect();

    let n = input.len() as i32;
    let m = input[0].len() as i32;
    let mut visible: HashSet<[i32;2]> = HashSet::new();
    let lanes = vec![
        ([0    ,0    ], [ 1,  0], n, [ 0,  1], m),
        ([0    ,0    ], [ 0,  1], m, [ 1,  0], n),
        ([(n-1),(m-1)], [-1,  0], n, [ 0, -1], m),
        ([(n-1),(m-1)], [ 0, -1], m, [-1,  0], n),
    ];
    for (p0,d1,n1,d2,n2) in lanes.iter() {
        for i1 in 0..*n1 {
            let mut h = -1i8;
            for i2 in 0..*n2 {
                let a = vec2_add(*p0, vec2_add(vec2_scale(*d1, i1), vec2_scale(*d2, i2)));
                let v = input[a[0] as usize][a[1] as usize];
                if v>h {
                    h=v;
                    visible.insert(a);
                }
            }
        }
    } 
    let part1 = visible.len();

    let mut scenic: HashMap<[i32;2], usize> = HashMap::new();
    for (p0,d1,n1,d2,n2) in lanes.iter() {
        for i1 in 0..*n1 {
            let mut s: Vec<i8> = Vec::new(); 
            for i2 in 0..*n2 {
                let a = vec2_add(*p0, vec2_add(vec2_scale(*d1, i1), vec2_scale(*d2, i2)));
                let v = input[a[0] as usize][a[1] as usize];
                let visible_trees = s.iter().rev().enumerate()
                    .find_map(|(i, &t)| if t>=v {Some(i+1)} else {None})
                    .unwrap_or_else(|| s.len());
                *(scenic.entry(a).or_insert(1))*=visible_trees;
                s.push(v);
            }
        }
    } 
    let part2 = scenic.values().max().unwrap();
    Ok((part1.to_string(), part2.to_string()))
}

// Handle each point independently to allow simple parallelization
// This has worse complexity than origininal solution, but might be faster.
fn solution_1(input_s: &str) -> Result<(String, String)> {
    let input: HashMap<[i32;2],i8> = input_s.trim().split("\n").enumerate()
        .flat_map(|(i, s)| 
            s.as_bytes().iter().enumerate()
            .map(move |(j, b)| ([i as i32, j as i32], (b-b'0') as i8))
        ).collect();
    let dirs = [[0,1], [1,0], [0,-1], [-1,0]];

    let part1 = input.par_iter().filter(|(&p0, &v)| { 
        dirs.iter().any(|d|
            // return true if visible in direction d 
            iterate(p0, |&p| vec2_add(p, *d))
            .skip(1)
            .map_while(|p| input.get(&p))
            .all(|&o| o<v)
        )
    }).count();    

    let part2: usize = input.par_iter().map(|(&p0, &v)| { 
        dirs.iter().map(|d|
            iterate(p0, |&p| vec2_add(p, *d))
            .skip(1).enumerate()
            .find_map(|(i, p)|{
                if let Some(o) = input.get(&p) {
                    if o<&v {None} else {Some(i+1)}
                } else {
                    Some(i)
                }
            })
            .unwrap()
        ).product()
    }).max().unwrap();    

    Ok((part1.to_string(), part2.to_string()))
}

type Solution = dyn Fn(& str) -> Result<(String, String)>;
const SOLUTIONS: [(&str, &Solution);2] = [
    ("Original", &solution_0), 
    ("Parallel", &solution_1),
];

#[test]
fn test_solution() -> Result<()> {
    for (_, solution) in SOLUTIONS {
        let res=solution(&fs::read_to_string("test00.txt")?)?;
        println!("Part 1: {}\nPart 2: {}", res.0, res.1);
        assert!(res.0=="21");
        assert!(res.1=="8");
    }
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    for (_, solution) in SOLUTIONS.iter().cycle().take(10) {solution(&input)?;} //warmup
    for (name, solution) in SOLUTIONS {
        let start = Instant::now();
        let res = solution(&input)?;
        println!("---\n{} ({} us)\nPart 1: {}\nPart 2: {}", name, start.elapsed().as_micros(), res.0, res.1, );
    }
    Ok(())
}
