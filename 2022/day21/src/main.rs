#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use std::{fs, time::Instant, collections::HashMap};

use parse_display::{Display, FromStr};

type Id = [u8;4];
fn to_id(s: &str) -> Id {
    (s.as_bytes()[0..4]).try_into().unwrap()
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{lhs} {op} {rhs}")]
struct Operation<T> {
    lhs: T,
    rhs: T,
    op: char,
}

#[derive(Display, FromStr, PartialEq, Debug)]
enum Job<T> {
    #[display("{0}")]
    Op(Operation<T>),
    #[display("{0}")]
    Num(isize),
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{name}: {job}")]
struct Monkey<T> {
    name: String,
    job: Job<T>,
}

fn compute(a: isize, b: isize, op: char) -> isize {
    match op {
        '+' => a+b,
        '*' => a*b,
        '-' => a-b,
        '/' => a/b,
        _ => {panic!("Invalid operand")}
    }
}

fn parse(input_s: &str) -> Result<HashMap<Id, Job<Id>>> {
    input_s.trim_end()
        .split("\n")
        .map(|s| {
            s
            .parse::<Monkey<String>>()
            .with_context(|| format!("Parsing {}", s))
            .and_then(|m| Ok((
                to_id(&m.name), 
                match m.job {
                    Job::Op(Operation{lhs, rhs, op}) => Job::Op(Operation{lhs: to_id(&lhs), rhs: to_id(&rhs), op}),
                    Job::Num(v) => Job::Num(v),
                }
            )))
        }).collect::<Result<_, _>>()
}

fn solution_1(input_s: &str) -> Result<isize> {
    let jobs = parse(input_s)?;
    
    let mut values: HashMap<Id, isize> = jobs.iter().filter_map(|(name, job)| {
            match job {
                Job::Num(v) => Some((*name, *v)),
                _ => None,
            }
        }).collect();
    let mut w: Vec<Id> = vec![to_id("root")];
    while let Some(&d) = w.iter().rev().next() {
        if values.contains_key(&d) {w.pop(); continue;}
        match &jobs[&d] {
            Job::Op(Operation{lhs, rhs, op}) => {
                match (values.get(lhs), values.get(rhs)) {
                    (None, None) => {w.push(*lhs); w.push(*rhs);}
                    (None, _) => {w.push(*lhs);}
                    (_, None) => {w.push(*rhs);}
                    (Some(a), Some(b)) => {
                        values.insert(d, compute(*a, *b, *op));
                        w.pop();
                    }
                }
            },
            Job::Num(_) => {panic!("Num monkey not registered in values")}
        }
    }
    Ok(values[&to_id("root")])
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let part1 = solution_1(input_s)?;



    let part2 = 0;
    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "152");
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
