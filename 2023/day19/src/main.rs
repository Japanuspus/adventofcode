#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use std::{fs, time::Instant, collections::{hash_map, HashMap}};
use itertools::Itertools;
use parse_display::{Display, FromStr};

mod nm {
    pub use nom::multi::*;
    pub use nom::sequence::*;
    pub use nom::character::complete::*;
    pub use nom::bytes::complete::*;
    pub use nom::error::*;
    pub use nom::combinator::*;
    pub use nom::IResult;

    /// A combinator that takes a parser `inner` and produces a parser that 
    /// also consumes both leading and trailing whitespace, 
    /// returning the output of `inner`.
    pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
        inner: F,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        F: Fn(&'a str) -> IResult<&'a str, O, E>,
    {
        delimited(multispace0, inner, multispace0)
    }
}

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display(style="lowercase")]
enum Xmas {X=0, M=1, A=2, S=3,}
type WF<'a> = (&'a str, Vec<(Option<(Xmas, char, u16)>, &'a str)>);

//px{a<2006:qkq,m>2090:A,rfg}
fn parse_wf(s: &str) -> nm::IResult<&str, Vec<WF>> {
    let cond_entry = nm::tuple((
        nm::map_res(nm::alpha1, |v: &str| v.parse::<Xmas>()), 
        nm::one_of("<>"), 
        nm::u16)
    );
    let cond = nm::terminated(cond_entry,  nm::char(':'));
    let rule = nm::pair(nm::opt(cond), nm::alpha1);
    let rules = nm::delimited(nm::char('{'), nm::separated_list1(nm::char(','), rule), nm::char('}'));
    let workflow = nm::pair(nm::alpha1, rules);
    nm::separated_list1(nm::newline, workflow)(s)
}

type Material = Vec<u16>;

fn parse_material(s: &str) -> nm::IResult<&str, Vec<Material>> {
    let val = nm::preceded(nm::pair(nm::anychar, nm::char('=')), nm::u16);
    let values = nm::separated_list1(nm::char(','), val);
    let wvalues = nm::delimited(nm::char('{'), values, nm::char('}'));
    nm::separated_list1(nm::newline, wvalues)(s)
}

fn parse(s: &str) -> nm::IResult<&str, (Vec<WF>, Vec<Material>)> {
    nm::separated_pair(
        parse_wf, 
        nm::tag("\n\n"), 
        parse_material
    )(s)
}

type Region = [(u16, u16);4];
const ALL: Region = [(1, 4000);4];

fn solution(input_s: &str) -> Result<[String; 2]> {
    let (rest, (wf, materials)) = parse(input_s).map_err(|e| e.to_owned())?;
    assert!(rest.trim().len()==0);
    let workflows: HashMap<&str, Vec<_>> = wf.into_iter().collect();

    let mut part1: usize = 0;
    for m in &materials {
        let mut wf: &str = "in";
        let acc = loop {
            if wf=="A" {break true;};
            if wf=="R" {break false;};
            let wf_spec = workflows
                .get(wf).with_context(|| format!("Looking for wf {}", wf))?;
            wf = wf_spec.iter().filter_map(|(rule_option, next_wf)| {
                if let Some((an, on, b)) = rule_option {
                    let a = m[*an as usize];
                    let ok = if *on=='<' {a<*b} else {a>*b};
                    if !ok {return None}
                }
                Some(next_wf)
            })
            .next().with_context(|| format!("No match for {:?} within {:?}", m, wf_spec))?;
        };
        if acc {
            part1 += m.iter().sum::<u16>() as usize;
        }
    }

    // Part 2 
    let mut part2: usize = 0;
    let mut work: Vec<(&str, Region)> = vec![("in", ALL)];
    while let Some((wf, reg)) = work.pop() {
        if wf=="A" {
            part2+=reg.iter().map(|(a,b)| (b+1-a) as usize).product::<usize>(); 
            continue
        }
        if wf=="R" {continue}
        let wf_spec = workflows
            .get(wf).with_context(|| format!("Looking for wf {}", wf))?;
        let mut remain = reg;
        for (cond, next_wf) in wf_spec {
            if let Some((an, on, b)) = cond {
                let mut next_reg: Region = remain;
                let i = *an as usize;
                if *on=='>' {
                    next_reg[i].0 = b+1;
                    remain[i].1 = *b;
                } else {
                    next_reg[i].1 = b-1;
                    remain[i].0 = *b;
                }
                if next_reg[i].0<=next_reg[i].1 {
                    work.push((next_wf, next_reg));
                }
                if remain[i].0>remain[i].1 {break};                
            } else {
                work.push((next_wf, remain))
            }
        }
    }

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "19114");
    assert_eq!(res[1], "167409079868000");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    let start = Instant::now();
    let (res, time) = loop {
        let lap = Instant::now();
        let res = solution(&input)?;
        if start.elapsed().as_millis()>300 {break (res, lap.elapsed())};
    };
    println!( "({} us)\nPart 1: {}\nPart 2: {}", time.as_micros(), res[0], res[1]);
    Ok(())
}
