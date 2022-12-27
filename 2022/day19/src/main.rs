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
    Ore,
    Clay,
    Obsidian,
    Geode,
}
impl Resource {
    const VALUES: [Self;4] = [Self::Ore, Self::Clay, Self::Obsidian, Self::Geode];
    const fn index(&self) -> usize {
        match self {
            Self::Ore => 3,
            Self::Clay => 2,
            Self::Obsidian => 1,
            Self::Geode => 0,    
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

struct RobotPrice {
    product: Resource,
    price: Vec<(u32, Resource)>
}

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

// compute a bitvec indicating robot purchases that are not reachable as next purchase
fn unreachable(robots: ResVec<u8>, resources: ResVec<u8>, time: u8, costs: &CostMap) -> u8 {
    let final_resources = vec4_add(resources, vec4_scale(robots, time));
    let final_possible = Resource::VALUES.iter().filter_map(|r| 
        vec4_checked_sub(final_resources, costs[r.index()]).and_then(|_| Some(r.bitflag()))
    ).reduce(|a,b| a|b).unwrap_or(0);
    Resource::ALL & !final_possible
}

fn value_vector(costs: &CostMap) -> [usize;4] {
    
}


#[derive(Debug, Clone)]
struct State {
    // Remaining time (at beginning of run)
    time: u8,
    // "unreachable" indicates that with current ressoureces/robots, we cannot wait 
    // long enough to be in a position to make this purchase.
    disallowed: u8, // unreachable or skipped 
    robots: ResVec<u8>,
    resources: ResVec<u8>,
}

impl State {
    fn new(costs: &CostMap) -> Self {
        let mut robots = [0;4];
        robots[Resource::Ore] = 1;
        let resources = [0;4];
        let time = 24;
        Self{time, robots, resources, disallowed: unreachable(robots, resources, time, costs)}
    }
}


fn vec4_checked_sub(a: ResVec<u8>, b: ResVec<u8>) -> Option<ResVec<u8>> {
    if a.iter().zip(b.iter()).all(|(av, bv)| av>=bv) {
        Some(vec4_sub(a, b))
    } else {
        None
    }
}

fn process_blueprint(bp: &Blueprint) -> usize {
    let costs = bp_cost_matrix(bp);
    let mut max_geode: u8 = 0;
    let mut work = Vec::<State>::new();
    work.push(State::new(&costs));
    while let Some(w) = work.pop() {
        if w.time == 0 {
            let o=w.resources[Resource::Geode];
            if o>0 {
                println!("BP# {:02} - Found {:3} geodes, Backlog: {}", bp.id, &o, work.len());
                if o>max_geode {max_geode=o};
            }
            continue;
        }
        let purchase_resources: [(Resource, Option<ResVec<u8>>);4] = Resource::VALUES.map(|r| (
            r, 
            vec4_checked_sub(w.resources, costs[r]).and_then(|res| Some(vec4_add(res, w.robots)))
        ));
        let possible_now = purchase_resources.iter().filter_map(|(r, v)| v.and_then(|_| Some(r.bitflag()))).reduce(|a, b| a|b).unwrap_or(0);
        let time = w.time-1;
        // Maybe wait
        {
            let disallowed = w.disallowed | possible_now; //If we wait, anything that was possible to buy now should not be bought later
            if disallowed != Resource::ALL {
                // There are things we could wait for
                work.push(State{robots: w.robots, resources: vec4_add(w.resources, w.robots), time, disallowed});
            }    
        }

        // Possible purchases
        for (r, o_res) in purchase_resources.into_iter() {
            if let Some(resources) = o_res {
                let not_allowed = (r.bitflag()&w.disallowed)>0;
                if not_allowed {continue}
                let robots = vec4_add(w.robots, r.unit());
                let disallowed = unreachable(robots, resources, time, &costs);
                work.push(State{robots, resources, disallowed, time})                
            }
        }
    };
    max_geode as usize
}

#[test]
fn test_blueprint() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let bp = parse_blueprint(input.split("\n").next().unwrap()).unwrap().1;
    let res = process_blueprint(&bp);
    assert!(res == 12);
    Ok(())
}


fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<Blueprint> = input_s.trim_end().split("\n")
        .map(|s| parse_blueprint(s).unwrap().1)
        .collect(); 

    let part1 = input.iter().map(|bp| process_blueprint(bp)*bp.id as usize).sum::<usize>();
    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "33");
    assert!(res[1] == "0");
    Ok(())
}

fn main() -> Result<()> {
    //let input = &fs::read_to_string("input.txt")?;
    let input = &fs::read_to_string("test00.txt")?;
    //for _ in 0..20 {solution(&input)?;} //warmup
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
