#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use itertools::Itertools;
use nom;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::{Finish, IResult};
use std::cmp::Ordering;
use std::{fs, time::Instant};

#[derive(PartialEq, Debug)]
enum Value {
    List(Vec<Value>),
    Number(i32),
}

fn parse_packet(s: &str) -> IResult<&str, Value> {
    let ll = nom::multi::separated_list0(char(','), parse_packet);
    let lp = nom::combinator::map(
        nom::sequence::delimited(char('['), ll, char(']')),
        |v| Value::List(v),
    );
    let lv = nom::combinator::map(nom::character::complete::i32, |v| Value::Number(v));
    nom::branch::alt((lp, lv))(s)
}

impl std::str::FromStr for Value {
    type Err = nom::error::Error<String>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_packet(s).finish() {
            Ok((_remaining, v)) => Ok(v),
            Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

fn value_iter(v: &Value) -> Box<dyn Iterator<Item=&Value> + '_> {
    match v {
        Value::Number(_) => Box::new(std::iter::once(v)),
        Value::List(v) => Box::new(v.iter()),
    }
}

fn check_value(v1: &Value, v2: &Value) -> Option<bool> {
    match (v1, v2) {
        (Value::Number(a), Value::Number(b)) => {if a == b {None} else {Some(a < b)}}
        (a, b) => {
            let (mut ia, mut ib)=(value_iter(a), value_iter(b));
            loop {
                if let Some(r) = match (ia.next(), ib.next()) {
                    (Some(av), Some(bv)) => check_value(av, bv),
                    (None, None) => return None,
                    (Some(_), None) => Some(false),
                    (None, Some(_)) => Some(true),
                } {return Some(r)};
            };
        }
    }
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<Value> = input_s
        .trim_end()
        .split("\n")
        .filter(|ln| ln.len() > 0)
        .map(|ln| ln.parse())
        .collect::<Result<_, _>>()?;

    let part1: usize = input
        .chunks(2)
        .enumerate()
        .filter_map(|(i, p)| if check_value(&p[0], &p[1]).unwrap() {Some(i + 1)} else {None})
        .sum();

    // sort by index
    let mut input = input;
    input.push(Value::List(vec![Value::List(vec![Value::Number(2)])]));
    input.push(Value::List(vec![Value::List(vec![Value::Number(6)])]));

    let mut ivec: Vec<usize> = (0..input.len()).collect();
    ivec.sort_by(|&a, &b| {
        if check_value(&input[a], &input[b]).unwrap() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    let part2 = ivec
        .iter()
        .enumerate()
        .filter_map(|(i, &v)|if v >= input.len() - 2 {Some(i + 1)} else {None})
        .product::<usize>();

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "13");
    assert!(res[1] == "140");
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
