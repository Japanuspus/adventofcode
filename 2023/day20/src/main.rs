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
    let module = nm::separated_pair(modspec, nm::ws(nm::tag("->")), receivers);
    nm::separated_list1(nm::newline, module)(s)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ModuleState<'a> {
    Conjunction(BTreeMap<&'a str, bool>),
    FlipFlop(bool),
    Broadcast,
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

    // inputs for each module
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
            if s {signal_count[0]+=1} else {signal_count[1]+=1};
            if let Some((rcvs, state)) = modules.get_mut(dst) {
            if let Some(output) = { // this should be a function, but borrow checker...
                    match state {
                        ModuleState::Broadcast => Some(s),
                        ModuleState::FlipFlop(v) => if !s {*v = !*v; Some(*v)} else {None},
                        ModuleState::Conjunction(mem) => {
                            mem.insert(src, s);
                            Some(!mem.values().all(|m| *m))
                        }   
                    }
                } {
                    for r in rcvs.iter() {
                        signals.push_back((dst, output, r));
                    }    
                }
            }
        }
    }
    let part1 = signal_count[0]*signal_count[1];

    // part 2
    // find the single module that sends to "rx"
    let mut modules = init_modules(&module_specs, &inputs);
    let src_name = {
        let src_names = &inputs["rx"]; //rx is not defined as a module
        assert_eq!(src_names.len(), 1);
        src_names[0]
    };
    // ... check that it is a conjunction
    {
        let (_, src_state) = &modules[src_name];
        assert!(match src_state {ModuleState::Conjunction(_) => true, _=>false});
    }
    // Find ancestors of inputs to <src_name>
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
    // ... and check that they are extinct.
    if ancestor_inputs.values().combinations(2)
        .all(|v| v[0].intersection(v[1]).count()==0) {
        println!("Ancestors of rx input conjunction are distinct");
    } else {
        panic!("Ancestors of rx input conjunction are not distinct");
    };

    // Look for state returns in each of the ancestries
    let mut signals: VecDeque<(&str, bool, &str)> = VecDeque::new(); //src, signal, dst
    let mut loop_detect: Vec<HashMap<Vec<ModuleState>, usize>> 
        = ancestor_inputs.keys().map(|_| HashMap::new()).collect();
    let mut loop_result: BTreeMap<&str, [usize;2]> = BTreeMap::new();

    let mut push_count = 0usize;
    'outer: loop {
        for (lp, (anc, desc)) in loop_detect.iter_mut().zip(ancestor_inputs.iter()) {
            let key: Vec<ModuleState> = desc.iter().map(|d| &modules[d].1).cloned().collect();
            if let Some(previous_push_count) = lp.insert(key, push_count) {
                loop_result.entry(anc).or_insert([previous_push_count, push_count]);
            }
        }
        if loop_result.len() == loop_detect.len() {break 'outer};

        signals.push_back(("in", false, "broadcaster"));
        push_count += 1;
        while let Some((src, s, dst)) = signals.pop_front() {
            if dst==src_name && s{
                println!("Signal@push#{:4} {} {}->{}", push_count, s, src, dst);
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
                } {
                    for r in rcvs.iter() {
                        signals.push_back((dst, output, r));
                    }    
                }
            };    
        }
    }
    println!("Loops:\n{:?}", loop_result);

    let periods: Vec<_> = loop_result.values().map(|v| v[1]-v[0]).collect();
    let part2 = periods.into_iter().reduce(|a, b| a.lcm(&b)).unwrap();

    Ok([part1.to_string(), part2.to_string()])
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

