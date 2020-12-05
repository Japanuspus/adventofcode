#![allow(unused)]

use std::collections::{HashMap, VecDeque};
use day11::State;

type Pos = (isize, isize);

fn dp(p: &Pos, d: isize) -> Pos {
    match d {
        1 => (p.0, p.1-1),
        2 => (p.0, p.1+1),
        3 => (p.0-1, p.1),
        4 => (p.0+1, p.1),
        _ => {panic!("bad direction")}
    }
}

fn opp(d: isize) -> isize {
    match d {
        1 => 2,
        2 => 1, 
        3 => 4, 
        4 => 3, 
        _ => {panic!("bad direction")}
    }
}

fn fill_distance(map: &HashMap<Pos, isize>, origin: Pos) -> HashMap<Pos, isize> {
    // min distance floodfill
    let mut dists = HashMap::new();
    dists.insert(origin, 0);
    let mut work = VecDeque::new();
    work.push_back(origin);
    while let Some(p0) = work.pop_front() {
        let pts: Vec<_> = (1..5)
            .map(|d| dp(&p0, d))
            .filter(|p1|  dists.get(p1).is_none() && *map.get(p1).unwrap_or(&0)>0)
            .collect();
        let d1 = 1+dists.get(&p0).unwrap();
        for p in pts {
            dists.insert(p, d1);
            work.push_back(p);
        }
    }
    dists
}

// N1, S2, W3, E4
fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");
    let mut s = State::from(&input);


    // Depth first exploration 
    let mut trace = Vec::new(); // for backtrace
    let mut map = HashMap::new(); // 0: wall, 1-> ok, 2 -> oxygen
    let mut oxygen = None;

    let mut pos = (0, 0);
    map.insert(pos.clone(), 1);
    while let Some(dir) = (1..5)
            .filter(|d| map.get(&dp(&pos, *d)).is_none())
            .next().or_else(|| trace.pop()) 
    {
        let next_pos = dp(&pos, dir);
        let res = s.next_numbers(1, || Some(dir)).unwrap().unwrap()[0];
        let was_new = map.insert(next_pos, res).is_none();
        if res>0 {
            if res>1 {
                println!("Found oxygen");
                oxygen = Some(next_pos.clone());
            }
            if was_new {
                trace.push(opp(dir)
            )};
            pos = next_pos;
        };
    }
    let map = map;
    let oxygen = oxygen.unwrap();

    // min distance floodfill
    let dists = fill_distance(&map, (0,0));
    println!("Part 1: {}", dists.get(&oxygen).unwrap());

    // Part 2
    let ofill = fill_distance(&map, oxygen.clone());
    println!("Part 2: {}",
        ofill.values().max().unwrap());
}