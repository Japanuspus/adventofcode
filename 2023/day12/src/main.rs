#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use std::{fs, time::Instant, str::FromStr};
use itertools::Itertools;

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

#[derive(Debug)]
struct Record {
    // LSB == Rightmost entry in definition string
    good: u64,
    unknown: u64,
    length: usize,
}

impl FromStr for Record {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut good = 0;
        let mut unknown = 0;
        let mut length = 0;
        for c in s.chars() {
            good<<=1;
            unknown<<=1;
            length+=1;
            match c {
                '?' => {unknown |= 1;}
                '#' => {good |= 1;}
                '.' => {}
                _ => {return Err(anyhow!("Unknown char: {}", c))}
            }
        };
        if length>64 {return Err(anyhow!("Field too long"))}
        Ok(Self{good, unknown, length})
    }
}

#[test]
fn parse_record() {
    let r: Record = "..?##?".parse().unwrap();
    assert_eq!(    0b001001, r.unknown);
    assert_eq!(    0b000110, r.good);
    assert_eq!(r.length, 6);
}

// struct Spacer {
//     // n-1 dividers move between 0 and total_space (incl)
//     dividers: [u8;64],
//     n: usize,
//     total_space: usize,
//     init: bool,
// }

// impl Spacer {
//     fn new(n: usize ,total_space: usize) -> Self {
//         let mut dividers = [0u8;64]; // only 0..n are used
//         for i in 0..(n-1) {dividers[i] = total_space as u8;}
//         Self{dividers, n, total_space, init: True}
//     }

//     fn step(&mut self) -> bool {
//         for i in 0..(self.n-1) {
//             if self.dividers[i]>0 {
//                 let v = self.dividers[i]-1;
//                 for j in 0..(i+1) {
//                     self.dividers[j]=v;
//                 }
//                 return true;
//             }
//         }
//         false
//     }
// }

// impl Iterator for Spacer {
//     type Item = [u8;64];

//     fn next(&mut self) -> Option<Self::Item> {
//         if !self.init{
//             if !self.step() {
//                 return None
//             }
//         } else {
//             self.init = false;
//         }
//         let mut spaces=[0u8;64];
//         let mut last_div = 0;
//         for i in 0..self.n {
//             spaces[i] = self.dividers[i]-last_div;
//             last_div = self.dividers[i];
//         }
//         Some(spaces)
//     }
// }

// #[test]
// fn test_spacer() {
//     let s = Spacer::new(3, 7);
//     let mut v = [0u8;64];
//     v[0] = 64;
//     assert_eq!(s.next(), )
// }

fn part1(r: &Record, groups: &Vec<usize>) -> usize {
    let min_size: usize = groups.iter().sum::<usize>()+groups.len()-1;
    let max_shift = r.length - min_size;
    let mut shifts = vec![max_shift;groups.len()];
    //First -> leftmost -> MSB
    let group_masks: Vec<u64> = {
        let v: Vec<u64> = groups.iter().rev()
            .scan(0usize, |pad, g| {let r= *pad; *pad += *g+1; Some((r, g))})
            .map(|(pad, g)| (0..*g).fold(0u64, |acc, _| (acc<<1)|1)<<pad)
            .collect();
        v.into_iter().rev().collect()
    };

    let mut count: usize = 0;
    'outer: loop {
        // build combined mask
        let mask = group_masks.iter().zip(shifts.iter().rev()).fold(0,|mask, (gm, gs)| mask|(gm<<gs));

        // check mask
        // good < mask: all good covered by mask
        // mask < (good | maybe): mask only covers good | maybe
        if (mask & r.good == r.good) && (mask & (r.good | r.unknown) == mask) {
            count+=1;
        } 
        
        // update shifts
        for i in 0..shifts.len() {
            if let Some(v) = shifts[i].checked_sub(1) {
                for j in 0..(i+1) {
                    shifts[j]=v;
                }
                continue 'outer;
            }
        }
        break
    }

    println!("Record: {:?}, {}", r, count);
    count
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<(Record, Vec<usize>)> = input_s
        .trim_end()
        .split("\n")
        .filter_map(|ln| {
            ln.split_once(' ')
            .and_then(|(b,r)| Some((
                b.parse().unwrap(),
                r.split(',').map(|s| s.parse().unwrap()).collect_vec()
                
            )))
        })
        .collect();


    let part1: usize = input.iter().map(|r| part1(&r.0, &r.1)).sum();
    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test01.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "21");
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
