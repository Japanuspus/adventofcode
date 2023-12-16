#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use vecmath::vec2_add;
use std::{fs, time::Instant, collections::{HashMap, HashSet}};
use itertools::Itertools;

type V = [i16;2];

fn solution(input_s: &str) -> Result<[String; 2]> {
    let circuit: HashMap<V, char> = input_s
        .trim_end()
        .split("\n")
        .enumerate()
        .flat_map(|(y,s)| 
            s.chars().enumerate().filter_map(move |(x, c)| if c=='.' {None} else {Some(([x as i16, y as i16], c))})
        )
        .collect();
    let pmax: V = [0,1].map(|i| circuit.keys().map(|v| v[i]).max().unwrap());

    let mut work: Vec<(V, V)> = vec![([0,0], [1,0])];
    let mut active: HashSet<(V, V)> = HashSet::new();
    while let Some(pd) = work.pop() {
        if pd.0.iter().zip(pmax.iter()).any(|(v, vmax)| *v<0 || v>  vmax) {continue}
        if !active.insert(pd) {continue}
        let mut ds: Vec<V> = Vec::new();
        match (pd.1, circuit.get(&pd.0)) {
            (d, None) => {ds.push(d);}
            (d @ [0,_], Some('|')) => {ds.push(d);}
            (d @ [_,0], Some('-')) => {ds.push(d);}
            ([_,0], Some('|')) => {ds.push([0, 1]); ds.push([ 0,-1]);}
            ([0,_], Some('-')) => {ds.push([1, 0]); ds.push([-1, 0]);}
            ([dx, dy], Some('/'))  => {ds.push([-dy, -dx]);}
            ([dx, dy], Some('\\')) => {ds.push([ dy,  dx]);}
            _ => {}
        };
        for d in ds {
            work.push((vec2_add(pd.0, d), d));            
        };
        //println!("{}: {:?} {:?} -> {:?}", work.len(), pd, circuit.get(&pd.0), &ds);
    }

    // let energized: HashSet<V> = active.iter().map(|(p, _)| *p).collect();
    // for y in 0..pmax[1] {
    //     for x in 0..pmax[0] {
    //         print!("{}", if energized.contains(&[x,y]) {'#'} else {'.'});
    //     }
    //     println!("");
    // }
    // let part1 = energized.len(); 
    
    let part1 = active.iter().map(|(p, _)| *p).unique().count();
    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "46");
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

// // Make it simple to compare timing for multiple solutions
// type Solution = dyn Fn(&str) -> Result<[String; 2]>;
// const SOLUTIONS: [(&str, &Solution); 1] = [("Original", &solution)];

// #[test]
// fn test_solution() -> Result<()> {
//     let input = &fs::read_to_string("test00.txt")?;
//     for (name, solution) in SOLUTIONS {
//         let res = solution(&input).with_context(|| format!("Running solution {}", name))?;
//         println!("---\n{}\nPart 1: {}\nPart 2: {}", name, res[0], res[1]);
//         assert_eq!(res[0], "0");
//         assert_eq!(res[1], "0");
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
