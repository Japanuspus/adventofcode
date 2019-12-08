#![allow(unused)]

use permutohedron::heap_recursive;
use std::collections::{HashSet, HashMap};
use std::iter;

fn digits_from_right(k: &isize) -> impl Iterator<Item=u8> {
    k.to_string().into_bytes().into_iter().rev().map(|c| c-b'0').chain(iter::repeat(0))
}

#[test]
fn test_digits_from_right() {
    let mut i = digits_from_right(&20);
    assert_eq!(i.next(), Some(0));
    assert_eq!(i.next(), Some(2));
    assert_eq!(i.next(), Some(0));
    assert_eq!(i.next(), Some(0));
}

#[derive(Debug)]
struct State {
    tape: Vec<isize>,
    pc: usize
}

impl State {
    fn get(&mut self, m: &mut impl iter::Iterator<Item=u8>) -> Result<isize,()> {
        let g1 = self.tape.get(self.pc).ok_or(())?;
        let r = match m.next() {
            Some(1) => g1,
            _ => self.tape.get(*g1 as usize).ok_or(())?
        };
        self.pc+=1;
        Ok(*r)
    }

    fn put(&mut self, v: isize) -> Result<(),()> {
        let a = *self.tape.get(self.pc).ok_or(())? as usize;
        if a < self.tape.len() {
            self.tape[a] = v;
            self.pc+=1;
            Ok(())
        } else {
            Err(())
        }
    }

    fn next_output(&mut self, inputs: &[isize]) -> Result<Option<isize>, ()> {
        next_output(self, inputs)
    }
}

fn next_output(s: &mut State, inputs: &[isize]) -> Result<Option<isize>,()> {
    let mut iter_input = inputs.iter();
    loop {
        let m = &mut digits_from_right(
            s.tape.get(s.pc).ok_or(())?
        );
        let op=m.take(2).zip(&[1, 10]).map(|(v, m)| v*m).sum();
        s.pc += 1;
        match op {
            1 => { // add
                let v = s.get(m)?+s.get(m)?;
                s.put(v)?;
            }
            2 => { // mul
                let v = s.get(m)?*s.get(m)?;
                s.put(v)?;
            }
            3 => { // in
                s.put(*iter_input.next().ok_or(())?)?;
            }
            4 => { // out
                let a = s.get(m)?;
                return Ok(Some(a));
            }
            5 => { // jnz
                let a = s.get(m)?;
                let d = s.get(m)? as usize;
                if a != 0 { s.pc = d as usize;}
            }
            6 => { // jz
                let a = s.get(m)?;
                let d = s.get(m)? as usize;
                if a == 0 { s.pc = d;}
            }
            7 => { // lt
                let v = (s.get(m)? < s.get(m)?) as isize;
                s.put(v)?;
            }
            8 => { // eq
                let v = (s.get(m)? == s.get(m)?) as isize;
                s.put(v)?;
            }
            99 => { // halt
                break;
            }
            _ => {
                //dbg!(s);
                println!("Unknown operand");
                return Err(());
            }
        }
    };
    Ok(None)
}

fn eval_first(input: &Vec<isize>, phases: &[isize], input0: isize) -> (Vec<State>, isize) {
    let mut states = Vec::new();
    let mut next_input = input0;
    for p in phases {
        let int_code_inputs = vec![*p, next_input];
        let mut s = State{tape: input.clone(), pc: 0};
        if let Some(res) = s.next_output(&int_code_inputs[..]).unwrap() {
            next_input = res;
        } else {
            dbg!(&s);
            panic!("No output before halt!");
        }
        states.push(s);
    }
    (states, next_input)
}

fn eval_other(states: &mut Vec<State>, input: isize) -> Option<isize> {
    let mut next_input = input;
    for s in states {
        if let Some(v) = s.next_output(&[next_input]).unwrap() {
            next_input = v;
        } else {
            return None;
        }
    }
    Some(next_input)
}

fn main() {
    let input: Vec<isize> = std::fs::read_to_string("input.txt")
        .expect("Error reading input file")
        .lines().next().unwrap()
        .split(',').map(|s| s.parse().unwrap())
        .collect();

    // part 1
    let mut outputs = Vec::new();
    let mut phases = [0, 1, 2, 3, 4];
    heap_recursive(&mut phases, |permutation| {
        outputs.push(eval_first(&input, permutation, 0).1);
    });
    println!("Part 1: {}", outputs.iter().max().unwrap());


    // part 2
    let mut outputs = Vec::new();
    let mut phases = [5, 6, 7, 8, 9];
    heap_recursive(&mut phases, |permutation| {
        let (mut states, input0) = eval_first(&input, permutation, 0);
        let mut next_input = input0;
        loop {
            if let Some(v) = eval_other(&mut states, next_input) {
                next_input = v;
            } else {
                outputs.push(next_input);
                break;
            }
        }
    });
    println!("Part 2: {}", outputs.iter().max().unwrap());
    
}



