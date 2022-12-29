#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use nom::sequence::separated_pair;
use vecmath::{vec4_add, vec4_sub, vec4_scale};
use std::collections::{VecDeque, HashSet, BTreeSet};
use std::{fs, time::Instant};
use parse_display::{Display, FromStr};
use nom;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::{Finish, IResult};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display(style = "lowercase")]
enum Resource {
    Geode,
    Obsidian,
    Clay,
    Ore,
}

impl Resource {
    const VALUES: [Self;4] = [Self::Geode, Self::Obsidian, Self::Clay, Self::Ore,];
    const fn index(&self) -> usize {
        match self {
            Self::Geode => 0,    
            Self::Obsidian => 1,
            Self::Clay => 2,
            Self::Ore => 3,
        }
    }
    const ALL: u8 = 0b1111;
    const fn bitflag(&self) -> u8 {
        1<<self.index()
    }
    const fn unit(&self) -> [u8;4] {
        let mut v = [0;4];
        v[self.index()] = 1;
        v
    }
}

#[derive(Debug)]
struct RobotPrice {
    product: Resource,
    price: Vec<(u32, Resource)>
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    recipes: Vec<RobotPrice>,
}

fn parse_resource(s: &str) -> IResult<&str, Resource> {
    nom::combinator::map(nom::character::complete::alpha1, |v: &str| {v.parse::<Resource>().unwrap()})(s)
}

//Each geode robot costs 2 ore and 7 obsidian.
fn parse_line(s: &str) -> IResult<&str, RobotPrice> {
    let (rest, (_, product, _, price)) = nom::sequence::tuple((
        tag("Each "),
        parse_resource,
        tag(" robot costs "),
        nom::multi::separated_list1(tag(" and "), separated_pair(nom::character::complete::u32, char(' '), parse_resource)),
    ))(s)?;
    Ok((rest,RobotPrice{product, price}))
}

fn parse_blueprint(s: &str) -> IResult<&str, Blueprint> {
    let (rest, (_, id, _, recipes,_)) = nom::sequence::tuple((
        tag("Blueprint "),
        nom::character::complete::u32,
        tag(": "),
        nom::multi::separated_list1(tag(". "), parse_line),
        tag(".")
    ))(s)?;
    Ok((rest, Blueprint{id, recipes}))
}

type ResVec<T> = [T;4];

impl <T> std::ops::Index<Resource> for ResVec<T> {
    type Output = T;
    fn index(&self, r: Resource) -> &Self::Output { &self[r.index()] }
}
impl <T> std::ops::IndexMut<Resource> for ResVec<T> {
    fn index_mut(&mut self, r: Resource) -> &mut Self::Output { &mut self[r.index()] }
}

/// CostMap[Ore] is a ResVec indicating the cost of an Ore robot
type CostMap = ResVec<ResVec<u8>>;

fn bp_cost_matrix(bp: &Blueprint) -> CostMap {
    let mut m = [[0u8;4];4];
    for r in &bp.recipes {
        let vr = &mut m[r.product];
        for (amount, item) in &r.price {
            vr[*item] = *amount as u8;
        }
    }
    m
} 

#[derive(Debug, Clone, PartialEq)]
struct State {
    time: u8, //remaining
    robots: ResVec<u8>,
    resources: ResVec<u8>,
}

fn vec4_checked_sub(a: ResVec<u8>, b: ResVec<u8>) -> Option<ResVec<u8>> {
    if a.iter().zip(b.iter()).all(|(av, bv)| av>=bv) {
        Some(vec4_sub(a, b))
    } else {
        None
    }
}

impl State {
    fn new(time: u8) -> Self {
        Self{time, robots: Resource::Ore.unit(), resources: [0;4]}
    }

    fn wait_for_robot(&self, robot: Resource, costs: &CostMap) -> Option<Self> {
        (0..self.time).rev().find_map(|time| 
            vec4_checked_sub(
                // resources are consumed 1 time unit before robot is completet
                vec4_add(self.resources, vec4_scale(self.robots, self.time-(time+1))), 
                costs[robot]
            ).and_then(|resources|
                Some(Self{time, resources: vec4_add(resources, self.robots), robots: vec4_add(self.robots, robot.unit())})
            ))
    }

    /// The trick: next state is not next time step, but next time a purchase is made
    fn branch<'a>(&'a self, costs: &'a CostMap) -> impl Iterator<Item=Self> + 'a {
        Resource::VALUES.iter().filter_map(|robot| self.wait_for_robot(*robot, costs)) 
    }

    fn secured_geodes(&self) -> u8 {
        self.resources[Resource::Geode] + self.time*self.robots[Resource::Geode]
    }

    // an upper bound on number of geodes we can reach from here
    fn bound(&self, costs: &CostMap) -> u8 {
        // propagate number of geodes to t=0, assuming infinite ore and clay resources
        // g, o, gr, or
        let gr_price = costs[Resource::Geode][Resource::Obsidian];
        (0..self.time).fold((
            self.resources[Resource::Geode], 
            self.resources[Resource::Obsidian],
            self.robots[Resource::Geode],
            self.robots[Resource::Obsidian],
        ), |(g, o, gr, or), _| 
            if let Some(o2) = o.checked_sub(gr_price) {
                (g+gr, o2+or, gr+1, or)
            } else {
                (g+gr, o+or, gr, or+1)
            }
        ).0
    }
}

