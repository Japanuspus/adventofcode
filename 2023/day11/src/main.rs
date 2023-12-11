#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use vecmath::{vec2_sub, vec2_len};
use std::{fs, time::Instant, collections::HashSet};


type V = [isize;2];

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<V> = input_s
        .trim_end()
        .split("\n")
        .enumerate()
        .flat_map(|(y, ln)| 
            ln.chars().enumerate().filter_map(
                move |(x, c)| if c=='#' {Some([x as isize, y as isize])} else {None}
            )
        )
        .collect();
    let max_i: V = [0,1].map(|i| input.iter().map(|v| v[i]).max().unwrap());

    let map_c:[Vec<isize>;2] = [0,1usize].map(|i| {
        let mut d: Vec<isize> = vec![2;max_i[i] as usize+1];
        for g in &input {d[g[i] as usize] = 1;};
        std::iter::once(0isize).chain(d.into_iter())
        .scan(0, |acc, dd| {*acc+=dd; Some(*acc)}).collect()
    });
    let galaxies: Vec<V> = input.iter().map(|p| [0,1].map(|i| map_c[i][p[i] as usize])).collect_vec();
    let part1: usize = galaxies.iter().tuple_combinations().map(|(a,b)| {
        vec2_sub(*a, *b).iter().map(|v| v.abs() as usize).sum::<usize>()
    }).sum();

    let map_c:[Vec<isize>;2] = [0,1usize].map(|i| {
        let mut d: Vec<isize> = vec![1_000_000;max_i[i] as usize+1];
        for g in &input {d[g[i] as usize] = 1;};
        std::iter::once(0isize).chain(d.into_iter())
        .scan(0, |acc, dd| {*acc+=dd; Some(*acc)}).collect()
    });
    let galaxies: Vec<V> = input.iter().map(|p| [0,1].map(|i| map_c[i][p[i] as usize])).collect_vec();
    let part2: usize = galaxies.iter().tuple_combinations().map(|(a,b)| {
        vec2_sub(*a, *b).iter().map(|v| v.abs() as usize).sum::<usize>()
    }).sum();

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "374");
    assert_eq!(res[1], "8410");
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
