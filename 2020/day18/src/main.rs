use anyhow::{Error, Result};
use apply::Apply;
use regex::Regex;
use std::{fs, iter::once};

#[derive(Debug)]
enum Token {
    Mul,
    Add,
    PStart,
    PEnd,
    Number(isize),
}

struct Parser {
    re: Regex,
}

impl Parser {
    fn parse<'a>(&'a self, ln: &'a str) -> impl Iterator<Item = Token> + 'a {
        self.re.find_iter(ln).map(|m| match m.as_str() {
            "*" => Token::Mul,
            "+" => Token::Add,
            "(" => Token::PStart,
            ")" => Token::PEnd,
            s => Token::Number(s.parse::<isize>().unwrap()),
        })
    }
    fn new() -> Result<Self> {
        Ok(Parser {
            re: regex::Regex::new(r"\+|\*|\(|\)|[0-9]+")?,
        })
    }
}

fn eval1(ts: &mut impl Iterator<Item = Token>) -> isize {
    let mut op: Option<Token> = None;
    let mut v: isize = 0; // allows () for 0 but ...

    while let Some(t) = ts.next() {
        if let Some(value) = match t {
            Token::PEnd => break,
            Token::PStart => Some(eval1(ts)),
            Token::Number(b) => Some(b),
            _ => {
                op = Some(t);
                None
            }
        } {
            if let Some(o) = op {
                v = match o {
                    Token::Add => v + value,
                    Token::Mul => v * value,
                    _ => panic!("non-operator token in op"),
                };
                op = None
            } else {
                v = value;
            }
        }
    }
    v
}

fn eval2_value(ts: &mut impl Iterator<Item = Token>) -> Result<isize> {
    if let Some(t) = ts.next() {
        match t {
            Token::PStart => Ok(eval2(ts)?),
            Token::Number(b) => Ok(b),
            _ => Err(Error::msg("expected value")),
        }
    } else {
        Err(Error::msg("empty expression"))
    }
}

// this could be split into eval2_sum and eval2, preferably with peek on tokens
fn eval2(ts: &mut impl Iterator<Item = Token>) -> Result<isize> {
    let mut p: isize = 1;
    let mut s: isize = eval2_value(ts)?;
    while let Some(t) = ts.next() {
        match t {
            Token::Add => {
                s += eval2_value(ts)?;
            }
            Token::Mul => {
                p *= s;
                s = eval2_value(ts)?;
            }
            Token::PEnd => break,
            _ => Err(Error::msg("Unexpected token"))?,
        }
    }
    Ok(p * s)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let parser = Parser::new()?;
    println!(
        "Part 1: {}",
        input
            .lines()
            .map(|ln| eval1(&mut parser.parse(ln)))
            .sum::<isize>()
    );
    println!(
        "Part 2: {}",
        input
            .lines()
            .map(|ln| eval2(&mut parser.parse(ln)))
            .sum::<Result<isize, _>>()?
    );
    Ok(())
}