fn branch_and_bound(state: State, costs: &CostMap, best: &mut u8) {
    if false {
        let secured_geodes = state.secured_geodes();
        let bound = state.bound(costs);
        assert!(bound >= secured_geodes);
        println!("G: {:2} (bd: {:2}) <-- {:?}", secured_geodes, bound, &state);
    };
    *best = state.secured_geodes().max(*best);
    for s in state.branch(costs) {
        if s.bound(costs) > *best {
            branch_and_bound(s, costs, best);
        }
    }
}

fn process_blueprint(bp: &Blueprint, time: u8) -> u8 {
    let costs = bp_cost_matrix(bp);
    let mut best = 0;
    branch_and_bound(State::new(time), &costs, &mut best);
    best
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<Blueprint> = input_s.trim_end().split("\n")
        .map(|s| parse_blueprint(s).unwrap().1)
        .collect(); 

    let part1 = input.iter().map(|bp| process_blueprint(bp, 24) as usize*bp.id as usize).sum::<usize>();
    let part2 = input.iter().map(|bp| process_blueprint(bp, 32) as usize).take(3).product::<usize>();

    Ok([part1.to_string(), part2.to_string()])
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

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "33");
    assert_eq!(res[1], "3472");
    Ok(())
}

#[test]
fn test_blueprint() -> Result<()> {
    let bps: Vec<_> = fs::read_to_string("test00.txt")?.trim_end().split("\n").map(|ln| parse_blueprint(ln).unwrap().1).collect();
    for (bp, exp) in bps.iter().zip([9, 12].iter()) {
        println!("\n\n ***** Blueprint: {:?}", bp);
        let res = process_blueprint(bp, 24);
        assert_eq!(res, *exp);
    }
    Ok(())
}
#[test]
fn test_resource_index_consistency() {
    assert!(Resource::VALUES.iter().map(|r| r.index()).enumerate().all(|(i, ri)| i == ri));
}

fn fixture_bp2() -> CostMap {
    let s = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    let bp = parse_blueprint(s).unwrap().1;
    bp_cost_matrix(&bp)
}

#[test]
fn test_costs() {
    let costs = fixture_bp2();
    assert_eq!(costs[Resource::Obsidian][Resource::Clay], 8);
}

#[test]
fn test_wait_for_robot() {
    let costs = fixture_bp2();
    let s = State::new(24);
    assert_eq!(None, s.wait_for_robot(Resource::Geode, &costs));
    assert_eq!(None, s.wait_for_robot(Resource::Obsidian, &costs));

    
    let mut s_c = s.clone();
    s_c.time = 20;
    s_c.robots[Resource::Clay] = 1;
    s_c.resources[Resource::Ore] = 1;
    assert_eq!(Some(s_c), s.wait_for_robot(Resource::Clay, &costs));
    

    let mut s_o = s.clone();
    s_o.time = 21;
    s_o.robots[Resource::Ore] = 2;
    s_o.resources[Resource::Ore] = 1;
    assert_eq!(Some(s_o.clone()), s.wait_for_robot(Resource::Ore, &costs));

    let mut s_oc = s_o.clone();
    s_oc.time = 19;
    s_oc.robots[Resource::Clay] = 1;
    s_oc.resources[Resource::Ore] = 2;
    assert_eq!(Some(s_oc.clone()), s_o.wait_for_robot(Resource::Clay, &costs));

    let mut s_oco = s_oc.clone();
    s_oco.time = 18;
    s_oco.robots[Resource::Ore] = 3;
    s_oco.resources[Resource::Ore] = 2;
    s_oco.resources[Resource::Clay] = 1;
    assert_eq!(Some(s_oco.clone()), s_oc.wait_for_robot(Resource::Ore, &costs));
}

#[test]
fn test_bound() {
    let costs = fixture_bp2();
    let mut state = {
        let mut resources = [0;4];
        resources[Resource::Obsidian] = 11;
        let mut robots = [0;4];
        robots[Resource::Obsidian] = 1;
        State{time: 1, resources, robots}
    };
    // At end of minute
    // 01: Res: 12 00 Rob: 02 00
    // 02: Res: 02 00 Rob: 02 01
    // 03: Res: 04 01 Rob: 03 01
    // 04: Res: 07 02 Rob: 04 01
    // 05: Res: 11 03 Rob: 05 01
    // 06: Res: 16 04 Rob: 06 01
    // 07: Res: 10 05 Rob: 06 02

    assert_eq!(state.bound(&costs), 0);

    state.time = 2;
    assert_eq!(state.bound(&costs), 0);

    state.time = 3;
    assert_eq!(state.bound(&costs), 1);

    state.time = 6;
    assert_eq!(state.bound(&costs), 4);

    state.time = 7;
    assert_eq!(state.bound(&costs), 5);
}

