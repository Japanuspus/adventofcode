#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use itertools::{Itertools, EitherOrBoth};
use nom;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::{Finish, IResult};
use std::cmp::Ordering;
use std::{fs, time::Instant};


fn parse_packet(s: &str) -> IResult<&str, Value> {
    let ll = nom::multi::separated_list0(char(','), parse_packet);
    let lp = nom::combinator::map(
        nom::sequence::delimited(char('['), ll, char(']')),
        |v| Value::List(v),
    );
    let lv = nom::combinator::map(nom::character::complete::i32, |v| Value::Number(v));
    nom::branch::alt((lp, lv))(s)
}

#[derive(PartialEq, Debug)]
enum Value {
    List(Vec<Value>),
    Number(i32),
}

impl Value {
    fn iter(&self) -> Box<dyn Iterator<Item=&Self> + '_> {
        match self {
            Value::Number(_) => Box::new(std::iter::once(self)),
            Value::List(v) => Box::new(v.iter()),
        }
    }
}

fn compare_value(v1: &Value, v2: &Value) -> Ordering {
    v1.iter().zip_longest(v2.iter()).map(|z|
        match z {
            EitherOrBoth::Both(Value::Number(a), Value::Number(b)) => a.cmp(b),
            EitherOrBoth::Both(a, b) => compare_value(a, b),
            EitherOrBoth::Left(_) => Ordering::Greater,
            EitherOrBoth::Right(_) => Ordering::Less,
       }
    ).skip_while(|c| c.is_eq()).next().unwrap_or(Ordering::Equal)
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<Value> = input_s
        .trim_end()
        .split("\n")
        .filter_map(|ln| if ln.len() > 0 {Some(parse_packet(ln).unwrap().1)} else {None})
        .collect();

    let part1: usize = input
        .chunks(2)
        .enumerate()
        .filter_map(|(i, p)| if compare_value(&p[0], &p[1])==Ordering::Less {Some(i+1)} else {None})
        .sum();

    // sort by index
    let mut input = input;
    input.push(Value::List(vec![Value::List(vec![Value::Number(2)])]));
    input.push(Value::List(vec![Value::List(vec![Value::Number(6)])]));

    let mut ivec: Vec<usize> = (0..input.len()).collect();
    ivec.sort_by(|&a, &b| compare_value(&input[a], &input[b]));
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

