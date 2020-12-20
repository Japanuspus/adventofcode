use std::iter::FromIterator;
use itertools::Itertools;
use std::collections::HashSet;
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

#[derive(Debug, Clone)]
enum Rule {
    Single(char),
    RuleLists(Vec<Vec<usize>>)
}

type RuleMap = HashMap<usize, Rule>;

#[derive(Debug)]
struct Input<'a> {rules: RuleMap, messages: Vec<&'a str>}

fn parse_number(input : &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_rule(s: &str) -> Result<(usize, Rule)> {
    let rule_list = separated_list1(tag(" "), parse_number);
    let rule1 = map(separated_list1(tag(" | "), rule_list), |v| Rule::RuleLists(v));
    let rule2 = map(delimited(char('"'), anychar, char('"')), |c| Rule::Single(c)); 
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

// part 2
fn possible<'a>(idx: usize, s: &'a str, rule_map: &RuleMap) -> Vec<&'a str> {
    if let Some(a) = s.chars().next() {
        match rule_map.get(&idx).unwrap() {
            Rule::Single(b) => if &a==b {vec![&s[1..]]} else {Vec::new()},
            Rule::RuleLists(rls) => {
                let mut res: Vec<Vec<&str>> = Vec::new();
                for rl in rls {
                    let mut pos: Vec::<&str> = vec![s];
                    for rule_idx in rl {
                        pos = pos.iter().map(|p| possible(*rule_idx, p, rule_map)).concat();
                    }
                    res.push(pos);
                };
                res.concat()
            }
        }
    } else {Vec::new()}
}

// part 1
fn expand(idx: usize, rule_map: &RuleMap) -> HashSet<String> {
    // this could have used a cache
    match rule_map.get(&idx).unwrap() {
        Rule::Single(c) => HashSet::from_iter(vec![c.to_string()].into_iter()),
        Rule::RuleLists(vv) => {
            let mut res = HashSet::new();
            for v in vv {
                let mut tmp = HashSet::new();
                tmp.insert("".to_string());
                for append in v.iter().map(|&i| expand(i, rule_map)) {
                    tmp = tmp.iter().flat_map(|s0| append.iter().map(move |s1| [s0, s1].iter().join("") )).collect();
                }
                res.extend(tmp.drain())
            }
            res
        }
    }
}

fn main() -> Result<()> {
    let inputs = fs::read_to_string("input.txt")?;
    let input = parse(&inputs)?;

    // let all_possible = expand(0, &input.rules);
    // println!("Part 1: possible {}, matches: {}", all_possible.len(), input.messages.iter().filter(|m| all_possible.contains(&m[..])).count());
    // println!("Part 2: expand 42: {:?}", expand(42, &input.rules));

    println!("Part 1 take 2: {}", input.messages.iter().filter(|m| possible(0, m, &input.rules).iter().any(|s| s.len()==0)).count());

    let mut rule_map2 = input.rules.clone();
    //8: 42 | 42 8
    //11: 42 31 | 42 11 31
    rule_map2.insert(8, Rule::RuleLists(vec![vec![42], vec![42, 8]]));
    rule_map2.insert(11, Rule::RuleLists(vec![vec![42, 31], vec![42, 11, 31]]));
    println!("Part 2 {}", input.messages.iter().filter(|m| possible(0, m, &rule_map2).iter().any(|s| s.len()==0)).count());

    Ok(())
}
