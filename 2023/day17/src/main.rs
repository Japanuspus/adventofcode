#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use vecmath::{vec2_add, vec2_mul, vec2_scale};
use std::{fs, time::Instant, collections::{HashMap, BTreeSet}};
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

type V=[i16; 2];

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: HashMap<V, u8> = input_s
        .trim_end()
        .split("\n")
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate().map( move |(x, c)| 
            ([x as i16, y as i16], c.to_digit(10).unwrap() as u8))
        )
        .collect();

    let mut work: BTreeSet<(usize, V,V)> = BTreeSet::new();
    work.insert((0, [0,0],[1,0]));
    work.insert((0, [0,0],[0,1]));
    let mut res: HashMap<(V, V), usize> = HashMap::new();
    while let Some((h, p, d)) = work.pop_first() {
        // if we already found a better way here, continue
        let mut improved = true;
        res.entry((p,d))
        .and_modify(|h_res| { 
            if *h_res<=h {
                improved = false
            } else {
                *h_res = h
            }
        }).or_insert(h);
        if !improved {continue}
        // otherwise, add possible continuations to work
        for s in [-1, 1] {
            let d2 = [s*d[1], s*d[0]];
            let mut h2 = h;
            for (p2, dh) in (1..4).map(|n| vec2_add(p, vec2_scale(d2, n)))
                            .map_while(|p2| input.get(&p2).and_then(|dh| Some((p2, *dh)))) {
                h2+=dh as usize;
                work.insert((h2, p2, d2));
            }
        }
    }
    let pmax = [0,1].map(|i| input.keys().map(|p| p[i]).max().unwrap() );

    // for y in 0..pmax[1] {
    //     for x in 0..pmax[0] {
    //         let mut hmin = 999;
    //         for d in [[0,1],[0,-1], [-1,0], [1,0]] {
    //             if let Some(h) = res.get(&([x,y], d)) {
    //                 hmin=hmin.min(*h);
    //             }
    //         }
    //         print!(" {:3}", hmin);
    //     }
    //     println!("");
    // }


    let part1 = res.get(&(pmax, [0,1])).unwrap().min(res.get(&(pmax, [1,0])).unwrap());
    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "102");
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
