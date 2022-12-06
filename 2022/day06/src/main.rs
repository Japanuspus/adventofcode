#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use std::{fs, collections::{HashSet}, str};

fn find_marker(s: &str, n: usize) -> Option<usize> {
    s.as_bytes().windows(n).enumerate().find_map(|(i, grp)|{
        if grp.iter().collect::<HashSet<_>>().len()==n {Some(i+n)} else {None}
    })
}

fn solution(input_s: &str) -> Result<(String, String)> {
    Ok((
        find_marker(input_s, 4).unwrap().to_string(),
        find_marker(input_s, 14).unwrap().to_string()
    ))
}

#[test]
fn test_solution() -> Result<()> {
    let res=solution(&fs::read_to_string("test00.txt")?)?;
    println!("Part 1: {}\nPart 2: {}", res.0, res.1);
    assert!(res.0=="7");
    assert!(res.1=="19");
    Ok(())
}

fn main() -> Result<()> {
    let res=solution(&fs::read_to_string("input.txt")?)?;
    println!("Part 1: {}\nPart 2: {}", res.0, res.1);
    Ok(())
}
