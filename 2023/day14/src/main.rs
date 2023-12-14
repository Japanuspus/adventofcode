#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use std::{fs, time::Instant, collections::HashSet};
use vecmath::{vec2_add, vec2_sub};

type V = [i8;2];
struct Board {
    nx: usize,
    ny: usize,
    cubes: HashSet<V>,
}

fn parse_input(s: &str) -> Result<(Board, HashSet<V>)> {
    let mut cubes = HashSet::new();
    let mut balls = HashSet::new();
    let (mut nx, mut ny) = (0,0);
    for (iy, r) in s.trim_end().split("\n").enumerate() {
        ny+=1;
        if iy==0 {nx=r.len();}
        for (ix, c) in r.chars().enumerate() {
            let key: V = [ix as i8, iy as i8];
            match c {
                '#' => {cubes.insert(key);},
                'O' => {balls.insert(key);},
                '.' => {},
                _ => {return Err(anyhow!("Unexpected char in line: {}", r));}
            } 
        }
    }
    cubes.extend((0..nx).map(|x| [x as i8, -1])); // north edge
    Ok((Board{nx, ny, cubes}, balls))
} 

fn solution(input_s: &str) -> Result<[String; 2]> {
    let (board, mut balls) = parse_input(input_s)?;
    
    let d: V = [0,-1];
    loop {
        let shifted: HashSet<V> = balls.iter().map(|&b| vec2_add(b, d)).collect();
        let mut ball2: HashSet<V> = shifted.difference(&board.cubes).cloned().collect();
        for mut b in shifted.intersection(&board.cubes).cloned() {
            loop {
                b = vec2_sub(b, d);    
                match ball2.replace(b) {
                    Some(other_b) => {b=other_b;},
                    None => {break;}
                }
            }
        }
        if ball2==balls {
            break
        } else {
            balls = ball2;
        }
    }

    let part1: usize = balls.iter().map(|[_, y]| board.ny-*y as usize).sum();
    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "136");
    assert_eq!(res[1], "0");
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

