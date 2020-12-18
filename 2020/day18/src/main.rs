use std::{fs, iter::once};
use anyhow::Result;
use apply::Apply;
use regex::Regex;

#[derive(Debug)]
enum Token {
    Mul,
    Add,
    PStart,
    PEnd,
    Number(isize)
}

struct Parser {
    re: Regex,
}

impl Parser {
    fn parse<'a>(&'a self, ln: &'a str) -> impl Iterator<Item=Token> + 'a {
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
            re: regex::Regex::new(r"\+|\*|\(|\)|[0-9]+")?
        })
    }
}

fn eval1(ts: &mut impl Iterator<Item=Token>) -> isize {
    let mut op: Option<Token> = None;
    let mut v: isize = 0;  // allows () for 0 but ...

    while let Some(t) = ts.next() {
        if let Some(value) = match t {
            Token::PEnd => break,
            Token::PStart => Some(eval1(ts)),
            Token::Number(b) => Some(b),
            _ => {op = Some(t); None}, 
        } {
            if let Some(o) = op {
                v = match o {
                    Token::Add => v+value,
                    Token::Mul => v*value,
                    _ => panic!("non-operator token in op"),
                };
                op = None
            } else {
                v = value;
            }
        }
    };
    v
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let parser = Parser::new()?;
    println!("Part 1: {}", 
        input.lines().map(|ln| eval1(&mut parser.parse(ln))).sum::<isize>());
    Ok(())
}
