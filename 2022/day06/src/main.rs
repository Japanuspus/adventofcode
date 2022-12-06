#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use std::{fs, collections::{HashMap, HashSet}, str};
// use parse_display::{Display, FromStr};

// #[derive(Display, FromStr, PartialEq, Debug)]
// enum Direction {
//     #[display("forward")]
//     Forward,
// }

// #[derive(Debug, Display, FromStr)]
// #[display("{direction} {distance}")]
// struct Step {
//     direction: Direction,
//     distance: i32,
// }

fn solution(input_s: &str) -> Result<(String, String)> {
    // let input: Vec<i32> = input_s
    //     .trim()
    //     .split("\n")
    //     .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
    //     .collect::<Result<_,_>>()?;

    let part1 = input_s.as_bytes().windows(4).enumerate().filter_map(|(i, grp)|{
        if grp.iter().collect::<HashSet<_>>().len()==4 {Some(i+4)} else {None}
    }).next().unwrap();
    let part2 = input_s.as_bytes().windows(14).enumerate().filter_map(|(i, grp)|{
        if grp.iter().collect::<HashSet<_>>().len()==14 {Some(i+14)} else {None}
    }).next().unwrap();

    Ok((part1.to_string(), part2.to_string()))
}

#[test]
fn test_solution() -> Result<()> {
    let res=solution(&fs::read_to_string("test00.txt")?)?;
    println!("Part 1: {}\nPart 2: {}", res.0, res.1);
    assert!(res.0=="7");
    assert!(res.1=="0");
    Ok(())
}

fn main() -> Result<()> {
    let res=solution(&fs::read_to_string("input.txt")?)?;
    println!("Part 1: {}\nPart 2: {}", res.0, res.1);
    Ok(())
}
