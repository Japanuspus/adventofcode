use anyhow::{Context, Result};
use std::{collections::HashSet, fs};
use parse_display::FromStr;

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
struct State {
    pc: usize,
    acc: isize,
}

#[derive(Debug, Clone, Copy)]
enum Flag {
    HALT,
}

impl State {
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

fn run_loop_check(prog: &Vec<Instruction>) -> (Option<Flag>, State) {
    let mut state = State::new();
    let mut visited: HashSet<usize> = HashSet::new();
    while visited.insert(state.pc) {
        if let Some(flag) = state.step(&prog) {
            return (Some(flag), state);
        }
        // println!("{:?}", state)
    }
    (None, state)
}

fn main() -> Result<()> {
    let prog: Vec<_> = fs::read_to_string("input.txt")?
        .lines()
        .map(|ln| {
            ln.parse::<Instruction>()
                .with_context(|| format!("Error parsing instruction \"{}\"", ln))
        })
        .collect::<Result<_, _>>()?;

    println!("Part 1: {}", run_loop_check(&prog).1.acc);

    for i in 0..prog.len() {
        let op = match &prog[i].op {
            Operation::JMP => Operation::NOP,
            Operation::NOP => Operation::JMP,
            _ => continue,
        };
        let prog2: Vec<_> = prog
            .iter()
            .enumerate()
            .map(|(ip, inst)| {
                if ip == i {
                    Instruction { op, arg: inst.arg }
                } else {
                    inst.clone()
                }
            })
            .collect();
        if let (Some(_), state) = run_loop_check(&prog2) {
            println!("Part 2: {}", state.acc);
        }
    }

    Ok(())
}
