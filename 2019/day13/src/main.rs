#![allow(unused)]

extern crate num_bigint;
extern crate num_traits;

use std::collections::{HashSet, HashMap};
use std::iter;
use num_bigint::{ToBigInt, BigInt};
use num_traits::{Zero, One};
use crate::num_traits::ToPrimitive;
use std::mem::replace;

fn digits_from_right(k: &BigInt) -> impl Iterator<Item=u8> {
    k.to_string().into_bytes().into_iter().rev().map(|c| c-b'0').chain(iter::repeat(0))
}

#[derive(Debug)]
pub struct State {
    tape: HashMap<BigInt, BigInt>,
    pc: BigInt,
    sp: BigInt //"relative base"
}

impl State {
    fn get_adress(&mut self, m: &mut impl iter::Iterator<Item=u8>) -> BigInt {
        // TODO: Avoid key clones by returning reference to static zero on lookup miss
        let g1 = self.pc.clone();
        match m.next() {
            Some(0) => { //normal
                self.tape.entry(g1).or_insert(Zero::zero()).clone()
            }
            Some(1) => { // immediate
                g1
            }
            Some(2) => { //relative
                self.tape.entry(g1).or_insert(Zero::zero()).clone() + &self.sp
            }
            _ => {panic!("Unknown mode");}
        }
    }

    fn get(&mut self, m: &mut impl iter::Iterator<Item=u8>) -> &BigInt {
        let g0 = self.get_adress(m);
        self.pc+=1;
        self.tape.entry(g0).or_insert(Zero::zero())
    }

    fn put(&mut self, v: BigInt, m: &mut impl iter::Iterator<Item=u8>) {
        let g0 = self.get_adress(m);
        self.pc+=1;
        self.tape.insert(g0, v);
    }

    pub fn next_output(&mut self, inputs: &[BigInt]) -> Result<Option<BigInt>, ()> {
        next_output(self, inputs)
    }
}

fn next_output(s: &mut State, inputs: &[BigInt]) -> Result<Option<BigInt>,()> {
    let mut iter_input = inputs.iter();
    loop {
        let m = &mut digits_from_right(
            s.tape.get(&s.pc).unwrap_or(&Zero::zero())
        );
        let op=m.take(2).zip(&[1, 10]).map(|(v, m)| v*m).sum();
        s.pc += 1;
        match op {
            1 => { // add
                let v = s.get(m).clone() + s.get(m);
                s.put(v, m);
            }
            2 => { // mul
                let v = s.get(m).clone() * s.get(m);
                s.put(v, m);
            }
            3 => { // in
                s.put(iter_input.next().ok_or(())?.clone(), m);
            }
            4 => { // out
                let a = s.get(m);
                return Ok(Some(a.clone()));
            }
            5 => { // jnz
                let cond = *s.get(m) != Zero::zero();
                let d = s.get(m);
                if cond { s.pc = d.clone();}
            }
            6 => { // jz
                let cond = *s.get(m) == Zero::zero();
                let d = s.get(m);
                if cond { s.pc = d.clone();}
            }
            7 => { // lt
                let v = (s.get(m).clone() < *s.get(m));
                s.put(if v {One::one()} else {Zero::zero()}, m);
            }
            8 => { // eq
                let v = (s.get(m).clone() == *s.get(m));
                s.put(if v {One::one()} else {Zero::zero()}, m);
            }
            9 => { // adjust relbase
                let v = s.get(m).clone();
                s.sp += v;
            }
            99 => { // halt
                break;
            }
            _ => {
                // dbg!(s);
                println!("Unknown operand");
                return Err(());
            }
        }
    };
    Ok(None)
}

pub fn from_str(s: &str) -> State {
    let input: Vec<BigInt> = s
        .lines().next().unwrap()
        .split(',').map(|s| s.parse().unwrap())
        .collect();

    State{
        tape: input.iter().enumerate().map(|(i,v)| (i.to_bigint().unwrap(), v.clone())).collect(),
        pc: Zero::zero(),
        sp: Zero::zero()
    }
}

fn read_point(s: &mut State, inputs: &Vec<BigInt>) -> Option<Vec<isize>> {
    let mut r = Vec::new();
    for _ in 0..3 {
        if let Some(o) = s.next_output(&inputs[..]).unwrap() {
            r.push(o.to_isize().unwrap())
        } else {
            return None
        }
    }
    Some(r)
}     

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");
    let mut s = from_str(&input);
    let inputs:Vec<BigInt> = vec![One::one()];
    let mut res = Vec::new();
    loop {
        if let Some(p) = read_point(&mut s, &inputs) {
            res.push(p);
        } else {
            break
        }
    }
    println!("Part 1: {}", res.iter().filter(|c| c[2]==2).count());
}
