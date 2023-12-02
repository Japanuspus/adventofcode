#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result, anyhow};
use std::{fs, time::Instant};

//Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

fn parse_reveal(s: &str) -> [u8;3] {
    let mut res = [0;3];
    for v in s.split(", ") {
        if let Some((vs, c)) = v.split_once(' ') {
            let v=vs.parse::<u8>().unwrap();
            res[
                match c {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2, 
                    _ => {panic!("Unknown color")}
                }
            ]=v;
        }        
    }
    res
}

struct Game {
    id: usize,
    reveals: Vec<[u8;3]>,
}

fn parse_game(s: &str) -> Result<Game> {
    if let Some((game, r)) = s.split_once(": ") {
        let id = game[5..].parse()?;
        let reveals = r.split("; ").map(|s| parse_reveal(s)).collect();
        return Ok(Game{id, reveals})
    }
    Err(anyhow!("No colon in game"))
}

// use parse_display::{Display, FromStr};

// #[derive(Display, FromStr, PartialEq, Debug)]
// enum Color {
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
    let input: Vec<Game> = input_s.trim_end()
        .split("\n")
        .map(|s| parse_game(s).with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_, _>>()?;

    let max1 = [12, 13, 14];
    let part1: usize = input.iter().filter_map(|g| {
        if g.reveals.iter().all(|r| 
            r.iter().zip(max1.iter()).all(|(l, r)| l<=r)
        ) {
            //println!("+ {}: {:?}", g.id, g.reveals);
            Some(g.id)
        } else {
            //println!("- {}: {:?}", g.id, g.reveals);
            None
        }
    }).sum();

    let part2:usize = input.iter().map(|g| {
        let mut m = g.reveals[0];
        for r in &g.reveals[1..] {
            for i in 0..3 {
                if r[i]>m[i] {m[i]=r[i]}
            }
        };
        m.iter().map(|v| *v as usize).product::<usize>()
    }).sum();

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "8");
    assert_eq!(res[1], "2286");
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
