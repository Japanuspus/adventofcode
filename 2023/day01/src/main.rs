#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use itertools::Itertools;
use std::{fs, time::Instant};

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

fn parse_line(s: &str) -> u32 {
    let mut cs = s.chars().filter_map(|c| c.to_digit(10));
    let c1 = cs.next().unwrap();
    let c2 = cs.last().unwrap_or(c1);
    10*c1+c2
}

const DIGIT_STRINGS: [&str; 20] = [
    "0", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
];

fn as_elf_digit(s: &[u8]) -> Option<u32> {
    for (i, ed) in DIGIT_STRINGS.iter().enumerate() {
        if s.starts_with(ed.as_bytes()) {
            return Some(i as u32%10)
        }
    }
    None
}

#[test]
fn test_as_elf_digit() {
    assert_eq!(as_elf_digit(b"seven"), Some(7));
    assert_eq!(as_elf_digit(b"2"), Some(2));
    assert_eq!(as_elf_digit(b"seven2"), Some(7));
    assert_eq!(as_elf_digit(b"2b"), Some(2));
}

fn parse_line2(s: &str) -> u32 {
    let s=s.as_bytes();
    let mut cs = (0..s.len()).filter_map(|idx| as_elf_digit(&s[idx..]));
    let c1 = cs.next().ok_or_else(|| format!("Error parsing {:?}", s)).unwrap();
    let c2 = cs.last().unwrap_or(c1);
    10*c1+c2
}

#[test]
fn test_parse_line2() {
    assert_eq!(parse_line2("6798seven"), 67);
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let part1:usize = input_s.trim_end()
        .split("\n")
        .map(|s| parse_line(s) as usize)
        .sum();
    
    let part2:usize = input_s.trim_end()
        .split("\n")
        .map(|s| parse_line2(s) as usize)
        .sum();
    
    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "142");

    let input = &fs::read_to_string("test01.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[1], "281");
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
