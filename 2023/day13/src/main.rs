#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use ndarray::Array2;
use std::{fs, time::Instant};


fn part1(s: &str) -> Result<usize> {
    let mut nrows: usize =0;
    let cs: Vec<u8> = s
        .split("\n")
        .inspect(|_| {nrows+=1})
        .flat_map(|ln| ln.as_bytes().iter())
        .cloned()
        .collect();
    let ncols = cs.len()/nrows;
    let m = Array2::from_shape_vec((nrows, ncols), cs)?;

    let rsym = (1..nrows).find(|&i| 
        (0..i).rev().zip(i..nrows).all(|(i1, i2)| m.row(i1)==m.row(i2))
    ).unwrap_or(0);
    
    let csym = (1..ncols).find(|&i| 
        (0..i).rev().zip(i..ncols).all(|(i1, i2)| m.column(i1)==m.column(i2))
    ).unwrap_or(0);
    
    Ok(csym+100*rsym)
}

// #[test]
// fn test_part1() {
//     let input = &fs::read_to_string("test00.txt")?;
//     assert_eq!(part1(input), Ok())
//     let res = solution(&input)?;
//     println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
//     assert_eq!(res[0], "");
//     assert_eq!(res[1], "0");
//     Ok(())
// }

fn solution(input_s: &str) -> Result<[String; 2]> {
    let part1: usize = input_s
        .trim_end()
        .split("\n\n")
        .map(|s| part1(s))
        .sum::<Result<_>>()?;

        let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "405");
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
