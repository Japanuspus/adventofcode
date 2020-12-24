use std::{collections::{HashMap, HashSet}, fs, num::NonZeroU16};
use anyhow::Result;
// use anyhow::Context;
// use itertools::Itertools;
// use parse_display::{FromStr};
use regex::Regex;
use apply::Apply;
// use num::{BigInt, Integer};
use vecmath::vec3_add;
//#[derive(Debug, FromStr)]
//#[display("{key}:{value}")]
//struct InputItem {key: String, value: String}
// https://www.redblobgames.com/grids/hexagons/
type XYZ = [i16; 3];

fn parse_steps(s: &str) -> Result<Vec<Vec<XYZ>>> {
    let re = Regex::new(r"(e)|(se)|(sw)|(w)|(nw)|(ne)")?;
    s.lines().map(|ln|{ re.find_iter(ln).map(|dd| match dd.as_str() {
        "e" => [1,-1,0],
        "ne" => [1,0,-1],
        "nw" => [0,1,-1],
        "w" => [-1,1,0],
        "sw" => [-1,0,1],
        "se" => [0,-1,1],
        _ => panic!("Unknown direction"),
    }).collect()
    }).collect::<Vec<Vec<XYZ>>>().apply(Ok)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let flips = parse_steps(&input)?;
    
    let mut flipcount:HashMap<XYZ, u16> = HashMap::new();
    for line in flips.iter() {
        let f = line.iter().fold([0,0,0], |acc, step| vec3_add(acc, step.clone()));
        flipcount.entry(f).and_modify(|e| *e+=1).or_insert(1);
    }

    println!("Part 1: {}", flipcount.iter().filter(|(_, ct)| (*ct % 2)==1).count());

    Ok(())
}
