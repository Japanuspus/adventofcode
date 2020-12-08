use anyhow::{Context, Result};
use std::{collections::HashSet, fs};
use parse_display::FromStr;
use apply::Also;

#[derive(Debug, FromStr, Clone, Copy)]
#[display("{}", style = "lowercase")]
enum Operation {
    ACC,
    JMP,
    NOP,
}

#[derive(Debug, FromStr, Clone)]
#[display("{op} {arg}")]
struct Instruction {
    op: Operation,
    arg: isize,
}

#[derive(Debug)]
struct RegisterState {
    pc: usize,
    acc: isize,
}

#[derive(Debug, Clone, Copy)]
enum Flag {
    HALT,
}

impl RegisterState {
    fn new() -> Self {
        Self { pc: 0, acc: 0 }
    }

    fn step(&mut self, prog: &Vec<Instruction>) -> Option<Flag> {
        if let Some(inst) = prog.get(self.pc) {
            if match inst.op {
                Operation::ACC => {
                    self.acc += inst.arg;
                    true
                }
                // look at https://stackoverflow.com/a/59209602/212538
                Operation::JMP => {
                    self.pc = (self.pc as isize + inst.arg) as usize;
                    false
                }
                Operation::NOP => true,
            } {
                self.pc += 1;
            }
            None
        } else {
            Some(Flag::HALT)
        }
    }
}

fn run_loop_check(prog: &Vec<Instruction>) -> (RegisterState, Option<Flag>) {
    let mut state = RegisterState::new();
    let mut visited: HashSet<usize> = HashSet::new();
    while visited.insert(state.pc) {
        if let Some(flag) = state.step(&prog) {
            return (state, Some(flag));
        }
        // println!("{:?}", state)
    }
    (state, None)
}

fn main() -> Result<()> {
    let prog: Vec<_> = fs::read_to_string("input.txt")?
        .lines()
        .map(|ln| {
            ln.parse::<Instruction>()
                .with_context(|| format!("Error parsing instruction \"{}\"", ln))
        })
        .collect::<Result<_, _>>()?;

    println!("Part 1: {}", run_loop_check(&prog).0.acc);

    println!("Part 2: {}",
        prog.iter().enumerate()
        .filter_map(|(i, inst)| match inst.op {
                Operation::JMP => Some((i, Operation::NOP)),
                Operation::NOP => Some((i, Operation::JMP)),
                _ => None,
            })
        .map(|(i, op)| prog.clone().also(|v| v[i].op = op))
        .find_map(|v| match run_loop_check(&v) {
            (state, Some(Flag::HALT)) => Some(state.acc),
            _ => None,
        }).unwrap_or(0));

    Ok(())
}
