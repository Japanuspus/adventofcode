#![allow(unused)]

use std::collections::BTreeSet;
// use std::collections::HashMap;
// use std::iter;
// use day11::State; // dep: day11={path="../day11"}
use modinverse;
use num_bigint::{BigInt, ToBigInt};
use num_traits::cast::{ToPrimitive};
use num_integer::{Integer};

#[derive(Debug)]
enum Shuffle {
    DealInc(isize),
    Cut(isize),
    DealNew
}

// deal with increment 64
// deal into new stack
// cut 1004
fn parse_shuffle(ln: &str) -> Shuffle {
    let p: Vec<_> = ln.split(' ').collect();
    let n = p.len();
    match p[n-2] {
        "increment" => Shuffle::DealInc(p[n-1].parse().unwrap()),
        "new" => Shuffle::DealNew,
        "cut" => Shuffle::Cut(p[n-1].parse().unwrap()),
        _ => {panic!();}
    }
}

fn do_shuffle(deck: &[u16], cmd: &Shuffle) -> Vec<u16> {
    match cmd {
        Shuffle::DealInc(n) => {
            let n = *n as usize;
            let ndeck = deck.len();
            let mut d2: Vec<u16> = (0..(ndeck as u16)).collect();
            let mut p = 0;
            for v in deck {
                d2[p] = *v;
                p = (p+n) % ndeck;
            }
            d2
        }
        Shuffle::Cut(v) => {
            let k = if *v<0 {deck.len()-v.abs() as usize} else {*v as usize};
            deck[k..].iter().chain(deck[..k].iter()).cloned().collect()
        }
        Shuffle::DealNew => 
            deck.iter().rev().cloned().collect()
    }
}

// All shuffle operations are linear maps (mod decksize)
fn coefficients(cmd: &Shuffle) -> (isize, isize) {
    match cmd {
        Shuffle::DealInc(k) => (*k, 0),
        Shuffle::Cut(v) => (1, -v),
        Shuffle::DealNew => (-1, -1)
    }
}


/// Return coefficients corresponding to applying first m0 and then m1
/// 
/// Corresponds to matrix product
/// (a1 b1)  (a0 b0)
/// (0   1)  (    1)
/// 
/// (a1*a0 a1*b0+b1)
/// (    0        1)

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");

    let cmds: Vec<Shuffle> = input
    .lines()
    .map(|ln| parse_shuffle(ln))
    .collect();
        
    let deck0: Vec<u16> = (0..10007).collect();
    let deck = cmds.iter().fold(deck0, |d, c| do_shuffle(&d, c));
    let (idx,_) = deck.iter().enumerate().filter(|(i, v)| **v == 2019).next().unwrap();
    println!("Part 1: {}", idx);

    // Part 2
    // combined map
    let (a_comb, b_comb) = cmds.iter()
    .map(coefficients)
    .fold(
        (1.to_bigint().unwrap(), 0.to_bigint().unwrap()),
        // a1, b1 is applied last
        |(a0, b0), (a1,b1)| (a0*a1, a1*b0+b1));
    println!("Combined map: {}*x+{}", a_comb, b_comb);

    let n_deck:usize = 119315717514047; 
    let index_initial = 2020;
    let n_repeat:usize = 101_741_582_076_661;
    //let n_deck:usize = 10007;
    //let index_initial = 2019;
    //let n_repeat:usize = 5;

    let index = 
        (index_initial.to_bigint().unwrap() * &a_comb + &b_comb) % n_deck;
    println!("Part 2, single shuffle round: {}", (index+n_deck)%n_deck);

    //let mut idx=index_initial.to_bigint().unwrap();
    //for i in 0..n_repeat { idx = (idx*&a+&b)%n_deck; }
    //println!("part 2 naive: {}", (idx+n_deck)%n_deck);

    let mut a=a_comb % n_deck;
    let mut b=b_comb % n_deck;
    let mut idx=index_initial.to_bigint().unwrap();
    let mut bits=n_repeat;
    loop {
        if bits & 1 != 0 {
            // println!("applying");
            idx = (idx*&a+&b)%n_deck;
        }
        // println!("Bits: {}", &bits);
        bits>>=1;
        if bits==0 {break}
        let b2 = b.clone()*&a+&b;
        let a2 = a.clone()*&a;
        a=a2 % n_deck;
        b=b2 % n_deck;
    }
    println!("part 2: {}", (idx+n_deck)%n_deck);

}
