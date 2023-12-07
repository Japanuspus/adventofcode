#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::{collections::HashSet, fs, time::Instant};

mod nm {
    pub use nom::bytes::complete::*;
    pub use nom::character::complete::*;
    pub use nom::error::*;
    pub use nom::multi::*;
    pub use nom::sequence::*;
    pub use nom::IResult;

    /// A combinator that takes a parser `inner` and produces a parser that 
    /// also consumes both leading and trailing whitespace, 
    /// returning the output of `inner`.
    pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
        inner: F,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        F: Fn(&'a str) -> IResult<&'a str, O, E>,
    {
        delimited(multispace0, inner, multispace0)
    }
}

fn parse(s: &str) -> nm::IResult<&str, Vec<(u16, (Vec<u16>, Vec<u16>))>> {
    let u16list = || nm::separated_list1(nm::space1, nm::u16);
    let win_have = nm::separated_pair(u16list(), nm::ws(nm::tag("|")), u16list());
    let id = nm::delimited(nm::ws(nm::tag("Card")), nm::u16, nm::ws(nm::tag(":")));
    let card = nm::pair(id, win_have);
    nm::separated_list1(nm::newline, card)(s)
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let (_rest, input) = parse(input_s).map_err(|e| e.to_owned())?;
    // println!("Rest: \n{}\nEND OF REST", rest);
    //println!("Input: {:?}", input);

    let part1: usize = input
        .iter()
        .map(|(_, (wins, have))| {
            let wins: HashSet<u16> = HashSet::from_iter(wins.iter().cloned());
            let n = have.iter().filter(|h| wins.contains(h)).count();
            if n == 0 {
                0
            } else {
                1usize << (n - 1)
            }
        })
        .sum();

    let mut copies: Vec<usize> = vec![0; input.len()];
    for (i, (_, (wins, have))) in input.iter().enumerate() {
        let wins: HashSet<u16> = HashSet::from_iter(wins.iter().cloned());
        let n = have.iter().filter(|h| wins.contains(h)).count();
        let nc = copies[i] + 1;
        for j in 0..n {
            copies[i + 1 + j] += nc
        }
    }
    let part2: usize = copies.iter().map(|v| v + 1).sum();

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "13");
    assert_eq!(res[1], "30");
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
