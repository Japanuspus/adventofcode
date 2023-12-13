#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, bail, Context, Result};
use core::fmt;
use itertools::Itertools;
use num::{BigInt, Integer, One};
use std::{collections::HashMap, fs, time::Instant};

mod nm {
    pub use nom::sequence::*;
    pub use nom::multi::*;
    pub use nom::*;
    pub use nom::error::*;
    pub use nom::character::complete::*;
    pub use nom::bytes::complete::*;
}

fn parse_nom(s: &str) -> nm::IResult<&str, (&str, Vec<Vec<&str>>)> {
    let (rest, lrs) = nm::terminated(nm::alpha1, nm::multispace1)(s)?;
    // QKX = (SQD, XTJ)
    let (rest, maps) = nm::separated_list1(
        nm::newline,
        nm::terminated(
            nm::separated_list1(nm::many1(nm::one_of(" (),=")), nm::alpha1),
            nm::tag(")")
        )
    )(rest)?;
    Ok((rest, (lrs, maps)))
}    

fn solution(input_s: &str) -> Result<[String;2]> {
    let (_rest, (lrs, maps)) = parse_nom(input_s).map_err(|e| e.to_owned())?;
    let lrs: &[u8] = lrs.as_bytes();
    let forks: HashMap<_,_> = maps.into_iter().map(|w| {
        //let v: Vec<String> = v0.into_iter().map(|s| s.to_owned()).collect();
        //(w[0].to_owned(), (w[1].to_owned(), w[2].to_owned()))
        (w[0],(w[1], w[2]))
    }).collect();

    // println!("{}", rest);
    // println!("{:?}", forks);

    let part1 = lrs
        .iter()
        .cycle()
        .scan("AAA", |state, step| {
            let inst = forks.get(*state).unwrap();
            *state = match step {
                b'L' => &inst.0,
                b'R' => &inst.1,
                _ => panic!("not L or R"),
            };
            Some(*state)
        })
        .take_while_inclusive(|state| *state != "ZZZ")
        .count();

    let periods: Vec<usize> = forks
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|start| {
            let zs: Vec<_> = lrs
                .iter()
                .cycle()
                .scan(start, |state, step| {
                    let inst = forks.get(*state).unwrap();
                    *state = match step {
                        b'L' => &inst.0,
                        b'R' => &inst.1,
                        _ => panic!("not L or R"),
                    };
                    Some(*state)
                })
                .enumerate()
                .filter(|(_, state)| state.ends_with('Z'))
                .take(2)
                .collect();
            assert_eq!(zs[0].1, zs[1].1); //Check that this is a cycle
            assert_eq!(2*(zs[0].0+1), (zs[1].0+1)); //Check that it loops all the way back
            zs[0].0 + 1
        })
        .collect();
    // for p in &periods {
    //     println!("Period: {:?} {}", p, p.1 - p.0)
    // }

    let part2 = periods.iter()
        .fold(BigInt::one(), |acc, p| acc.lcm(&BigInt::from(*p)));
    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test01.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "6");
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
