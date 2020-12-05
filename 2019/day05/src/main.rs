#![allow(unused)]

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
    fn get(&mut self, m: &mut impl iter::Iterator<Item=u8>) -> isize {
        let r = match m.next() {
            Some(1) => {self.tape[self.pc]},
            _ => self.tape[self.tape[self.pc] as usize]
        };
        self.pc+=1;
        r
    }

    fn put(&mut self, v: isize) {
        let a = self.tape[self.pc] as usize;
        self.tape[a] = v;
        self.pc+=1;
    }
}

fn eval_intcode(mut s: State) -> State {
    loop {
        let m = &mut digits_from_right(&s.tape[s.pc]);
        let op=m.take(2).zip(&[1, 10]).map(|(v, m)| v*m).sum();
        s.pc += 1;
        match op {
            1 => { // add
                let v = s.get(m)+s.get(m);
                s.put(v);
            }
            2 => { // mul
                let v = s.get(m)*s.get(m);
                s.put(v);
            }
            3 => { // in
                let v = s.stack.pop().unwrap();
                s.put(v);
            }
            4 => { // out
                let a = s.get(m);
                s.stack.push(a);
            }
            5 => { // jnz
                let a = s.get(m);
                let d = s.get(m) as usize;
                if a != 0 { s.pc = d as usize;}
            }
            6 => { // jz
                let a = s.get(m);
                let d = s.get(m) as usize;
                if a == 0 { s.pc = d;}
            }
            7 => { // lt
                let v = (s.get(m) < s.get(m)) as isize;
                s.put(v);
            }
            8 => { // eq
                let v = (s.get(m) == s.get(m)) as isize;
                s.put(v);
            }
            99 => { // halt
                break;
            }
            _ => {
                dbg!(s);
                panic!("Unknown operand");
            }
        }
    };
    s
}

fn main() {
    let input: Vec<isize> = std::fs::read_to_string("input.txt")
        .expect("Error reading input file")
        .lines().next().unwrap()
        .split(',').map(|s| s.parse().unwrap())
        .collect();

    let s = eval_intcode(State{stack: vec![1], tape: input.clone(), pc: 0});
    dbg!(&s.stack);

    let s = eval_intcode(State{stack: vec![5], tape: input.clone(), pc: 0});
    dbg!(&s.stack);
}


#[test]
fn test_eval() {
    let s = eval_intcode(State{stack: vec![], tape: vec![1002,4,3,4,33], pc: 0});
    assert_eq!(s.tape[4], 99);
}


#[test] 
fn test_part2() {
    let c = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
    1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
    999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

    let s = eval_intcode(State{stack: vec![7], tape: c.clone(), pc: 0});
    assert_eq!(s.stack[0], 999);

    let s = eval_intcode(State{stack: vec![8], tape: c.clone(), pc: 0});
    assert_eq!(s.stack[0], 1000);

    let s = eval_intcode(State{stack: vec![9], tape: c.clone(), pc: 0});
    assert_eq!(s.stack[0], 1001);
}
