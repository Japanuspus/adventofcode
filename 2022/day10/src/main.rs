#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use itertools::Itertools;
use std::{fs, time::Instant, iter};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display(style="lowercase")]
enum Op {
    Noop,
    #[display("{} {0}")]
    Addx(i32),
}

struct CPU {
    ip: usize,
    x: isize,
    add_pause: bool,
}

impl CPU {
    fn new() -> Self {
        CPU{ip:0, x:1, add_pause:false}
    }
    fn step(&mut self, op: &Op) {
        self.ip+=match (self.add_pause, op) {
            (false, Op::Noop) => {1},
            (false, Op::Addx(_)) => {self.add_pause=true; 0},
            (true, Op::Addx(v)) =>{self.add_pause=false; self.x+=*v as isize; 1},
            _ => {panic!();},
        };
    }
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<Op> = input_s.trim_end()
        .split("\n")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_, _>>()?;

    let mut cpu = CPU::new();
    let mut cc = 0usize;
    let mut signal = 0isize;
    while let Some(op) = input.get(cpu.ip) {
        cc+=1;
        if let Some(cc20) = cc.checked_sub(20) {
            if cc20%40==0 {signal+=(cc as isize)*cpu.x;}
        }
        cpu.step(op);
    };
    let part1 = signal;

    let mut cpu = CPU::new();
    let crt: Vec<char> = (0..240isize).map(|cc| {
        // let cycle = cc+1
        let crt_x = cc%40;
        let pixel = if (crt_x-cpu.x).abs()<2 {'#'} else {'.'};
        cpu.step(&input[cpu.ip]);
        pixel
    }).collect();
    let part2: String = crt.chunks(40).map(|ch| ch.iter().collect::<String>()).join("\n");
    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test01.txt")?;
    let res2 = &fs::read_to_string("test04.txt")?;
    let res = solution(&input).with_context(|| format!("Running solution"))?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "13140");
    assert!(res[1] == res2.trim());
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    for _ in 0..20 {
        solution(&input)?;
    } //warmup
    let start = Instant::now();
    let res = solution(&input)?;
    println!(
        "({} us)\nPart 1: {}\nPart 2: \n{}",
        start.elapsed().as_micros(), res[0], res[1],
    );
    Ok(())
}
