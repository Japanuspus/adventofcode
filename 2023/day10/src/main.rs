#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use std::{fs, time::Instant, collections::HashMap};
use vecmath::{vec2_add, vec2_scale, vec2_len, vec2_sub};

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
const NESW: [V;4] = [[0,-1], [1,0], [0,1], [-1,0]];

// fn loop_length(map: &HashMap<V, u8>,p0: V, d0: V) {

// }

fn next_direction(s: u8, d: usize) -> Option<usize> {
    match (s, (d+2)%4) {
        (b'|', 0) => Some(2), 
        (b'|', 2) => Some(0), 
        (b'-', 1) => Some(3), 
        (b'-', 3) => Some(1), 
        (b'7', 2) => Some(3), 
        (b'7', 3) => Some(2), 
        (b'F', 1) => Some(2), 
        (b'F', 2) => Some(1), 
        (b'L', 0) => Some(1), 
        (b'L', 1) => Some(0), 
        (b'J', 0) => Some(3), 
        (b'J', 3) => Some(0), 
        _ => None
    }
} 

fn solution(input_s: &str) -> Result<[String; 2]> {
    let map: HashMap<V, u8> = input_s
        .trim_end()
        .split("\n")
        .enumerate()
        .flat_map(|(y, ln)| 
            ln.as_bytes().iter().enumerate().map(move |(x, c)| ([x as i16, y as i16], *c))
        )
        .collect();
    let p0 = map.iter().find_map(|(k,v)| if *v==b'S' {Some(*k)} else {None}).unwrap();

    let max_loop = (0..4usize).filter_map(|d0| {
        // loop_length(map, p0, d0) -> Option<usize>
        let mut p: V = p0;
        let mut d = d0;
        let mut i: usize = 0;

        loop {
            // println!("{:?}, d:{}, i:{}", p, d, i);
            p = vec2_add(p, NESW[d]);
            i += 1;
            if let Some(&c) = map.get(&p) {
                if c==b'S' {
                    return Some(i)
                }
                //println!(">{}: {:?}, {}, {}", c as char, p, d, i);
                if let Some(d_new) = next_direction(c, d) {
                    //println!("<{}: {:?}, {}, {}", c as char, p, d, i);
                    d=d_new;
                } else {
                    return None
                }
            } else {
                return None
            }
        }
    }).max().unwrap();
    let part1 = (max_loop+1)/2;
    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test07.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "8");
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
