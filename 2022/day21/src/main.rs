#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use either::Either;
use std::{fs, time::Instant, collections::{HashMap, HashSet}};

use parse_display::{Display, FromStr};


#[derive(Display, FromStr, PartialEq, Debug)]
enum Job<T: Sized> {
    #[display("{lhs} {op} {rhs}")]
    Op{op: char, lhs: T, rhs: T},
    #[display("{0}")]
    Num(isize),
    Humn,
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{name}: {job}")]
struct Monkey<T> {
    name: T,
    job: Job<T>,
}

type Id = [u8;4];
fn to_id(s: &str) -> Id {
    (s.as_bytes()[0..4]).try_into().unwrap()
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
                    Job::Op{lhs, rhs, op} => Job::Op{lhs: to_id(&lhs), rhs: to_id(&rhs), op},
                    Job::Num(v) => Job::Num(v),
                    Job::Humn => {panic!("Did not expect type Humn in input")},
                }
            )))
        }).collect::<Result<_, _>>()
}

fn dependencies(j: &Job<Id>) -> impl Iterator<Item=Id> + '_ {
    match j {
        Job::Op{op:_, lhs, rhs} => Either::Left([*lhs, *rhs].into_iter()),
        _ => Either::Right([].into_iter()),
    }
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
            Job::Op{lhs, rhs, op} => {
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
            _ => {panic!("Num monkey not registered in values")}
        }
    }
    Ok(values[&to_id("root")])
}

fn is_tree(jobs: &HashMap<Id, Job<Id>>) -> bool {
    let mut visited: HashSet<Id> = HashSet::new();
    let mut work: Vec<Id> = vec![to_id("root")];
    while let Some(w) = work.pop() {
        work.extend(dependencies(&jobs[&w]));
        if !visited.insert(w) {
            return false
        }
    };
    true
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let part1 = solution_1(input_s)?;

    let root = to_id("root");
    let mut jobs = parse(input_s)?;
    jobs.insert(to_id("humn"), Job::Humn);
    if let Some(Job::Op{lhs:_, rhs:_, op}) = jobs.get_mut(&root) {
        *op = '-';
    } else {
        panic!("No OP for root");
    };

    //let mut stale: Vec::<Id> = vec![root.clone()];
    // expand multiplications
    // move divisions out
    // (a / b) +- d -> (a +- (d * b))/b
    // (a / b) * d -> (a * d) / b
    // move operations left. move plus left of minus
    // (a+b)+(c+d) => ((a+b)+c)+d
    // (a+b)-

    let part2 = 0;
    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_tree() -> Result<()> {
    for file_name in ["test00.txt", "input.txt"] {
        let input = &fs::read_to_string(file_name)?;
        let jobs = parse(&input)?;
        assert!(is_tree(&jobs));    
    }
    Ok(())
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
