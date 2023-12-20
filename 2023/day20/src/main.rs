#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use std::{fs, time::Instant, collections::{BTreeSet, BTreeMap, VecDeque}};
use itertools::Itertools;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
enum ModType {
    #[display("%")]
    FlipFlop,
    #[display("&")]
    Conjunction,
    #[display(":")]
    Broadcast,
}

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

    /// Ignore leading and trailing whitespace around `inner`
    pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
        inner: F,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        F: Fn(&'a str) -> IResult<&'a str, O, E>,
    {
        delimited(multispace0, inner, multispace0)
    }
}
//        &nt -> rq, fg, ft, nd, gt, xz

type ModuleSpec<'a>  = ((ModType, &'a str), Vec<&'a str>);

fn parse(s: &str) -> nm::IResult<&str, Vec<ModuleSpec>> {
    let modtype = nm::map_res(nm::opt(nm::one_of("&%")), |c| match c {
        Some('&') => Ok(ModType::Conjunction),
        Some('%') => Ok(ModType::FlipFlop),
        None => Ok(ModType::Broadcast),
        _ => Err(anyhow!("Bad modtype: {:?}", c))
    });
    let modspec = nm::pair(modtype, nm::alpha1);
    let receivers = nm::separated_list1(nm::ws(nm::char(',')),nm::alpha1);
    let module = nm::separated_pair(
        modspec,
        nm::ws(nm::tag("->")),
        receivers);
    nm::separated_list1(nm::newline, module)(s)
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ModuleState<'a> {
    Conjunction(BTreeMap<&'a str, bool>),
    FlipFlop(bool),
    Broadcast,
}

fn update_module_state<'a>(state: &'a mut ModuleState<'a>, src: &'a str, s: bool) -> Option<bool> {
    match state {
        ModuleState::Broadcast => Some(s),
        ModuleState::FlipFlop(v) => if !s {*v = !*v; Some(*v)} else {None},
        ModuleState::Conjunction(mem) => {
            mem.insert(src, s);
            Some(!mem.values().all(|m| *m))
        }   
    }
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let (rest, module_specs) = parse(input_s).map_err(|e| e.to_owned())?;
    assert!(rest.trim_end().len()==0);

    // module does not know receivers
    // module knows receivers
    let mut inputs: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for ((_, src), rcvs) in &module_specs {
        for r in rcvs {
            inputs.entry(r).or_insert(Vec::new()).push(src)
        }
    } 

    // part 1
    let mut modules: BTreeMap<&str, (&[&str], ModuleState)> = 
        module_specs.iter().map(|((m, src), rcvs)| {
            let state = match m {
                ModType::Broadcast => ModuleState::Broadcast,
                ModType::Conjunction => ModuleState::Conjunction(
                    inputs[src].iter().map(|&src| (src, false)).collect()
                ),
                ModType::FlipFlop => ModuleState::FlipFlop(false)
            };
            (*src, (&rcvs[..], state))
        }).collect(); 
    let mut signals: VecDeque<(&str, bool, &str)> = VecDeque::new(); //src, signal, dst
    let mut signal_count: [usize;2] = [0,0]; //true/false
    for _round in 0..1000 {
        signals.push_back(("in", false, "broadcaster"));
        while let Some((src, s, dst)) = signals.pop_front() {
            //println!("{} {} {}", src, s, dst);
            if s {signal_count[0]+=1} else {signal_count[1]+=1};
            //let (rcvs, state) = modules.get_mut(dst).with_context(|| anyhow!("Getting module {} from {}", dst, src))?;
            if let Some((rcvs, state)) = modules.get_mut(dst) {
                if let Some(output) = {
                    match state {
                        ModuleState::Broadcast => Some(s),
                        ModuleState::FlipFlop(v) => if !s {*v = !*v; Some(*v)} else {None},
                        ModuleState::Conjunction(mem) => {
                            mem.insert(src, s);
                            Some(!mem.values().all(|m| *m))
                        }   
                    }
                } {//update_module_state(state, src, s) {
                    for r in rcvs.iter() {
                        signals.push_back((dst, output, r));
                    }    
                }
            } else {
                //output module
            }
        }
    }
    let part1 = signal_count[0]*signal_count[1];

    // part 2
    let mut modules: BTreeMap<&str, (&[&str], ModuleState)> = 
        module_specs.iter().map(|((m, src), rcvs)| {
            let state = match m {
                ModType::Broadcast => ModuleState::Broadcast,
                ModType::Conjunction => ModuleState::Conjunction(
                    inputs[src].iter().map(|&src| (src, false)).collect()
                ),
                ModType::FlipFlop => ModuleState::FlipFlop(false)
            };
            (*src, (&rcvs[..], state))
        }).collect(); 
    let mut signals: VecDeque<(&str, bool, &str)> = VecDeque::new(); //src, signal, dst
    let mut button_count: usize = 0;
    'button: loop {
        signals.push_back(("in", false, "broadcaster"));
        button_count+=1;
        while let Some((src, s, dst)) = signals.pop_front() {
            //println!("{} {} {}", src, s, dst);
            if !s && dst=="rx" {break 'button}
            if let Some((rcvs, state)) = modules.get_mut(dst) {
                if let Some(output) = {
                    match state {
                        ModuleState::Broadcast => Some(s),
                        ModuleState::FlipFlop(v) => if !s {*v = !*v; Some(*v)} else {None},
                        ModuleState::Conjunction(mem) => {
                            mem.insert(src, s);
                            Some(!mem.values().all(|m| *m))
                        }   
                    }
                } {//update_module_state(state, src, s) {
                    for r in rcvs.iter() {
                        signals.push_back((dst, output, r));
                    }    
                }
            } else {
                //output module
            }
        }
    }
    let part2 = button_count;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "32000000");
    assert_eq!(res[1], "0");

    let input = &fs::read_to_string("test02.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "11687500");
    assert_eq!(res[1], "0");

    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    let start = Instant::now();
    let (res, time) = loop { // run warmup for 100ms
        let lap = Instant::now();
        let res = solution(&input)?;
        if start.elapsed().as_millis()>100 {break (res, lap.elapsed())};
    };
    println!( "({} us)\nPart 1: {}\nPart 2: {}", time.as_micros(), res[0], res[1]);
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
