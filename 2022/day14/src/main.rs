#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use std::collections::HashSet;
use std::{fs, time::Instant};

use nom;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::{Finish, IResult};



fn parse_point(s: &str) -> IResult<&str, [i16;2]> {
    let (rest, (a, b)) = nom::sequence::separated_pair(
        nom::character::complete::i16, 
        char(','),
        nom::character::complete::i16
    )(s)?;
    Ok((rest,[a,b]))
}
fn parse_line(s: &str) -> IResult<&str, Vec<[i16;2]>> {
    nom::multi::separated_list0(tag(" -> "), parse_point)(s)
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<Vec<[i16;2]>> = input_s.trim_end()
        .split("\n")
        .map(|s| parse_line(s).unwrap().1)
        .collect();

    let rocks: HashSet<[i16;2]> = input.iter().flat_map(|ln| {
        ln.windows(2).flat_map(|p| {
            let dtot = vecmath::vec2_sub(p[1], p[0]);
            let n = dtot[0].abs().max(dtot[1].abs());
            let d = [dtot[0]/n, dtot[1]/n];
            itertools::unfold(p[0].clone(), move |p| {let v = p.clone(); p[0]+=d[0]; p[1]+=d[1]; Some(v)}).take((n+1) as usize)
        })
    }).collect();
    let max_depth = rocks.iter().map(|p| p[1]).max().unwrap();

    let mut sand: HashSet<[i16;2]> = HashSet::new();

    'outer: loop {
        let mut p = [500,0];
        while let Some(p2) = [0,-1,1].iter().map(|dx| [p[0]+dx, p[1]+1]).filter(|p2| !rocks.contains(p2) && !sand.contains(p2)).next() {
            p = p2;
            if p[1]>max_depth {break 'outer}
        }
        sand.insert(p);
    }

    let part1 = sand.len();
    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "24");
    assert!(res[1] == "0");
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
