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
    stack: Vec<isize>,
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
}

fn eval_intcode(mut s: State) -> Result<State,()> {
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
                let v = s.stack.pop().unwrap();
                s.put(v)?;
            }
            4 => { // out
                let a = s.get(m)?;
                s.stack.push(a);
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
    Ok(s)
}

fn main() {
    let input: Vec<isize> = std::fs::read_to_string("input.txt")
        .expect("Error reading input file")
        .lines().next().unwrap()
        .split(',').map(|s| s.parse().unwrap())
        .collect();

    let mut phases = [0, 1, 2, 3, 4];
    let mut output = Vec::new();
    heap_recursive(&mut phases, |permutation| {
        let mut next_input = 0;
        for phase in permutation.iter() {
            let mut stack = vec![next_input];
            stack.push(*phase); 
            let res = eval_intcode(State{stack: stack, tape: input.clone(), pc: 0});
            match res {
                Ok(state) => {next_input = state.stack[0];}
                Err(_) => {
                    println!("intcode failed");
                    break;
                }
            };
        }
        output.push(next_input);
    });
    println!("Part 1: {}", output.iter().max().unwrap());
}



