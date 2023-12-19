#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use nom::Finish;
use std::{fs, time::Instant, collections::{hash_map, HashMap}};
use itertools::Itertools;

//px{a<2006:qkq,m>2090:A,rfg}
// use parse_display::{Display, FromStr};

// #[derive(Display, FromStr, PartialEq, Debug)]
// enum Direction {
//     #[display("forward")]
//     Forward,
// }

// #[derive(Debug, Display, FromStr)]
// #[display("{direction} {distance}")]
// struct Step {
//     direction: Direction,
//     distance: i32,
// }

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

//px{a<2006:qkq,m>2090:A,rfg}


type WF<'a> = (&'a str, Vec<(Option<(char, char, u16)>, &'a str)>);

fn parse_wf(s: &str) -> nm::IResult<&str, Vec<WF>> {
    let cond_entry = nm::tuple((nm::anychar, nm::one_of("<>"), nm::u16));
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

fn region_checked(reg: Region) -> Option<Region> {
    if reg.iter().any(|r| r.0>r.1) {None} else {Some(reg)}
}

fn region_intersect(r1: &Region, r2: &Region) -> Option<Region> {
    let mut res=ALL;
    for i in 0..4 {
        res[i] = (r1[i].0.max(r2[i].0), r1[i].1.min(r2[i].1))
    }
    region_checked(res)
}

fn region_split_cond(r: &Region, cond: &Option<(char, char, u16)>) 
-> (Option<Region>, Option<Region>) {
    if let Some((an, on, b)) = cond {
        let mut r_cond = *r;
        let mut r_other = *r;
        let i = match an {'x'=>0, 'm'=>1, 'a'=>2, 's'=>3, _=>panic!()};
        if *on=='>' {
            r_cond[i].0 = b+1;
            r_other[i].1 = *b;
        } else {
            r_cond[i].1 = b-1;
            r_other[i].0 = *    b;
        }
        (region_checked(r_cond), region_checked(r_other))
    } else {
        (Some(*r), None)
    }
}

fn region_size(r: &Region) -> usize {
    r.iter().map(|(a,b)| (b+1-a) as usize).product()
}

fn union_size(grp: &[Region]) -> usize {
    grp.iter().map(|r| region_size(r)).sum()
}

fn add_area(grp: &[Region], a: &Region) -> usize {
    // additional volume count from adding this region
    0
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let (rest, (wf, materials)) = parse(input_s).map_err(|e| e.to_owned())?;
    assert!(rest.trim().len()==0);
    let workflows: HashMap<&str, Vec<(Option<(char, char, u16)>, &str)>> = wf.into_iter().collect();

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
                    let a = m[match an {'x'=>0, 'm'=>1, 'a'=>2, 's'=>3, _=>panic!()}];
                    let ok = match on {
                        '<' => a<*b,
                        '>' => a>*b,
                        _ => panic!()
                    };
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

    // ** part 2 
    let mut accepted: Vec<Region> = Vec::new();
    let mut rejected: Vec<Region> = Vec::new();
    let mut work: Vec<(&str, Region)> = vec![("in", ALL)];
    while let Some((wf, reg)) = work.pop() {
        if wf=="A" {
            accepted.push(reg); 
            continue;
        }
        if wf=="R" {
            rejected.push(reg); 
            continue;
        }
        let wf_spec = workflows
            .get(wf).with_context(|| format!("Looking for wf {}", wf))?;
        let mut remain = reg;
        for (cond, next_wf) in wf_spec {
            let (a, b) = region_split_cond(&remain, cond);
            if let Some(next_reg) = a {
                work.push((next_wf, next_reg));
            }
            if let Some(r) = b {
                remain=r;
            } else {
                break;
            }
        }
    }

    // println!("{} -> {:?}", accepted.len(), &accepted);
    // println!("{} -> {:?}", rejected.len(), &rejected);

    let part2 = union_size(&accepted[..]);
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
    // for _ in 0..20 {
    //     solution(&input)?;
    // } //warmup
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

// // Make it simple to compare timing for multiple solutions
// type Solution = dyn Fn(&str) -> Result<[String; 2]>;
// const SOLUTIONS: [(&str, &Solution); 1] = [("Original", &solution)];

// #[test]
// fn test_solution() -> Result<()> {
//     let input = &fs::read_to_string("test00.txt")?;
//     for (name, solution) in SOLUTIONS {
//         let res = solution(&input).with_context(|| format!("Running solution {}", name))?;
//         println!("---\n{}\nPart 1: {}\nPart 2: {}", name, res[0], res[1]);
//         assert_eq!(res[0], "0");
//         assert_eq!(res[1], "0");
//     }
//     Ok(())
// }

// fn main() -> Result<()> {
//     let input = &fs::read_to_string("input.txt")?;
//     for (_, solution) in SOLUTIONS.iter().cycle().take(10) {
//         solution(&input)?;
//     } //warmup
//     for (name, solution) in SOLUTIONS {
//         let start = Instant::now();
//         let res = solution(&input)?;
//         println!(
//             "---\n{} ({} us)\nPart 1: {}\nPart 2: {}",
//             name, start.elapsed().as_micros(), res[0], res[1],
//         );
//     }
//     Ok(())
// }
