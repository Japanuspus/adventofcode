use anyhow::{Context, Result};
use itertools::iterate;
use parse_display::FromStr;
use std::{collections::HashMap, fs};

#[derive(Debug, FromStr)]
enum Command {
    #[display("mem[{0}] = {1}")]
    Mem(usize, usize),
    #[display("mask = {0}")]
    Mask(String),
}

#[derive(Debug)]
struct Mask {
    mask: usize,
    value: usize,
}

fn parse_mask(s: &str) -> Mask {
    s.chars().fold(Mask { mask: 0, value: 0 }, |mut m, c| {
        m.mask <<= 1;
        m.value <<= 1;
        match c {
            '1' => {
                m.mask |= 1;
                m.value |= 1;
            }
            '0' => m.mask |= 1,
            'X' => {}
            _ => panic!("bad char in mask"),
        };
        m
    })
}

fn part1(input: &str) -> Result<usize> {
    let commands: Vec<_> = input
        .lines()
        .map(|ln| ln.parse::<Command>().context(format!("Parsing {}", ln)))
        .collect::<Result<_, _>>()?;

    let mut ram: HashMap<usize, usize> = HashMap::new();
    let mut mask: Mask = Mask { mask: 0, value: 0 };
    for cmd in commands.iter() {
        match cmd {
            Command::Mem(addr, val) => {
                let masked_val = mask.value | (!mask.mask & val);
                // println!("{:#016b}, {:#016b}", val, masked_val);
                ram.insert(*addr, masked_val);
            }
            Command::Mask(s) => {
                mask = parse_mask(&s);
                // println!("Mask mask: {:#036b}\n    value: {:#036b}", mask.mask, mask.value);
            }
        }
    }

    Ok(ram.iter().map(|(_, v)| v).sum::<usize>())
}

const MASK36: usize = 0xFFF_FFF_FFF;

fn float_iter(f: usize) -> impl Iterator<Item = usize> {
    let idx: Vec<usize> = iterate(f, |v| v >> 1)
        .take(36)
        .enumerate()
        .filter(|(_, v)| v & 1 > 0)
        .map(|(i, _)| i)
        .collect();
    let n = 2usize.pow(idx.len() as u32);
    (0..n).map(move |bits| {
        iterate(bits, |v| v >> 1)
            .map(|v| v & 1)
            .zip(idx.iter())
            .map(|(b, i)| b << i)
            .sum()
    })
}

fn part2(input: &str) -> Result<usize> {
    let commands: Vec<_> = input
        .lines()
        .map(|ln| ln.parse::<Command>().context(format!("Parsing {}", ln)))
        .collect::<Result<_, _>>()?;

    let mut ram: HashMap<usize, usize> = HashMap::new();
    let mut mask: Mask = Mask { mask: 0, value: 0 };
    for cmd in commands.iter() {
        match cmd {
            Command::Mem(addr, val) => {
                let masked_val = *val;
                let floats = MASK36 ^ mask.mask;
                let addr0 = (addr | mask.value) & !floats;
                for float_val in float_iter(floats) {
                    let addr = addr0 | float_val;
                    //println!("{:#036b} <- {:#036b}", addr, masked_val);
                    ram.insert(addr, masked_val);
                }
            }
            Command::Mask(s) => {
                mask = parse_mask(&s);
                // println!("Mask mask: {:#036b}\n    value: {:#036b}", mask.mask, mask.value);
            }
        }
    }
    Ok(ram.iter().map(|(_, v)| v).sum::<usize>())
}

#[test]
fn test_part1() {
    let s = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    assert_eq!(part1(s).ok(), Some(165));
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}
