#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use itertools::Itertools;
use std::{fs, iter::Cycle, collections::HashMap};

fn parse(input_s: &str) -> Result<Vec<u32>> {
    input_s
    .trim()
    .split("\n")
    .map(|s| {let (_, b) = s.split_once(": ").unwrap(); b.parse().with_context(|| format!("Parsing {}", s))})
    .collect::<Result<_>>()
}

fn part1(input: &Vec<u32>) {
    let mut die = (1..=100u32).cycle();
    let mut die_count: usize = 0;
    let mut pos_minus_1: [u32; 2] = [input[0]-1, input[1]-1];
    let mut scores: [usize; 2] = [0;2];
    'outer: loop {
        for i in 0..2 {
            let roll = die.by_ref().take(3).sum::<u32>();
            die_count+=3;
            let p = (pos_minus_1[i]+roll) % 10;
            pos_minus_1[i] = p;
            scores[i]+=(p+1) as usize;
            if scores[i]>=1000 {break 'outer}
            //println!("Player {} roll {} new_pos {} score{}", i, &roll, pos_minus_1[i]+1, scores[i]);
        }
    }
    println!("Part 1: {}", die_count*scores.iter().min().unwrap());
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
struct GameState {
    pos_minus_1: [u8;2],
    score: [u8;2],
}

impl GameState {
    fn play(&self, i: usize, roll: u8) -> Self {
        let mut s = self.clone();
        let pm1 = (s.pos_minus_1[i]+roll) % 10;
        s.pos_minus_1[i] = pm1;
        s.score[i] += pm1+1;
        s
    }
}

type Counts = HashMap<GameState, usize>;

fn part2(input: &Vec<u32>) {
    let mut roll_count: [usize;10] = [0usize;10];
    for roll in (0..3).map(|_| (1usize..=3usize)).multi_cartesian_product().map(|v| v.iter().sum::<usize>()) {
        roll_count[roll]+=1;
    }

    let mut states: Counts = HashMap::new();
    states.insert(GameState{
        pos_minus_1: [input[0] as u8 -1, input[1] as u8 -1],
        score: [0;2],
    }, 1);

    let mut wins = [0usize;2];
    while states.len() > 0 {
        for i in 0..2 {
            let mut states_next: Counts = HashMap::new();
            for (s0, ct) in states.into_iter() {
                for roll in 3..=9 {
                    let n = ct * roll_count[roll as usize];
                    let s1 = s0.play(i, roll);
                    if s1.score[i] >= 21 {
                        wins[i]+=n;
                    } else {
                        *states_next.entry(s1).or_default() += n;
                    }
                }
            }
            states = states_next;
        }
    }
    //dbg!(wins);
    println!("Part 2: {}", wins.iter().max().unwrap());
}

fn solve(n: &str) -> Result<()> {
    println!("** FILE: {} **", n);
    let input = parse(&fs::read_to_string("test00.txt")?)?;
    part1(&input);
    part2(&input);
    Ok(())
}

fn main() -> Result<()> {
    solve("test00.txt")?; 
    solve("input.txt")?; 
    Ok(())
}
