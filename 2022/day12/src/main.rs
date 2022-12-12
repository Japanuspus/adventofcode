#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use vecmath::vec2_add;
use std::{fs, time::Instant, collections::HashMap};

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
    let mut s = [0;2];
    let mut e = [0;2];
    let mut h: HashMap<[i32;2], u8> = HashMap::new();
    for (i, ln) in input_s.trim_end().split("\n").enumerate() {
        for (j, c) in ln.as_bytes().iter().enumerate() {
            let v = match c {
                b'S' => {s=[i as i32,j as i32]; b'a'},
                b'E' => {e=[i as i32,j as i32]; b'z'},
                _ => *c, 
            };
            h.insert([i as i32,j as i32], v);
        }
    }

    let nbs = [[0,1], [1,0], [0,-1], [-1,0]];

    let mut work = vec![(0usize, s.clone())];
    let mut visited: HashMap<[i32;2], usize> = HashMap::new();
    visited.insert(s.clone(), 0);
    while let Some((d, p)) = work.pop() {
        let d2 = d+1;
        for p2 in nbs.iter()
            .map(|nb| vec2_add(p, *nb))
            .filter(|p2|
                h.get(&p)
                .and_then(|hp| h.get(p2).and_then(|&hp2| Some(hp2<=hp+1)))
                .unwrap_or(false)
            ) {
            if let Some(v) = visited.get(&p2) {
                if v<=&d2 {continue;}
            };
            visited.insert(p2.clone(), d2);
            work.push((d2, p2));
        }
    }

    // for (i, ln) in input_s.trim_end().split("\n").enumerate() {
    //     for (j, c) in ln.as_bytes().iter().enumerate() {
    //         print!(
    //             " {}{}", 
    //             match visited.get(&[i as i32,j as i32]) { Some(_)=>'>', None=>' '},
    //             *c as char);
    //     }
    //     println!();
    // }
  
    let part1 = visited.get(&e).unwrap();
    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "31");
    assert!(res[1] == "0");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    // for _ in 0..20 {solution(&input)?;} //warmup
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
//         assert!(res[0] == "0");
//         assert!(res[1] == "0");
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
