#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use vecmath::vec2_add;
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
    distance: i16,
    hex: String,
}

type V=[i16;2];

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<Edge> = input_s
        .trim_end()
        .split("\n")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_, _>>()?;

    let mut edges: HashSet<V> = HashSet::new();
    edges.extend(input.iter()
        .flat_map(|edge| {
            let d: V = match edge.direction {
                'U' => [ 0, -1],
                'D' => [ 0,  1],
                'L' => [-1,  0],
                'R' => [ 1,  0],
                _ => panic!()
            };
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
    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "62");
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
