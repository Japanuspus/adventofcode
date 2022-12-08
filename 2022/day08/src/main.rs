#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use std::collections::{HashMap, HashSet};
use std::{fs, time::Instant};
// use ndarray::prelude::*;
// use ndarray::Array;

fn solution(input_s: &str) -> Result<(String, String)> {
    let input: Vec<Vec<u8>> = input_s
        .trim()
        .split("\n")
        .map(|s| s.as_bytes().iter().map(|b| b-b'0').collect())
        .collect();

    let n:usize = input.len();
    let m:usize = input[0].len();
    let mut visible: HashSet<(i32,i32)> = HashSet::new();
    let lanes = vec![
        ((0,0), (1, 0), n, (0, 1), m),
        ((0,0), (0, 1), m, (1, 0), n),
        (((n-1) as i32,(m-1) as i32), (-1,  0), n, ( 0, -1), m),
        (((n-1) as i32,(m-1) as i32), ( 0, -1), m, (-1,  0), n),
    ];
    for (p0,d1,n1,d2,n2) in lanes.iter() {
        for i1 in 0..*n1 {
            let mut h = -1i8;
            for i2 in 0..*n2 {
                let a1 = p0.0+d1.0*(i1 as i32)+d2.0*(i2 as i32);
                let a2 = p0.1+d1.1*(i1 as i32)+d2.1*(i2 as i32);
                let v = input[a1 as usize][a2 as usize] as i8;
                if v>h {
                    h=v;
                    visible.insert((a1, a2));
                }
            }
        }
    } 
    let part1 = visible.len();

    let mut scenic: HashMap<(i32,i32), usize> = HashMap::new();

    for (p0,d1,n1,d2,n2) in lanes.iter() {
        for i1 in 0..*n1 {
            let mut s: Vec<i8> = Vec::new(); 
            for i2 in 0..*n2 {
                let a1 = p0.0+d1.0*(i1 as i32)+d2.0*(i2 as i32);
                let a2 = p0.1+d1.1*(i1 as i32)+d2.1*(i2 as i32);
                let v = input[a1 as usize][a2 as usize] as i8;
                let mut visible_trees = s.iter().rev().take_while(|&t| t<&v).count();
                if visible_trees<s.len() {visible_trees+=1;}
                *(scenic.entry((a1, a2)).or_insert(1))*=visible_trees;

                //update s to be visible trees looking back
                // //s is a decreasing list
                // loop {
                //     if s.len()==0 || s.last().unwrap()>&v {break;}
                //     s.pop();
                // }
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
