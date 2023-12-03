#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::{fs, time::Instant, collections::{HashMap, HashSet, btree_map::{VacantEntry, OccupiedEntry}}, sync::Arc, iter};

fn solution(input_s: &str) -> Result<[String; 2]> {
    let symbols: HashMap<(i32, i32), char> = input_s
        .trim_end()
        .split("\n")
        .enumerate()
        .flat_map(|(ln, s)| {
            s.chars().enumerate()
            .filter(|(_, c)| !(*c=='.' || c.is_ascii_digit()))
            .map(move |(i, c)| ((i as i32, ln as i32), c))
        }).collect();
    
    let re = Regex::new(r"\d+")?;
    let mut part1:usize = 0;
    for (ln, s) in input_s.trim_end().split("\n").enumerate() {
        let ln=ln as i32;
        for m in re.find_iter(s) {
            if (-1i32..2).any(|dl| 
                ((m.start() as i32-1)..(m.end() as i32+1))
                .any(|x| symbols.contains_key(&(x, ln+dl))))
            {
                part1 += m.as_str().parse::<usize>()?;
            }  
        }        
    }

    let mut gears: HashMap<(i32, i32), isize> = HashMap::new();
    // negative: one nb. positive: product. zero: more than two nbs
    for (ln, s) in input_s.trim_end().split("\n").enumerate() {
        let ln=ln as i32;
        for m in re.find_iter(s) {
            for dl in -1i32..2 {
                for x in (m.start() as i32-1)..(m.end() as i32+1) {
                    let key = (x, ln+dl);
                    if let Some(c) = symbols.get(&key) {
                        if c==&'*' {
                            let val = m.as_str().parse::<usize>()? as isize;
                            gears.entry(key)
                            .and_modify(|v| *v = if *v<0 {-*v*val} else {0})
                            .or_insert(-val);                           
                        }
                    }
                }
            }
        }
    }
    let part2: isize = gears.values().filter(|&v| *v>0).sum();

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "4361");
    assert_eq!(res[1], "467835");
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
