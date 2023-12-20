#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use num::Integer;
use std::{fs, time::Instant, collections::{BTreeSet, BTreeMap, VecDeque, HashSet, HashMap}};
use itertools::Itertools;

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

type ModuleSpec<'a>  = ((Option<char>, &'a str), Vec<&'a str>);

fn parse(s: &str) -> nm::IResult<&str, Vec<ModuleSpec>> {
    let modtype = nm::opt(nm::one_of("&%"));
    let modspec = nm::pair(modtype, nm::alpha1);
    let receivers = nm::separated_list1(nm::ws(nm::char(',')),nm::alpha1);
    let module = nm::separated_pair(
        modspec,
        nm::ws(nm::tag("->")),
        receivers);
    nm::separated_list1(nm::newline, module)(s)
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

fn is_init(m: &ModuleState<'_>) -> bool {
    match m {
        ModuleState::Broadcast => true,
        ModuleState::FlipFlop(v) => !v,
        ModuleState::Conjunction(mem) => mem.values().all(|m| !*m),
    }
}

fn init_modules<'a>(
    module_specs: &'a Vec<ModuleSpec<'a>>, 
    inputs: &'a BTreeMap<&str, Vec<&str>>
) -> BTreeMap<&'a str, (&'a [&'a str], ModuleState<'a>)> {
    module_specs.iter().map(|((m, src), rcvs)| {
        let state = match *m {
            None => ModuleState::Broadcast,
            Some('&') => ModuleState::Conjunction(
                inputs[src].iter().map(|&src| (src, false)).collect()
            ),
            Some('%') => ModuleState::FlipFlop(false),
            _ => {panic!()}
        };
        (*src, (&rcvs[..], state))
    }).collect()
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
    let mut modules = init_modules(&module_specs, &inputs);
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
            }
        }
    }
    let part1 = signal_count[0]*signal_count[1];

    // part 2
    let mut modules = init_modules(&module_specs, &inputs);
    let src_name = {
        let src_names = &inputs["rx"]; //rx is not defined as a module
        assert_eq!(src_names.len(), 1);
        src_names[0]
    };
    {
        let (_, src_state) = &modules[src_name];
        assert!(match src_state {ModuleState::Conjunction(_) => true, _=>false});
    }
    let ancestor_inputs: BTreeMap<&str, BTreeSet<&str>> = {
        let rx_input_names: HashSet<&str> = inputs[src_name].iter().cloned().collect();
        println!("rx <- {} -< {:?}", src_name, rx_input_names);
        println!("Input ancestors");
    
        rx_input_names.iter().map(|&r| {
        let mut anc = BTreeSet::new();
        let mut work = vec![r];
        while let Some(w) = work.pop() {
            if w=="broadcaster" {continue}
            if anc.insert(w) {
                work.extend(inputs[w].iter());
            }
        }
        println!("{} ==> {:?}", r, anc);
        (r, anc)
    }).collect()};
    // are the ancestors distinct?
    if ancestor_inputs.values().combinations(2)
        .all(|v| v[0].intersection(v[1]).count()==0) {
        println!("Ancestors of rx input conjunction are distinct");
    } else {
        panic!("Ancestors of rx input conjunction are not distinct");
    };

    let mut signals: VecDeque<(&str, bool, &str)> = VecDeque::new(); //src, signal, dst
    // let mut push_period: HashMap<&str, usize> = HashMap::new();
    let mut loop_detect: Vec<HashMap<Vec<ModuleState>, usize>> 
        = ancestor_inputs.keys().map(|_| HashMap::new()).collect();
    let mut loop_result: BTreeMap<&str, [usize;2]> = BTreeMap::new();

    let mut push_count = 0usize;
    'outer: loop {
        for (lp, (anc, desc)) in loop_detect.iter_mut().zip(ancestor_inputs.iter()) {
            let key: Vec<ModuleState> = desc.iter().map(|d| &modules[d].1).cloned().collect();
            if let Some(previous_push_count) = lp.insert(key, push_count) {
                //println!("Loop detected {} -- {} for {}", previous_push_count, push_count, anc);
                loop_result.entry(anc).or_insert([previous_push_count, push_count]);
            }
        }
        if loop_result.len() == loop_detect.len() {break 'outer};

        signals.push_back(("in", false, "broadcaster"));
        push_count += 1;
        while let Some((src, s, dst)) = signals.pop_front() {
            if dst==src_name && s{
                println!("Signal@{:4} {} {}->{}", push_count, s, src, dst);
            }
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
            };    
        }


        // for (n, grp) in ancestor_inputs.iter() {
        //     let n_uninit = grp.iter().filter(|&a| !is_init(&modules[a].1)).count();
        //     if n_uninit == 0 {
        //         push_period.entry(n).or_insert(push_count);
        //         println!("Found period for {} at {}", n, push_count);
        //     }
        // }
        // if push_period.len()==ancestor_inputs.len() {break 'outer};
    }

    println!("Loops:\n{:?}", loop_result);

    // output is in last state before repeat
    // input state push count
    // 0 -- 1 -- 2 -...- 3733 --signal--3734(==1)
    
    let periods: Vec<_> = loop_result.values().map(|v| v[1]-v[0]).collect();
    let part2 = periods.into_iter().reduce(|a, b| a.lcm(&b)).unwrap();

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
