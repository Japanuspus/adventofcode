#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use std::fs;
use std::collections::HashSet;
use itertools::Itertools;

fn priority(c: u8) -> u8 {
    if c>=b'a' {1+c-b'a'} else {27+c-b'A'}
}

fn solution(input_s: &str) -> Result<(String, String)> {
    let part1: usize = input_s
        .trim()
        .split("\n")
        .map(|s| s.as_bytes())
        .map(|s| {
            let n = s.len()/2;
            let c1 = s[..n].iter().cloned().collect::<HashSet<_>>();
            let c2 = s[n..].iter().cloned().collect::<HashSet<_>>();
            priority(c1.intersection(&c2).cloned().next().unwrap())
        } as usize)
        .sum();

    let part2: usize = input_s
        .trim()
        .split("\n")
        .map(|s| s.as_bytes().iter().cloned().collect::<HashSet<_>>())
        .tuples::<(_,_,_)>()
        .map(|(a,b,cs)| {
            let ab = a.intersection(&b).cloned().collect::<HashSet<_>>();
            priority(ab.intersection(&cs).cloned().next().unwrap())
        } as usize)
        .sum();

        Ok((part1.to_string(), part2.to_string()))
}

#[test]
fn test_solution() -> Result<()> {
    let res=solution(&fs::read_to_string("test00.txt")?)?;
    println!("Part 1: {}\nPart 2: {}", res.0, res.1);
    assert!(res.0=="157");
    assert!(res.1=="70");
    Ok(())
}

fn main() -> Result<()> {
    let res=solution(&fs::read_to_string("input.txt")?)?;
    println!("Part 1: {}\nPart 2: {}", res.0, res.1);
    Ok(())
}
