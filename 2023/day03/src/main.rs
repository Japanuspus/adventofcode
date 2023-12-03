#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::{fs, time::Instant, collections::{HashMap, HashSet}, sync::Arc, iter};

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

fn solution(input_s: &str) -> Result<[String; 2]> {
    // let symbols: Vec<HashMap<i32, char>> = input_s
    //     .trim_end()
    //     .split("\n")
    //     .map(|s| {
    //         s.chars().enumerate()
    //         .filter(|(_, c)| !(*c=='.' || c.is_ascii_digit()))
    //         .map(|(i, c)| (i as i32, c))
    //         .collect()
    //     }).collect();

    // // padded symbol locs to prepare for windowing
    // let symbol_locs: Vec<HashSet<i32>> = 
    //     iter::once(HashSet::new())
    //     .chain(symbols.iter().map(|sl| sl.keys().cloned().collect()))
    //     .chain(iter::once(HashSet::new()))
    //     .collect();

    let symbols: HashMap<(i32, i32), char> = input_s
        .trim_end()
        .split("\n")
        .enumerate()
        .flat_map(|(ln, s)| {
            s.chars().enumerate()
            .filter(|(_, c)| !(*c=='.' || c.is_ascii_digit()))
            .map(move |(i, c)| ((i as i32, ln as i32), c))
        }).collect();
    
    let re = Regex::new(r"\d+")?;
    let mut part1:usize = 0;
    for (ln, s) in input_s.trim_end().split("\n").enumerate() {
        let ln=ln as i32;
        for m in re.find_iter(s) {
            if (-1i32..2).any(|dl| 
                ((m.start() as i32-1)..(m.end() as i32+1))
                .any(|x| symbols.contains_key(&(x, ln+dl))))
            {
                part1 += m.as_str().parse::<usize>()?;
            }  
        }        
    }

    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "4361");
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
