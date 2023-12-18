#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use vecmath::{vec2_add, vec2_scale};
use std::{fs, time::Instant, collections::HashSet};
use itertools::Itertools;

use parse_display::{Display, FromStr};

// #[derive(Display, FromStr, PartialEq, Debug)]
// enum Direction {
//     #[display("forward")]
//     Forward,
// }

// L 6 (#2d8140)
#[derive(Debug, Display, FromStr)]
#[display("{direction} {distance} (#{hex})")]
struct Edge {
    direction: char,
    distance: i64,
    hex: String,
}

type V=[i64;2];

fn edge_d(d: char) -> Option<V> {
    match d {
        'U' => Some([ 0, -1]),
        'D' => Some([ 0,  1]),
        'L' => Some([-1,  0]),
        'R' => Some([ 1,  0]),
        _ => None
    }
}

//fn greens_area(input: &Vec<(V, i64)>)

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<Edge> = input_s
        .trim_end()
        .split("\n")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_, _>>()?;

    let mut edges: HashSet<V> = HashSet::new();
    edges.extend(input.iter()
        .flat_map(|edge| {
            let d = edge_d(edge.direction).unwrap();
            std::iter::repeat(d).take(edge.distance as usize)
        })
        .scan([0,0], |p, d| {*p = vec2_add(*p, d); Some(*p)})
    );
    assert!(edges.contains(&[0,0]));
    assert!(!edges.contains(&[1,1]));

    let mut fill = edges.clone();
    let mut work: Vec<V> = vec![[1,1],];
    while let Some(w) = work.pop() {
        if fill.insert(w) {
            work.extend(
                [[0,-1],[-1,0],[0,1],[1,0]]
                .iter().map(|d| vec2_add(w, *d))
            )
        }
    }
    let part1 = fill.len();

    // Using greens theorem 2*a = sum x dy - y dx
    // exactly one unit is not counted if adding 1/2 * edge len
    //let edges_norm: Vec<(V, i64)> = input.iter().map(|e| (edge_d(e.direction).unwrap(), e.distance)).collect();
    
    //0 means R, 1 means D, 2 means L, and 3 means U.

    let dirs = [[1,0],[0,1],[-1,0],[0,-1]];
    let edges_norm: Vec<(V, i64)> = input.iter().map(|e| {
        let s=&e.hex;
        let d=dirs[(s.as_bytes()[5]-b'0') as usize];
        let n=i64::from_str_radix(&s[..5], 16).unwrap();
        (d, n)
    }).collect();


    let (a2_inner, _) = edges_norm.iter()
    .fold((0, [0,0]), |(a2, p), (d, n)| {
        let np = vec2_add(p, vec2_scale(*d, *n));
        let na2 = a2 + n*(p[0]*d[1]-p[1]*d[0]);
        (na2, np)
    });
    let edges_len: i64 = edges_norm.iter().map(|(_, n)| n).sum();

    println!("a2_inner: {}, edges: {}, tot: {}, 2*part2: {} ", a2_inner, edges.len(), a2_inner as usize+edges.len(), 2*part1);

    let part2: i64 = 1+(a2_inner+edges_len)/2;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "62");
    assert_eq!(res[1], "952408144115");
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
