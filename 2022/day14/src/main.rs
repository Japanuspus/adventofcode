#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use itertools::Itertools;
use std::collections::HashSet;
use std::{fs, time::Instant};

use nom;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::{Finish, IResult};



fn parse_point(s: &str) -> IResult<&str, [i16;2]> {
    let (rest, (a, b)) = nom::sequence::separated_pair(
        nom::character::complete::i16, 
        char(','),
        nom::character::complete::i16
    )(s)?;
    Ok((rest,[a,b]))
}

fn parse_line(s: &str) -> IResult<&str, Vec<[i16;2]>> {
    nom::multi::separated_list0(tag(" -> "), parse_point)(s)
}

fn parse(s: &str) -> HashSet<[i16;2]> {
    s.trim_end()
    .split("\n")
    .map(|s| parse_line(s).unwrap().1)
    .flat_map(|ln| 
        ln.into_iter().tuple_windows().flat_map(|(p0, p1)| {
            let dtot = vecmath::vec2_sub(p1, p0);
            let n = dtot[0].abs().max(dtot[1].abs());
            let d = [dtot[0]/n, dtot[1]/n];
            itertools::unfold(p0, move |p| {
                let v = p.clone(); 
                p[0]+=d[0];
                p[1]+=d[1]; 
                Some(v)
            }).take((n+1) as usize)
        })
    ).collect()
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let rocks = parse(&input_s);
    let max_depth = rocks.iter().map(|p| p[1]).max().unwrap();

    let mut sand: HashSet<[i16;2]> = HashSet::new();
    'outer: loop {
        let mut p = [500,0];
        while let Some(p2) = [0,-1,1].iter().map(|dx| [p[0]+dx, p[1]+1]).filter(|p2| !rocks.contains(p2) && !sand.contains(p2)).next() {
            p = p2;
            if p[1]>max_depth {break 'outer}
        }
        sand.insert(p);
    }
    let part1 = sand.len();

    let mut sand: HashSet<[i16;2]> = HashSet::new();
    loop {
        let mut p = [500,0];
        while let Some(p2) = [0,-1,1].iter().map(|dx| [p[0]+dx, p[1]+1]).filter(|p2| !rocks.contains(p2) && !sand.contains(p2)).next() {
            p = p2;
            if p[1]>max_depth {break}
        }
        sand.insert(p);
        if p[1]==0 {break}
    }
    let part2 = sand.len();

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "24");
    assert!(res[1] == "93");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    for _ in 0..20 {solution(&input)?;} //warmup
    let start = Instant::now();
    let res = solution(&input)?;
    println!(
        "({} us)\nPart 1: {}\nPart 2: {}",
        start.elapsed().as_micros(), res[0], res[1],
    );
    Ok(())
}
