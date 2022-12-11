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


fn monkey_business(input: &Vec<MonkeyInput>, mut items: Vec<Vec<usize>>, rounds: usize, divisor: usize) -> usize {
    let mut inspection_counts = vec![0usize;input.len()];
    let bigmod = input.iter().map(|m| m.modulo).product::<usize>();
    for _ in 0..rounds {
        for m in input {
            // grab list of items for this monkey
            let ws=std::mem::replace(&mut items[m.id], Vec::new());
            inspection_counts[m.id]+=ws.len();
            for w in ws {
                //new worry
                let a = match m.opa {Value::This => w, Value::Other(v) => v};
                let b = match m.opb {Value::This => w, Value::Other(v) => v};
                let w = match m.op {'+' => a+b, '*' => a*b, _ => panic!()};
                let w = w/divisor;
                // destination
                let dst = if (w%m.modulo)==0 {m.totrue} else {m.tofalse};
                items[dst].push(w%bigmod);
            }
        }
    }
    inspection_counts.iter().sorted().rev().take(2).product::<usize>()
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<MonkeyInput> = input_s.trim_end()
        .split("\n\n")
        .map(|s| s.parse().with_context(|| format!("Parsing >{}<", s)))
        .collect::<Result<_, _>>()?;

    let items: Vec<Vec<usize>> = input
    .iter().map(|m| 
        m.items.split(", ").map(|v| v.parse::<usize>().unwrap()).collect()
    ).collect();

    let part1 = monkey_business(&input, items.clone(), 20, 3);
    let part2 = monkey_business(&input, items, 10000, 1);
    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input).with_context(|| format!("Running solution"))?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "10605");
    assert!(res[1] == "2713310158");
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
        start.elapsed().as_micros(), res[0], res[1],
    );
    Ok(())
}
