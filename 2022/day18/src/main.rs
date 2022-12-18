#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use vecmath::{vec2_add, vec3_add};
use std::{fs, time::Instant, collections::{HashSet, VecDeque}};

type Coord = [i8;3];

fn solution(input_s: &str) -> Result<[String; 2]> {
    let droplets: HashSet<Coord> = input_s.trim_end()
        .split("\n")
        .map(|s| 
            s.split(',')
            .map(|v| v.parse::<i8>().with_context(|| format!("Parsing {}", s)))
            .collect::<Result<Vec<_>,_>>()
            .and_then(|r| Ok(r.try_into().unwrap()))  //TOOD: proper error map
        )
        .collect::<Result<_, _>>()?;

    let nbs: [Coord;6]=[
        [ 0, 0, 1], [ 0, 0,-1],
        [ 0, 1, 0], [ 0,-1, 0],
        [ 1, 0, 0], [-1, 0, 0],
    ];

    let part1:usize = droplets.iter().map(|d| nbs.iter().filter(|&nb| !droplets.contains(&vec3_add(*d,*nb))).count()).sum();

    // flood-fill exterior
    let vmin: Coord = (0..3usize).map(|i| droplets.iter().map(|d| d[i]-1).min().unwrap()).collect::<Vec<_>>().try_into().unwrap();
    let vmax: Coord = (0..3usize).map(|i| droplets.iter().map(|d| d[i]+1).max().unwrap()).collect::<Vec<_>>().try_into().unwrap();
    let mut exterior: HashSet<Coord> = HashSet::new();
    let mut frontier: VecDeque<Coord> = VecDeque::new();
    frontier.push_back(vmin.clone());
    let mut part2 = 0;
    while let Some(p) = frontier.pop_front() {
        if exterior.contains(&p) {continue};
        for nb in nbs.iter().map(|d| vec3_add(p, *d)) {
            if nb.iter().zip(vmin.iter()).any(|(v, bound)| v<bound) {continue};
            if nb.iter().zip(vmax.iter()).any(|(v, bound)| v>bound) {continue};
            if droplets.contains(&nb) {part2+=1; continue}
            frontier.push_back(nb);
        }
        exterior.insert(p);
    }

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "64");
    assert!(res[1] == "58");
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

