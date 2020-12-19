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
use nom::{IResult};
use std::collections::HashMap;
use std::fs;
use anyhow::Result;
use anyhow::Error;
// use itertools::Itertools;
// use parse_display::{FromStr};
// use regex::Regex;
// use apply::Also;
// use num::{BigInt, Integer};

type RuleList = (usize, usize);
#[derive(Debug)]
enum Rule {
    single(char),
    rule_lists(Vec<RuleList>)
}

struct Input<'a> {rules: HashMap<usize, Rule>, messages: Vec<&'a str>}

fn parse_number(input : &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_rule(s: &str) -> IResult<&str, (usize, Rule)> {
    //let number = map_res(recognize(digit1), str::parse);
    let rule_list = separated_pair(parse_number, tag(" "), parse_number);
    let rule1 = map(separated_list1(tag(" | "), rule_list), |v| Rule::rule_lists(v));
    let rule2 = map(delimited(char('"'), anychar, char('"')), |c| Rule::single(c)); 
    let rule = alt((rule1, rule2));
    let mut line = separated_pair(parse_number, tag(": "), rule);
    line(s)
}


// fn parse(s: &str) -> Result<Input> {
//     let mut parts = s.split("\n\n");
//     let rules: HashMap<_,_> = parts.next().ok_or(Error::msg("no rules"))?.lines().map(|ln| parse_rule(ln)).collect::<Result<_,_>>()?;
//     let messages = parts.next().ok_or(Error::msg("no rules"))?.lines().collect();
//     Ok(Input{rules, messages})
// }

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
 
    println!("Part 1: {}", input.len());
    println!("{:?}", parse_rule("2: \"a\""));
    println!("{:?}", parse_rule("4: 12 3"));
    println!("{:?}", parse_rule("6: 12 3 | 4 8"));
    Ok(())
}
