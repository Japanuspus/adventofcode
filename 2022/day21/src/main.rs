#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use either::Either;
use std::{fs, time::Instant, collections::{HashMap, HashSet}, fmt};

use parse_display::{Display, FromStr};


#[derive(Display, FromStr, PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
struct Id([u8;4]);

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

impl From<&str> for Id {
    fn from(value: &str) -> Self {
        //let value: &str = value.into();
        Id((value.as_bytes()[0..4]).try_into().unwrap())
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
                Id::from(&m.name[..]), 
                match m.job {
                    Job::Op{lhs, rhs, op} => Job::Op{lhs: Id::from(&lhs[..]), rhs: Id::from(&rhs[..]), op},
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


fn is_tree(jobs: &HashMap<Id, Job<Id>>) -> bool {
    let mut visited: HashSet<Id> = HashSet::new();
    let mut work: Vec<Id> = vec![Id::from("root")];
    while let Some(w) = work.pop() {
        work.extend(dependencies(&jobs[&w]));
        if !visited.insert(w) {
            return false
        }
    };
    true
}

fn resolve_in_place(jobs: &mut HashMap<Id, Job<Id>>) {
    // jobs is a tree-structure: we can store job on lookup as is will only be refenced once
    let root = Id::from("root");
    let mut w: Vec<Id> = vec![root];
    let mut resolved: HashSet<Id> = HashSet::new();
    while let Some(id) = w.iter().rev().cloned().next() {
        if match &jobs[&id] {
            Job::Num(_) => true,
            Job::Humn => true,
            Job::Op{lhs, rhs, op} => {
                if resolved.contains(lhs) || resolved.contains(rhs) {
                    if let (Job::Num(a), Job::Num(b)) = (&jobs[lhs], &jobs[rhs]) {
                        jobs.insert(id, Job::Num(compute(*a, *b, *op)));
                    }
                    true
                } else {
                    w.push(*lhs);
                    w.push(*rhs);
                    false
                }
            }
        } {
            resolved.insert(id);
            w.pop();
        }
    }
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let root = Id::from("root");
    let jobs_in = parse(input_s)?;

    let mut jobs = jobs_in.clone();
    resolve_in_place(&mut jobs);
    let part1 = if let Job::Num(v) = jobs[&root] {v} else {panic!("Part1: Root not resolved")};

    let mut jobs = jobs_in.clone();
    jobs.insert(Id::from("humn"), Job::Humn);
    if let Some(Job::Op{lhs:_, rhs:_, op}) = jobs.get_mut(&root) {
        *op = '-';
    } else {
        panic!("No OP for root");
    };
    resolve_in_place(&mut jobs);
    let jobs = jobs;

    let mut pt = &jobs[&root];
    let mut val: isize = 0;
    loop {
        (val, pt) = match pt {
            Job::Humn => {break;}
            Job::Num(_) => {panic!("Trying to solve a number")}
            Job::Op{op, lhs, rhs} => match (op, &jobs[lhs], &jobs[rhs]) {
                ('+', Job::Num(v), b)|('+', b, Job::Num(v)) => (val-v, b),
                ('*', Job::Num(v), b)|('*', b, Job::Num(v)) => (val/v, b),
                ('-', Job::Num(v), b) => (v-val, b), // val = v - b
                ('-', b, Job::Num(v)) => (val+v, b), // val = b - v
                ('/', Job::Num(v), b) => (v/val, b), // val = v/b
                ('/', b, Job::Num(v)) => (val*v, b), // val = b/v
                _ => panic!(),                 
            }
        };
    }
    let part2 = val;

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
    assert!(res[1] == "301");
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
