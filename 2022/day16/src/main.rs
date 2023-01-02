#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use std::{fs, time::Instant, collections::{HashMap, VecDeque, HashSet}, sync::Arc};
use nom::{IResult, bytes::complete::tag};
use itertools::Itertools;
//Valve ES has flow rate=0; tunnels lead to valves KE, JT

type Valve = [u8;2];

struct Input {
    valve: Valve,
    capacity: usize,
    connections: Vec<Valve>,
}

fn parse_valve(s: &str) -> IResult<&str, Valve> {
    let (rest, v) = nom::character::complete::alpha1(s)?;
    Ok((rest, [v.as_bytes()[0], v.as_bytes()[1]])) 
}

fn parse_line(s: &str) -> IResult<&str, Input> {
    let (rest, (_, valve, _, capacity, _, connections)) = nom::sequence::tuple((
        tag("Valve "),
        parse_valve, 
        tag(" has flow rate="),
        nom::character::complete::u64,
        nom::branch::alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        nom::multi::separated_list0(tag(", "), parse_valve)
    ))(s)?;
    Ok((rest, Input{valve, capacity: capacity as usize, connections}))
}

struct BBState {
    active: HashSet<Valve>,
    vented: usize,
    valve: Valve,
    time: u16,
}

impl BBState {
    fn new() -> Self {
        Self{active: HashSet::new(), vented: 0, valve: *b"AA", time: 30}
    }
}

type Distances = HashMap<Valve, Vec<u16>>;
type Resources = Vec<(u16, Valve)>;

struct BBContext {
    distances: Distances,
    resources: Resources, // sorted by capacity
}

fn branch_and_bound(from: BBState, context: &BBContext, best: &mut usize) {
    // check bound: assume all distances are 1, start with largest capacity
    let bound: usize = from.vented + context.resources.iter().rev()
    .filter(|(_, k)| !from.active.contains(k))
    .enumerate()
    .filter_map(|(i, (capacity, _valve))| 
        from.time.checked_sub(2*i as u16)
            .and_then(|tp| Some(tp as usize * *capacity as usize)))
    .sum::<usize>();
    if bound <= *best {return}

    // branch
    *best = (*best).max(from.vented);
    let branch_values: Vec<(usize, u16, Valve)> = context.distances[&from.valve].iter()
        .zip(context.resources.iter())
        .filter(|(_, (_, k))| !from.active.contains(k))
        .filter_map(|(dist, (capacity, valve))|
            from.time.checked_sub(dist+1)
            .and_then(|prod_time| 
                Some((prod_time as usize * *capacity as usize, prod_time as u16, *valve))
            )
        ).sorted().rev().collect();

    for (dv, time, valve) in branch_values {
        let state = {
            let mut active = from.active.clone();
            active.insert(valve);
            let vented = from.vented + dv;
            BBState{valve, vented, active, time}
        };
        branch_and_bound(state, context, best);
    }
} 

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<Input> = input_s.trim_end()
        .split("\n")
        .map(|s| parse_line(s).unwrap().1)
        .collect();

    // build distance map between resources
    let valves: HashMap<Valve, &Input> = input.iter().map(|i| (i.valve, i)).collect();
    let resources: Resources = input.iter()
        .filter(|i| i.valve==*b"AA" || i.capacity>0)
        .map(|i| (i.capacity as u16, i.valve))
        .sorted().collect();
    let mut distances: Distances = HashMap::new();
    for (_, a) in resources.iter() {
        // BFS from a
        let mut frontier: VecDeque<(Valve, u16)> = VecDeque::new();
        let mut visited: HashMap<Valve, u16> = HashMap::new();
        frontier.push_back((*a, 0));
        while let Some((n, d)) = frontier.pop_front() {
            for nb in valves.get(&n).unwrap().connections.iter() {
                visited.entry(*nb)
                .or_insert_with(|| {
                    frontier.push_back((*nb, d+1));
                    d+1
                });
            }
        }
        distances.insert(*a, resources.iter().map(|(_, b)| visited[b]).collect());
    }
    let bb_context = BBContext{distances, resources};

    let mut part1: usize = 0;
    branch_and_bound(BBState::new(), &bb_context, &mut part1);
    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "1651");
    assert_eq!(res[1], "0");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    for _ in 0..20 {solution(&input)?;} //warmup
    let start = Instant::now();
    let res = solution(&input)?;
    println!(
        "({} us)\nPart 1: {}\nPart 2: {}",
        start.elapsed().as_micros(), res[0], res[1],
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
//         assert!(res[0] == "0");
//         assert!(res[1] == "0");
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
