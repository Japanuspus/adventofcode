use anyhow::Context;
use nom::branch::alt;
use nom::sequence::delimited;
use nom::character::complete::{anychar, char};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::character::complete::digit1;
use nom::combinator::recognize;
use nom::combinator::map_res;
use nom::bytes::complete::tag;
use nom::sequence::separated_pair;
use nom::{IResult, Finish};
use std::collections::HashMap;
use std::fs;
use anyhow::Result;
use anyhow::Error;
// use itertools::Itertools;
// use parse_display::{FromStr};
// use regex::Regex;
// use apply::Also;
// use num::{BigInt, Integer};

#[derive(Debug)]
enum Rule {
    single(char),
    rule_lists(Vec<Vec<usize>>)
}

#[derive(Debug)]
struct Input<'a> {rules: HashMap<usize, Rule>, messages: Vec<&'a str>}

fn parse_number(input : &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_rule(s: &str) -> Result<(usize, Rule)> {
    let rule_list = separated_list1(tag(" "), parse_number);
    let rule1 = map(separated_list1(tag(" | "), rule_list), |v| Rule::rule_lists(v));
    let rule2 = map(delimited(char('"'), anychar, char('"')), |c| Rule::single(c)); 
    let rule = alt((rule1, rule2));
    let mut line = separated_pair(parse_number, tag(": "), rule);
    let res = line(s)
    .finish()
    .map(|(_, r)| r)
    // https://github.com/Geal/nom/blob/master/doc/nom_recipes.md#implementing-fromstr
    .map_err(|nom::error::Error { input, code }| nom::error::Error{input: input.to_string(), code});
    Ok(res?)
}

fn parse(s: &str) -> Result<Input> {
    let mut parts = s.split("\n\n");
    let rules: HashMap<_,_> = parts.next().ok_or(Error::msg("no rules"))?.lines().enumerate()
    .map(|(i, ln)| parse_rule(ln).with_context(|| format!("While parsing line {}: {}", i, ln))).collect::<Result<_,_>>()?;
    let messages = parts.next().ok_or(Error::msg("no rules"))?.lines().collect();
    Ok(Input{rules, messages})
}

fn main() -> Result<()> {
    let inputs = fs::read_to_string("input.txt")?;
    let input = parse(&inputs);
 
    println!("Part 1: {:?}", &input);
    Ok(())
}
