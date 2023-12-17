#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use vecmath::{vec2_add, vec2_mul, vec2_scale};
use std::{fs, time::Instant, collections::{HashMap, BTreeSet}};
use itertools::Itertools;

type V=[i16; 2];

fn minimal_heat(input: &HashMap<V, u8>, d_skip: usize, d_max: usize) -> usize {
    let mut work: BTreeSet<(usize, V,V)> = BTreeSet::from_iter([
        (0, [0,0],[1,0]), (0, [0,0],[0,1]),
    ]);
    let mut res: HashMap<(V, V), usize> = HashMap::new();
    while let Some((h, p, d)) = work.pop_first() {
        // if we already found a better way here, continue
        let mut improved = true;
        res.entry((p,d))
        .and_modify(|h_res| { 
            if *h_res<=h {
                improved = false
            } else {
                *h_res = h
            }
        }).or_insert(h);
        if !improved {continue}
        // otherwise, add possible continuations to work
        for d2 in [[-d[1], -d[0]], [d[1], d[0]]] {
            work.extend(
                (0..d_max)
                .scan(p, |p2, _| {*p2=vec2_add(*p2, d2); Some(*p2)})
                .scan(h, |h2, p2| input.get(&p2).and_then(|dh| {
                    *h2+=*dh as usize; Some((*h2, p2, d2))
                }))
                .skip(d_skip)
            );
        };    
    }

    let pmax = [0,1].map(|i| input.keys().map(|p| p[i]).max().unwrap() );
    *res.get(&(pmax, [0,1])).unwrap().min(res.get(&(pmax, [1,0])).unwrap())
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: HashMap<V, u8> = input_s
        .trim_end()
        .split("\n")
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate().map( move |(x, c)| 
            ([x as i16, y as i16], c.to_digit(10).unwrap() as u8))
        )
        .collect();

    let part1 = minimal_heat(&input, 0, 3);
    let part2 = minimal_heat(&input, 3, 10);

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "102");
    assert_eq!(res[1], "94");

    let input = &fs::read_to_string("test03.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[1], "71");

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
