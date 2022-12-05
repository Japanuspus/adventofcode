#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use std::{
    collections::{HashMap, VecDeque},
    fs, str,
};

use parse_display::{Display, FromStr};

#[derive(Debug, Display, FromStr)]
#[display("move {n} from {o} to {d}")]
struct Step {
    n: usize,
    o: usize,
    d: usize,
}

/// back of deque is top of stack
fn parse_stacks(s: &str) -> Vec<VecDeque<u8>> {
    let mut stacks: HashMap<usize, VecDeque<u8>> = HashMap::new();
    for ln in s.split("\n").map(|cs| cs.as_bytes()) {
        if ln[1].is_ascii_digit() {
            break;
        }
        for (i, c) in ln.chunks(4).enumerate().filter_map(|(i, ch)| {
            let c = ch[1];
            if c.is_ascii_whitespace() { None } else { Some((i, c)) }
        }) {
            stacks.entry(i).or_default().push_front(c);
        }
    }
    (0..stacks.len())
        .filter_map(|i| stacks.remove(&i))
        .collect()
}

fn read_stacks(stacks: &mut Vec<VecDeque<u8>>) -> String {
    String::from_utf8(stacks.iter_mut().filter_map(|s| s.pop_back()).collect()).unwrap()
}

fn solution(input_s: &str) -> Result<(String, String)> {
    let mut input = input_s.split("\n\n");
    let stacks0 = parse_stacks(input.next().unwrap());
    let moves: Vec<Step> = input.next().unwrap().trim()
        .split('\n')
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_, _>>()?;

    let mut stacks: Vec<_> = stacks0.iter().map(|d| d.clone()).collect();
    for m in &moves {
        for _ in 0..m.n {
            let v = stacks[m.o - 1].pop_back().unwrap();
            stacks[m.d - 1].push_back(v);
        }
    }
    let part1 = read_stacks(&mut stacks);

    let mut stacks = stacks0;
    for m in &moves {
        let buf: Vec<u8> = (0..m.n)
            .filter_map(|_| stacks[m.o - 1].pop_back())
            .collect();
        for v in buf.iter().rev() {
            stacks[m.d - 1].push_back(*v)
        }
    }
    let part2 = read_stacks(&mut stacks);

    Ok((part1, part2))
}

#[test]
fn test_solution() -> Result<()> {
    let res = solution(&fs::read_to_string("test00.txt")?)?;
    println!("Part 1: {}\nPart 2: {}", res.0, res.1);
    assert!(res.0 == "CMZ");
    assert!(res.1 == "MCD");
    Ok(())
}

fn main() -> Result<()> {
    let res = solution(&fs::read_to_string("input.txt")?)?;
    println!("Part 1: {}\nPart 2: {}", res.0, res.1);
    Ok(())
}
