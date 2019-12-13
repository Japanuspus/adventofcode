#![allow(unused)]

use std::collections::HashMap;
use num::Integer; //dependency: num = "0.2"

type V3 = Vec<isize>;

fn abssum(v: &V3) -> isize {
    v.iter().map(|x| x.abs()).sum()
}

fn add(a: &V3, b: &V3) -> V3 {
    a.iter().zip(b.iter()).map(|(a,b)| a+b).collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Moon {
    r: V3,
    v: V3
}

impl Moon {
    fn etot(&self) -> isize {
        abssum(&self.r)*abssum(&self.v)
    }
}

// <x=-1, y=-4, z=0>
fn parse_moon(s: &str) -> Moon {
    let p:Vec<_> = s.split(|c| c==',' || c=='=' || c=='>').collect();
    let mut xyz = Vec::new();
    xyz.push(p[1]);
    xyz.push(p[3]);
    xyz.push(p[5]);
    return Moon{r: xyz.iter().map(|n| n.parse().unwrap()).collect(), v: vec![0,0,0]}
}


// Part 1
fn acc1(d: &Moon, s: &Moon) -> V3 {
    d.r.iter()
    .zip(s.r.iter())
    .map(|(a,b)| if a>b {-1} else {if a<b {1} else {0}})
    .collect()
}

fn acc(m: &Moon, src: &Vec<Moon>) -> V3 {
    src.iter()
    .map(|s| acc1(m,s))
    .fold(vec![0,0,0], |s, da| add(&s, &da))
}

fn next_pos(m: &Moon, acc: &V3) -> Moon {
    let v = add(&m.v, acc);
    let r = add(&m.r, &v);
    Moon{v, r}
}

fn step(ms: &Vec<Moon>) -> Vec<Moon> {
    ms.iter()
    .map(|m| next_pos(m, &acc(m, ms)))
    .collect()
}

// Part 2: use V3 for all x coordinates, x velocities etc
fn acc_t(c0: isize, cs: &Vec<isize>) -> isize {
    cs.iter().map(|c1| if *c1>c0 {1} else {if *c1<c0 {-1} else {0}}).sum()
}

fn step_t(cs: &Moon) -> Moon {
    //let acc = cs.r.iter().map(move |c0| cs.r.iter().map(|c1| if c1>c0 {1} else {if c1<c0 {-1} else {0}}).sum());
    let v:Vec<isize> = cs.r
        .iter()
        .zip(cs.v.iter())
        .map(|(c0, v0)| *v0 + acc_t(*c0, &cs.r))
        .collect();
    let r = cs.r.iter().zip(v.iter()).map(|(r, dr)| r+dr).collect();
    Moon{r, v}
}

fn find_cycle(cs0: &Moon) -> (usize, usize) {
    let mut cs = cs0.clone();
    let mut iters = HashMap::new();
    for iter in (0..) {
        if let Some(k) = iters.get(&cs) {
            return (*k, iter);
        } else {
            let next_cs = step_t(&cs);
            iters.insert(cs, iter);
            cs = next_cs;
        }
    };
    panic!("No cycle")
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
    .expect("Error reading input file");

    let moons: Vec<_> = input.lines().map(|s| parse_moon(s)).collect();
    let ms1000 = (0..1000).fold(moons.clone(), |ms,_| step(&ms));
    let energy:isize = ms1000.iter()
        .map(|m| m.etot())
        .sum();
    println!("Part 1: {}", energy);

    // part 2
    let css: Vec<_> = (0..3).map(|q| Moon{r: moons.iter().map(|m| m.r[q]).collect(), v: moons.iter().map(|m| m.v[q]).collect()}).collect();
    let ret: Vec<_> = css.iter().map(find_cycle).collect();
    dbg!(&ret);
    let cycles: Vec<usize> = ret.iter().map(|cc| cc.1).collect();
    dbg!(&cycles);
    println!("Part 2: {}", cycles.iter().fold(1, |cycle, c| cycle.lcm(c)));
} 