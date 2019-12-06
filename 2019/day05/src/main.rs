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
            1 => { // add
                let a = *get_next(&mut s, opit.next());
                let b = *get_next(&mut s, opit.next());
                put_next(&mut s, a+b);
            }
            2 => { // mul
                let a = *get_next(&mut s, opit.next());
                let b = *get_next(&mut s, opit.next());
                put_next(&mut s, a*b);
            }
            4 => { // out
                let a = *get_next(&mut s, opit.next());
                s.stack.push(a);
            }
            3 => { // in
                let v = s.stack.pop().unwrap();
                put_next(&mut s, v);
            }
            5 => { // jnz
                let a = *get_next(&mut s, opit.next());
                let d = *get_next(&mut s, opit.next());
                if a != 0 {
                    s.pc = d as usize;
                }
            }
            6 => { // jz
                let a = *get_next(&mut s, opit.next());
                let d = *get_next(&mut s, opit.next());
                if a == 0 {
                    s.pc = d as usize;
                }
            }
            7 => { // lt
                let a = *get_next(&mut s, opit.next());
                let b = *get_next(&mut s, opit.next());
                put_next(&mut s, (a < b) as isize)
            }
            8 => { // eq
                let a = *get_next(&mut s, opit.next());
                let b = *get_next(&mut s, opit.next());
                put_next(&mut s, (a == b) as isize)
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