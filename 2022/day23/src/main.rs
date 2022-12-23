#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use vecmath::vec2_add;
use std::{fs, time::Instant, collections::{HashSet, HashMap}};
use itertools::{Itertools, MinMaxResult};

// 0 7 6 
// 1   5
// 2 3 4
const NEIGHBORS: [Pos;8] = [[-1, -1], [-1, 0], [-1,1], [0, 1], [1, 1], [1, 0], [1, -1], [0, -1]];
struct Dir {
    shift: usize,
    check: [usize; 3],
}
const DIRECTIONS: [Dir;4] = [
    Dir{shift:7, check: [6, 7, 0]}, //N
    Dir{shift:3, check: [2, 3, 4]}, //S
    Dir{shift:1, check: [0, 1, 2]}, //W
    Dir{shift:5, check: [4, 5, 6]}, //E
];

type Pos = [i16;2];
type Elfs = HashSet::<Pos>;

fn move_1(elfs: &Elfs, d0: usize) -> Elfs {
    let mut props: HashMap<Pos, usize> = HashMap::new();
    let mut props_by_elf: Vec<(Pos, Option<Pos>)> = Vec::new();
    for elf in elfs.iter() {
        let to_check: Vec<(Pos, bool)> = NEIGHBORS.iter().map(|nb| {let p = vec2_add(*elf, *nb); (p, elfs.contains(&p))}).collect();
        match if to_check.iter().all(|(_, occ)| !occ) {
            // All nbs unocc
            None
        } else {
            DIRECTIONS.iter().cycle().skip(d0).take(4).find(|d| to_check.iter().cycle().skip(d.check[0]).take(3).all(|(_, occ)| !occ))
        } {
            None => props_by_elf.push((*elf, None)),
            Some(d) => {
                let p = to_check[d.shift].0;
                *(props.entry(p).or_default())+=1;
                props_by_elf.push((*elf, Some(p)));
            }
        };
    };
    props_by_elf.into_iter().map(|(elf, m)| {
        if let Some(p) = m {
            if props[&p]<2 {p} else {elf}
        } else {elf}
    }).collect()
}

fn print_elfs(elfs: &Elfs) {
    for r in -3..15 {
        for c in -3..15 {
            print!("{}", if elfs.contains(&[c as i16, r as i16]) {'#'} else {'.'});
        }
        println!();
    }
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: HashSet::<[i16;2]> = input_s.trim_end()
        .split("\n").enumerate()
        .flat_map(|(r, s)| s.chars().enumerate()
            .filter_map(move |(c, v)| if v=='#' {Some([c as i16, r as i16])} else {None})
        ).collect();

    // coords are col, row
    let mut elfs = input.clone();
    //print_elfs(&elfs);
    for id in 0..10 {
        elfs = move_1(&elfs, id%4);
        //println!("\n After Iteration {}", id+1);
        //print_elfs(&elfs);
    }
    let part1: usize = [0usize,1].into_iter()
    .map(|i| if let MinMaxResult::MinMax(v1, v2) = elfs.iter().map(|p| p[i]).minmax() {(1+v2-v1) as usize} else {panic!()})
    .product::<usize>() - elfs.len();
    
    let part2 = (0..4).cycle().scan(input.clone(), |elfs, id|  {
        let elfs2 = move_1(&elfs, id);
        if &elfs2 == elfs {None} else {*elfs=elfs2; Some(())}
    }).count()+1;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "110");
    assert!(res[1] == "20");
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
