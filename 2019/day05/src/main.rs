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

fn get_next(s: &mut State, mode: Option<u8>) -> &isize{
    let r = match mode {
        Some(1) => {&s.tape[s.pc]},
        _ => &s.tape[s.tape[s.pc] as usize]
    };
    s.pc+=1;
    r
}

fn put_next(s: &mut State, v: isize) {
    let a = s.tape[s.pc] as usize; 
    s.tape[a] = v;
    s.pc+=1;
}

fn eval_intcode(mut s: State) -> State {
    loop {
        let mut opit = digits_from_right(&s.tape[s.pc]);
        let op=opit.next().unwrap() + 10*opit.next().unwrap();
        s.pc += 1;
        match op {
            99 => {break;}
            4 => {
                let a = *get_next(&mut s, opit.next());
                s.stack.push(a);
            }
            3 => {
                let v = s.stack.pop().unwrap();
                put_next(&mut s, v);
            }
            1 => {
                let a = *get_next(&mut s, opit.next());
                let b = *get_next(&mut s, opit.next());
                put_next(&mut s, a+b);
            }
            2 => {
                let a = *get_next(&mut s, opit.next());
                let b = *get_next(&mut s, opit.next());
                put_next(&mut s, a*b);
            }
            _ => {
                dbg!(s);
                panic!("Unknown operand");
            }
        }
    };
    s
}

#[test]
fn test_eval() {
    let s = eval_intcode(State{stack: vec![], tape: vec![1002,4,3,4,33], pc: 0});
    assert_eq!(s.tape[4], 99);
}

fn main() {
    let input: Vec<isize> = std::fs::read_to_string("input.txt")
        .expect("Error reading input file")
        .lines().next().unwrap()
        .split(',').map(|s| s.parse().unwrap())
        .collect();

    let s = eval_intcode(State{stack: vec![1], tape: input.clone(), pc: 0});
    
    dbg!(&s.stack);
}