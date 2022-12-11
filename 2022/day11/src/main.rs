#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use itertools::Itertools;
use std::{fs, time::Instant};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
enum Value {
    #[display("old")]
    This,
    #[display("{0}")]
    Other(usize)
}

#[derive(Debug, Display, FromStr)]
#[display(
"Monkey {id}:
  Starting items: {items}
  Operation: new = {opa} {op} {opb}
  Test: divisible by {modulo}
    If true: throw to monkey {totrue}
    If false: throw to monkey {tofalse}")]
struct MonkeyInput {
    id: usize,
    items: String,
    opa: Value,
    op: char,
    opb: Value,
    modulo: usize,
    totrue: usize,
    tofalse: usize,
}


//fn monkey_business(input: &Vec<MonkeyInput>, items: Vec<Vec<usize>>, )

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<MonkeyInput> = input_s.trim_end()
        .split("\n\n")
        .map(|s| s.parse().with_context(|| format!("Parsing >{}<", s)))
        .collect::<Result<_, _>>()?;

    let items0: Vec<Vec<usize>> = input
    .iter().map(|m| 
        m.items.split(", ").map(|v| v.parse::<usize>().unwrap()).collect()
    ).collect();

    let mut items = items0.clone();
    let mut inspection_counts = vec![0usize;input.len()];
    for _ in 0..20 {
        for m in &input {
            // grab list of items for this monkey
            let ws=std::mem::replace(&mut items[m.id], Vec::new());
            inspection_counts[m.id]+=ws.len();
            for w in ws {
                //new worry
                let a = match m.opa {Value::This => w, Value::Other(v) => v};
                let b = match m.opb {Value::This => w, Value::Other(v) => v};
                let w = match m.op {'+' => a+b, '*' => a*b, _ => panic!()};
                let w = w/3;
                // destination
                let dst = if (w%m.modulo)==0 {m.totrue} else {m.tofalse};
                items[dst].push(w);
            }
        }
    }
    let part1 = inspection_counts.iter().sorted().rev().take(2).product::<usize>();

    let mut items = items0.clone();
    let mut inspection_counts = vec![0usize;input.len()];
    for _ in 0..10000 {
        for m in &input {
            // grab list of items for this monkey
            let ws=std::mem::replace(&mut items[m.id], Vec::new());
            inspection_counts[m.id]+=ws.len();
            for w in ws {
                //new worry
                let a = match m.opa {Value::This => w, Value::Other(v) => v};
                let b = match m.opb {Value::This => w, Value::Other(v) => v};
                let w = match m.op {'+' => a+b, '*' => a*b, _ => panic!()};
                // destination
                let dst = if (w%m.modulo)==0 {m.totrue} else {m.tofalse};
                items[dst].push(w);
            }
        }
    }
    let part2 = inspection_counts.iter().sorted().rev().take(2).map(|v| *v as u128).product::<u128>();

    Ok([part1.to_string(), part2.to_string()])
}

// Make it simple to compare timing for multiple solutions
type Solution = dyn Fn(&str) -> Result<[String; 2]>;
const SOLUTIONS: [(&str, &Solution); 1] = [("Original", &solution)];

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    for (name, solution) in SOLUTIONS {
        let res = solution(&input).with_context(|| format!("Running solution {}", name))?;
        println!("---\n{}\nPart 1: {}\nPart 2: {}", name, res[0], res[1]);
        assert!(res[0] == "10605");
        assert!(res[1] == "2713310158");
    }
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    for (_, solution) in SOLUTIONS.iter().cycle().take(10) {
        solution(&input)?;
    } //warmup
    for (name, solution) in SOLUTIONS {
        let start = Instant::now();
        let res = solution(&input)?;
        println!(
            "---\n{} ({} us)\nPart 1: {}\nPart 2: {}",
            name, start.elapsed().as_micros(), res[0], res[1],
        );
    }
    Ok(())
}
