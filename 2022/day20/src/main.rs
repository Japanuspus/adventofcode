#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use itertools::Itertools;
use std::{fs, time::Instant, collections::{HashMap, HashSet}};


type Ring = Vec<[u16;2]>;

fn traverse(ring: &Ring, start_index: usize, d: usize) -> impl Iterator<Item=usize> + '_ {
    itertools::unfold(start_index, move |i| {*i = ring[*i][d] as usize; Some(*i)})
}

fn traverse_values<'a>(ring: &'a Ring, values: &'a [i32]) -> impl Iterator<Item=i32> + 'a {
    let idx0 = values.iter().enumerate().find_map(|(i, v)| if *v==0 {Some(i)} else {None}).unwrap();
    traverse(ring, idx0, 1).map(|idx| values[idx]).take(values.len())
}

fn print_ring(ring: &Ring, values: &[i32]) {
    for v in traverse_values(ring, values) {
        print!("{}, ", &v);
    }
    println!();
}

fn check_ring(ring: &Ring) -> bool {
    [0, 1].iter().all(|d| 
        Some(0) == traverse(ring, 0, *d).nth(ring.len()-1)
    )
}

fn process_ring(ring: &mut Ring, values: &[i32], mult: isize) {
    let n = values.len();
    for (idx, &val) in values.iter().enumerate() {
        if val==0 {continue};
        let val = (val as isize * mult)%((n-1) as isize);
        let (d, dbar) = if val<0 {(0,1)} else {(1, 0)};
        // fuse
        let nbs = ring[idx];
        ring[nbs[0] as usize][1] = nbs[1];
        ring[nbs[1] as usize][0] = nbs[0];
        // find insertion point
        let (prev, cur) = itertools::unfold((nbs[dbar], nbs[d]), |(prev, cur)| {
            *prev = *cur;
            *cur = ring[*cur as usize][d];
            Some((*prev, *cur))
        }).nth(val.abs() as usize-1).unwrap();
        // insert
        ring[prev as usize][d]=idx as u16;
        ring[cur as usize][dbar]=idx as u16;
        ring[idx] = if d>0 {[prev, cur]} else {[cur, prev]};
    }
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<i32> = input_s.trim_end()
        .split("\n")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_, _>>()?;

    let n = input.len();
    let mut ring: Vec<[u16;2]> = (0..n).into_iter()
        .map(|w| [
            ((w as isize)-1isize).rem_euclid(n as isize) as u16,
            ((w as isize)+1isize).rem_euclid(n as isize) as u16,
        ]).collect();
    if !check_ring(&ring) {
        panic!();
    }
    let mut ring2 = ring.clone();

    process_ring(&mut ring, &input, 1);
    let idx0 = input.iter().enumerate().find_map(|(i, v)| if *v==0 {Some(i)} else {None}).unwrap();
    let part1: isize = traverse(&ring, idx0, 1).skip(999).step_by(1000).take(3).map(|idx| input[idx] as isize).sum();

    let key = 811589153isize;
    for _ in 0..10 {
        process_ring(&mut ring2, &input, key);
    }
    let part2: isize = traverse(&ring2, idx0, 1).skip(999).step_by(1000).take(3).map(|idx| key * input[idx] as isize).sum();

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "3");
    assert!(res[1] == "1623178306");
    Ok(())
}

#[test]
fn test_part1() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "23321");
    Ok(())
}


fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    // for _ in 0..20 {solution(&input)?;} //warmup
    let start = Instant::now();
    let res = solution(&input)?;
    println!(
        "({} us)\nPart 1: {}\nPart 2: {}",
        start.elapsed().as_micros(), res[0], res[1],
    );
    Ok(())
}

//7160 low
//1646 low
// // Make it simple to compare timing for multiple solutions
// type Solution = dyn Fn(&str) -> Result<[String; 2]>;
// const SOLUTIONS: [(&str, &Solution); 1] = [("Original", &solution)];

// #[test]
// fn test_solution() -> Result<()> {
//     let input = &fs::read_to_string("test00.txt")?;
//     for (name, solution) in SOLUTIONS {
//         let res = solution(&input).with_context(|| format!("Running solution {}", name))?;
//         println!("---\n{}\nPart 1: {}\nPart 2: {}", name, res[0], res[1]);
//         assert!(res[0] == "0");
//         assert!(res[1] == "0");
//     }
//     Ok(())
// }

// fn main() -> Result<()> {
//     let input = &fs::read_to_string("input.txt")?;
//     for (_, solution) in SOLUTIONS.iter().cycle().take(10) {
//         solution(&input)?;
//     } //warmup
//     for (name, solution) in SOLUTIONS {
//         let start = Instant::now();
//         let res = solution(&input)?;
//         println!(
//             "---\n{} ({} us)\nPart 1: {}\nPart 2: {}",
//             name, start.elapsed().as_micros(), res[0], res[1],
//         );
//     }
//     Ok(())
// }
