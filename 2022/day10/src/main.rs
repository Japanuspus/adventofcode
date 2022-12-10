#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use itertools::Itertools;
use std::{fs, time::Instant, iter};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
enum Op {
    #[display("noop")]
    Noop,
    #[display("addx {0}")]
    Addx(#[display("{}")] i32),
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<Op> = input_s.trim_end()
        .split("\n")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_, _>>()?;

    let mut ip = 0usize;
    let mut cc = 0usize;
    let mut x = 1isize;
    let mut add_pause = false;
    let mut signal = 0isize;
    while let Some(op) = input.get(ip) {
        cc+=1;
        if let Some(cc20) = cc.checked_sub(20) {
            if cc20%40==0 {signal+=(cc as isize)*x;}
        }
        ip+=match (add_pause, op) {
            (false, Op::Noop) => {1},
            (false, Op::Addx(_)) => {add_pause=true; 0},
            (true, Op::Addx(v)) =>{add_pause=false; x+=*v as isize; 1},
            _ => {panic!();},
        };
    };
    let part1 = signal;

    let mut ip = 0usize;
    let mut x = 1isize;
    let mut add_pause = false;
    let crt: Vec<char> = (0..240isize).map(|cc| {
        // let cycle = cc+1
        let crt_x = cc%40;
        let pixel = if (crt_x-x).abs()<2 {'#'} else {'.'};
        let op = &input[ip];
        ip+=match (add_pause, op) {
            (false, Op::Noop) => {1},
            (false, Op::Addx(_)) => {add_pause=true; 0},
            (true, Op::Addx(v)) =>{add_pause=false; x+=*v as isize; 1},
            _ => {panic!();},
        };
        pixel
    }).collect();
    let part2: String = crt.chunks(40).map(|ch| ch.iter().collect::<String>()).join("\n");
    Ok([part1.to_string(), part2.to_string()])
}

// Make it simple to compare timing for multiple solutions
type Solution = dyn Fn(&str) -> Result<[String; 2]>;
const SOLUTIONS: [(&str, &Solution); 1] = [("Original", &solution)];

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test01.txt")?;
    for (name, solution) in SOLUTIONS {
        let res = solution(&input).with_context(|| format!("Running solution {}", name))?;
        println!("---\n{}\nPart 1: {}\nPart 2: {}", name, res[0], res[1]);
        assert!(res[0] == "13140");
        assert!(res[1] == "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n");
    }
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    for (_, solution) in SOLUTIONS.iter().cycle().take(10) {
        solution(&input)?;
    } //warmup
    for (name, solution) in SOLUTIONS {
        let start = Instant::now();
        let res = solution(&input)?;
        println!(
            "---\n{} ({} us)\nPart 1: {}\nPart 2: \n{}",
            name, start.elapsed().as_micros(), res[0], res[1],
        );
    }
    Ok(())
}
