use std::{collections::VecDeque, fs};
use anyhow::Result;
// use itertools::Itertools;
// use anyhow::Context;
// use itertools::Itertools;
// use parse_display::{FromStr};
// use regex::Regex;
// use apply::Also;
// use num::{BigInt, Integer};

//#[derive(Debug, FromStr)]
//#[display("{key}:{value}")]
//struct InputItem {key: String, value: String}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let decks: Vec<VecDeque<usize>> = input.split("\n\n").map(|pt| {
        println!("{:?}", pt);
        pt.lines().skip(1).filter_map(|ln| ln.parse::<usize>().ok()).collect()
    }).collect();

    let mut p1 = decks[0].clone();
    let mut p2 = decks[1].clone();
    while (p1.len()>0) & (p2.len()>0) {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        if c1>c2 {
            p1.push_back(c1); p1.push_back(c2);
        } else {
            p2.push_back(c2); p2.push_back(c1);
        }
    }

    let w = if p1.len()==0 {&p2} else {&p1};
    println!("Part 1: {}", w.iter().rev().enumerate().map(|(i, c)| (i+1)*c).sum::<usize>());

    Ok(())
}
