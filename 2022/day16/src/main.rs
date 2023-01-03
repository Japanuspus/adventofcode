#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use std::{fs, time::Instant, collections::{HashMap, VecDeque, HashSet}, sync::Arc};
use nom::{IResult, bytes::complete::tag};
use itertools::Itertools;
//Valve ES has flow rate=0; tunnels lead to valves KE, JT

type Valve = [u8;2];

struct Input {
    valve: Valve,
    capacity: u16,
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
    Ok((rest, Input{valve, capacity: capacity as u16, connections}))
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Resource {
    capacity: u16,  // primary sort value 
    valve: Valve,
    distances: Vec<u16>,
}
type Resources = Vec<Resource>;

fn parse_resources(input_s: &str) -> Result<Resources> {
    // sorted by increasing capacity. AA is included as first entry
    let input: Vec<Input> = input_s.trim_end()
        .split("\n")
        .map(|s| parse_line(s).unwrap().1)
        .collect();

    // build distance map between resources
    let valves: HashMap<Valve, &Input> = input.iter().map(|i| (i.valve, i)).collect();
    let r_valves: Vec<(u16, Valve)> = input.iter()
        .filter(|i| i.valve==*b"AA" || i.capacity>0)
        .map(|i| (i.capacity, i.valve))
        .sorted().collect();

    Ok(r_valves.iter()
        .map(|(capacity, a)| {
            // BFS from a
            let mut frontier: VecDeque<(Valve, u16)> = VecDeque::new();
            let mut visited: HashMap<Valve, u16> = HashMap::new();
            frontier.push_back((*a, 0));
            while let Some((n, d)) = frontier.pop_front() {
                for nb in valves[&n].connections.iter() {
                    visited.entry(*nb)
                    .or_insert_with(|| {
                        frontier.push_back((*nb, d+1));
                        d+1
                    });
                }
            }
            Resource{valve: *a, capacity: *capacity, distances: r_valves.iter().map(|(_, b)| visited[b]).collect()}
        }).collect::<Vec<_>>())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Agent {
    valve: u16, // index of Valve, 
    time: u16, // Time remaining once this valve is activated
}

#[derive(Debug, Clone)]
struct BBState2<const N: usize> {
    vented: usize, // includes all future venting from valves targeted by agents
    active: u16,   // includes valves targeted by agents
    agents: [Agent;N],
}

struct ProductIndex<const N: usize> {
    bounds: [usize;N],
    current: [usize;N],
    is_empty: bool,
}
impl <const N: usize> ProductIndex<N> {
    fn new(bounds: [usize;N]) -> Self {
        Self{bounds, current: [0;N], is_empty: bounds.iter().any(|b| *b==0)}
    }
}

impl <const N: usize> Iterator for ProductIndex<N> {
    type Item = [usize;N];
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty {return None};
        let mut i=0;
        while self.current[i]==self.bounds[i] {
            if i+1 >= N {return None}
            self.current[i] = 0;
            self.current[i+1] += 1;
            i+=1;
        };
        let res = Some(self.current);
        self.current[0]+=1;
        res
    }
}

impl <const N: usize> BBState2<N> {
    fn new(time: u16) -> Self {
        Self{active: 1, vented: 0, agents: [Agent{valve: 0, time}; N]}
    }

    /// Assume each agent can open one valve for each two minutes once it has completed current tasks
    fn bound(&self, resources: &Resources) -> usize {
        let mut capacities = resources.iter().enumerate().rev()
            .filter_map(|(i, r)| if 1<<i & self.active == 0 {Some(r.capacity)} else {None});
        let t_max = self.agents.iter().map(|a| a.time).max().unwrap();
        // self.vented + (0..t_max).rev().flat_map(|t_prod| 
        //     self.agents.iter()
        //     .filter_map(|a|
        //         a.time.checked_sub(t_prod)
        //         .and_then(|t_free| if t_free>0 && t_free%2==0 {
        //             capacities.next().and_then(|capacity| Some(t_prod as usize * capacity as usize))
        //         } else {None})
        //     )
        // ).sum::<usize>()
        let mut s = self.vented;
        for t_prod in (0..t_max).rev() {
            for a in self.agents {
                if let Some(t_free) = a.time.checked_sub(t_prod) {
                    if t_free>0 && t_free % 2== 0 {
                        if let Some(capacity) = capacities.next() {
                            s+= capacity as usize * t_prod as usize;
                        }
                    }
                }
            }
        };
        s
    }

    fn branch(&self, resources: &Resources) -> impl Iterator<Item=Self> + '_{
        //actions for each agent,
        let t_max = self.agents.iter().map(|a| a.time).max().unwrap();
        let actions_by_agent: [Vec<(usize, Agent)>;N] = self.agents.map(|a|
            if a.time == t_max {
                resources.iter().enumerate().rev()
                .filter(|(ir, _)| 1<<ir & self.active == 0)
                .filter_map(|(ir, r)| 
                    a.time.checked_sub(1+r.distances[a.valve as usize])
                    .and_then(|time| Some((
                        time as usize * r.capacity as usize, 
                        Agent{time, valve: ir as u16}))))
                .collect()
            } else {
                vec![(0, a)]
            }
        );
        let action_bounds: [usize; N] = actions_by_agent.iter().map(|a| a.len()).collect::<Vec<usize>>().try_into().unwrap();
        ProductIndex::new(action_bounds)
        .map(move |iv| {
            let dv = actions_by_agent.iter().zip(iv.iter()).map(|(av, i)| av[*i].0).sum::<usize>();
            let mut actions = [Agent{valve: 0, time: 0};N];
            for i in 0..N {
                actions[i] = actions_by_agent[i][iv[i]].1;
            }
            (dv, actions)
        })
        // filter out actions with multiple agents sharing a target
        .filter(|(_, actions)| (actions.iter().fold(0u16, |acc, agent| acc | (1<<agent.valve)).count_ones() as usize) == N)
        // .sorted().rev() -- the allocation slow things down
        .map(|(dv, agents)| {
            let active = agents.iter().fold(self.active, |active, a| active | 1<<a.valve);
            Self{vented: self.vented+dv, active, agents}
        })
    }

    fn branch_and_bound(&self, resources: &Resources, best: &mut usize) {
        (*best) = (*best).max(self.vented);
        for n in self.branch(resources) {
            let bound = n.bound(resources);
            if bound>*best {
                n.branch_and_bound(resources, best)
            }
        }
    }
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let resources = parse_resources(input_s)?;

    let mut part1: usize = 0;
    BBState2::<1>::new(30).branch_and_bound(&resources, &mut part1);

    let mut part2: usize = 0;
    BBState2::<2>::new(26).branch_and_bound(&resources, &mut part2);

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_bstate() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let resources = parse_resources(input)?;
    let branches: Vec<_> = BBState2::<1>::new(30).branch(&resources).collect();
    assert_eq!(branches.len(), resources.len()-1);
    Ok(())
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "1651");
    assert_eq!(res[1], "1707");
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
