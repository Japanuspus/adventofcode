#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use std::collections::{HashMap, HashSet};
use std::{fs, time::Instant};
use vecmath::{vec2_add, vec2_cast, vec2_scale};

fn solution(input_s: &str) -> Result<(String, String)> {
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

#[test]
fn test_solution() -> Result<()> {
    let res=solution(&fs::read_to_string("test00.txt")?)?;
    println!("Part 1: {}\nPart 2: {}", res.0, res.1);
    assert!(res.0=="21");
    assert!(res.1=="8");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    let start = Instant::now();
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}\nRuntime: {}us", res.0, res.1, start.elapsed().as_micros());
    Ok(())
}
