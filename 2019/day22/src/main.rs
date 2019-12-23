#![allow(unused)]

// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::iter;
// use day11::State; // dep: day11={path="../day11"}
use modinverse;
use num_bigint::{BigInt, ToBigInt};
use num_traits::cast::ToPrimitive;

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

fn next_index(deck_size: isize, cmd: &Shuffle, index: isize) -> isize {
    let res = match cmd {
        Shuffle::DealInc(k) => {
            ((index.to_bigint().unwrap() * k) % &deck_size).to_isize().unwrap()
        }
        Shuffle::Cut(v) => {
            let cut = if *v<0 {deck_size-v.abs()} else {*v};
            let rest = deck_size-cut;
            if index<cut {
                index+rest
            } else {
                index-cut
            }
        }
        Shuffle::DealNew => {
            deck_size - index -1
        }
    };
    assert!(res>=0 && res<deck_size);
    res
}

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
    let n_deck = 119315717514047; 
    let index_initial = 2020;
    let n_repeat = 101741582076661usize;
    //let ndeck = 10007;
    //let index_initial = 2019;
    println!("isize::MAX/ Deck size: {}",std::isize::MAX/n_deck);
    // MISSING n_repeat!
    let idx = cmds.iter().fold(index_initial, |idx, cmd| next_index(n_deck, cmd, idx));
    println!("Part 2: {}", idx);
}

// Reverse code is not needed -- but I realized this a bit too late...
fn previous_index(deck_size: isize, cmd: &Shuffle, index: isize) -> isize {
    match cmd {
        Shuffle::DealInc(k) => {
            // index m1 is mapped to m2 = m1*k % n
            // Solve for m1 by chinese remainders, letting x = m1*k: 
            // x = m2 % n
            // x = 0  % k
            // Given a1,a2 so that a1*n + a2*k == 1
            // x = 0*a1*n + m2*a2*k 
            // == m1 * k ==> m1 = m2*a2

            let n = deck_size;
            // Cannot call egcd with BigInt as it does not have copy trait
            let (gcd, a1, a2) = modinverse::egcd(n, *k);
            assert_eq!(gcd, 1);
            assert_eq!(
                1.to_bigint().unwrap(), 
                (a1.to_bigint().unwrap()*&n)+(a2.to_bigint().unwrap()*k)
            );
            let index_a2 = index.to_bigint().unwrap() * &a2; 
            let res = (((index_a2 % n) + n) % n).to_isize().unwrap();
            assert_eq!(res.to_bigint().unwrap() * k % n, index.to_bigint().unwrap());
            res
        }
        Shuffle::Cut(v) => {
            let cut = if *v<0 {deck_size-v.abs()} else {*v};
            let rest = deck_size-cut;
            if index<rest {
                index+cut
            } else {
                index-rest
            }
        }
        Shuffle::DealNew => {
            deck_size - index -1
        }
    }
}

#[test]
fn test_previous_index() {
    assert_eq!(previous_index(10, &Shuffle::DealNew, 4), 5);
    assert_eq!(previous_index(10, &Shuffle::DealNew, 0), 9);
    assert_eq!(previous_index(10, &Shuffle::DealNew, 9), 0);

    assert_eq!(previous_index(10, &Shuffle::Cut(3), 0), 3);
    assert_eq!(previous_index(10, &Shuffle::Cut(3), 6), 9);
    assert_eq!(previous_index(10, &Shuffle::Cut(3), 7), 0);
    assert_eq!(previous_index(10, &Shuffle::Cut(3), 9), 2);


    assert_eq!(previous_index(10, &Shuffle::Cut(-4), 0), 6);
    assert_eq!(previous_index(10, &Shuffle::Cut(-4), 3), 9);
    assert_eq!(previous_index(10, &Shuffle::Cut(-4), 4), 0);
    assert_eq!(previous_index(10, &Shuffle::Cut(-4), 9), 5);

    assert_eq!(previous_index(10, &Shuffle::DealInc(3), 0), 0);
    assert_eq!(previous_index(10, &Shuffle::DealInc(3), 1), 7);
    assert_eq!(previous_index(10, &Shuffle::DealInc(3), 2), 4);
    assert_eq!(previous_index(10, &Shuffle::DealInc(3), 3), 1);
    assert_eq!(previous_index(10, &Shuffle::DealInc(3), 4), 8);
}