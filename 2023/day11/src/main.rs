#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use vecmath::{vec2_sub, vec2_len};
use std::{fs, time::Instant, collections::HashSet};

// use parse_display::{Display, FromStr};

// #[derive(Display, FromStr, PartialEq, Debug)]
// enum Direction {
//     #[display("forward")]
//     Forward,
// }

// #[derive(Debug, Display, FromStr)]
// #[display("{direction} {distance}")]
// struct Step {
//     direction: Direction,
//     distance: i32,
// }

type V = [i16;2];

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<V> = input_s
        .trim_end()
        .split("\n")
        .enumerate()
        .flat_map(|(y, ln)| 
            ln.chars().enumerate().filter_map(
                move |(x, c)| if c=='#' {Some([x as i16, y as i16])} else {None}
            )
        )
        .collect();

    let max_i: V = [0,1].map(|i| input.iter().map(|v| v[i]).max().unwrap());
    let map_c:[Vec<i16>;2] = [0,1usize].map(|i| {
        let mut d: Vec<i16> = vec![2;max_i[i] as usize+1];
        for g in &input {
            d[g[i] as usize] = 1;
        };
        std::iter::once(0i16).chain(d.into_iter())
        .scan(0, |acc, dd| {*acc+=dd; Some(*acc)}).collect()
    });

    println!("map c: {:?}", map_c);

    let galaxies: Vec<V> = input.iter().map(|p| [0,1].map(|i| map_c[i][p[i] as usize])).collect_vec();
    println!("Galaxies: {:?}", galaxies);


    let mut part1: usize = 0;
    for (a, b) in galaxies.iter().tuple_combinations() {
        let d: usize=vec2_sub(*a, *b).iter().map(|v| v.abs() as usize).sum();
        part1 +=d;
    }
    let part2 = 0;

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
