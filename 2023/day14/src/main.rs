#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use std::{fs, time::Instant, collections::{HashSet, HashMap, BTreeSet}, borrow::Cow};
use vecmath::{vec2_add, vec2_sub};

type V = [i8;2];
struct Board {
    nx: usize,
    ny: usize,
    cubes: BTreeSet<V>,
}

fn parse_input(s: &str) -> Result<(Board, BTreeSet<V>)> {
    let mut cubes = BTreeSet::new();
    let mut balls = BTreeSet::new();
    let (mut nx, mut ny) = (0usize, 0usize);
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
    cubes.extend((0..nx).map(|x| [x as i8, ny as i8])); // south edge
    cubes.extend((0..ny).map(|y| [-1, y as i8])); // west edge
    cubes.extend((0..ny).map(|y| [nx as i8, y as i8])); // east edge
    Ok((Board{nx, ny, cubes}, balls))
} 

fn tilt(board: &Board, balls: &BTreeSet<V>, d: V) -> BTreeSet<V> {
    let mut balls: Cow<BTreeSet<V>> = Cow::Borrowed(balls);
    loop {
        let shifted: BTreeSet<V> = balls.iter().map(|&b| vec2_add(b, d)).collect();
        let mut ball2: BTreeSet<V> = shifted.difference(&board.cubes).cloned().collect();
        for mut b in shifted.intersection(&board.cubes).cloned() {
            loop {
                b = vec2_sub(b, d);    
                match ball2.replace(b) {
                    Some(other_b) => {b=other_b;},
                    None => {break;}
                }
            }
        }
        if ball2==*balls {
            break;
        } else {
            balls = Cow::Owned(ball2);
        }
    }
    balls.into_owned()
}

fn spin(board: &Board, balls: &BTreeSet<V>) -> BTreeSet<V> {
    //NWSE
    [[0,-1], [-1, 0], [0, 1], [1, 0]]
    .iter()
    .fold(Cow::Borrowed(balls), |balls, d| Cow::Owned(tilt(board, &balls, *d)))
    .into_owned()
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let (board, balls) = parse_input(input_s)?;
    
    let balls1 = tilt(&board, &balls, [0,-1]);
    let part1: usize = balls1.iter().map(|[_, y]| board.ny-*y as usize).sum();

    let mut hist: HashMap<BTreeSet<V>, usize> = HashMap::new();
    let mut balls2 = balls.clone();
    let repeat = loop {
        let next_balls = spin(&board, &balls2);
        if let Some(old_v) = hist.insert(balls2, hist.len()) {
            break((old_v, hist.len()))
        }
        balls2 = next_balls;
    };
    println!("Repeat at: {:?}", repeat);

    let n = 1_000_000_000;
    let n_cycle = repeat.1-repeat.0;
    let n_eff = repeat.0+((n-repeat.0) % n_cycle);
    let balls3 = (0..n_eff).fold(Cow::Borrowed(&balls), |b, _| Cow::Owned(spin(&board, &b))).into_owned();

    let part2: usize = balls3.iter().map(|[_, y]| board.ny-*y as usize).sum();

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "136");
    assert_eq!(res[1], "64");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
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

