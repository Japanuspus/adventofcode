use std::{collections::HashSet, fs};
use anyhow::Result;
// use anyhow::Context;
// use itertools::Itertools;
// use parse_display::{FromStr};
// use regex::Regex;
// use apply::Also;
// use num::{BigInt, Integer};

//#[derive(Debug, FromStr)]
//#[display("{key}:{value}")]
//struct InputItem {key: String, value: String}

type P = u32;


/// ps is indexed by cup label. value of p[i] is label of cup CW from cup labelled i
fn unwrap_pointers(ps: &[P], cur: P) -> Vec<P> {
    let mut res = Vec::new();
    let mut p = cur;
    for _ in 0..ps.len()+1 {
        res.push(p);
        p = ps[p as usize];
        if p==cur {return res;}
    }
    panic!("Loop while unfolding, cur={}:\n 0, 1, 2, 3, 4, 5, 6, 7, 8, 9\n{:?}", cur, ps);
}

fn part1(input: &str) {
    let init: Vec<P> = input.trim().chars().map(|c| c.to_digit(10).unwrap() as P).collect();
    let n = init.len();

    let mut pointers: [P; 10] =[0; 10];
    for w in init.windows(2) {
        pointers[w[0] as usize] = w[1];
    }
    pointers[init[n-1] as usize] = init[0];
    let mut cur = init[0];

    println!("Input: {}, as pointer: {:?}", input, unwrap_pointers(&pointers, cur));

    for idx in 0..100 {
        // take out three
        let t0 = pointers[cur as usize]; // first taken out
        let t1 = pointers[t0 as usize]; // first taken out
        let t2 = pointers[t1 as usize]; // first taken out
        let ts: HashSet<P> = [t0, t1, t2].iter().cloned().collect();

        pointers[cur as usize] = pointers[t2 as usize];
        
        // destination: reduce value of cur by 1 until valid
        let mut dst = if cur>1 {cur-1} else {n as P};  //labels are 1..(n+1)
        while ts.contains(&dst) {
            dst = if dst>1 {dst-1} else {n as P};  //labels are 1..(n+1)
        }

        // insert ts
        pointers[t2 as usize] = pointers[dst as usize];
        pointers[dst as usize] = t0;

        // update cur
        cur = pointers[cur as usize];

        println!("Round {}: {:?} - current: {}", idx, unwrap_pointers(&pointers, cur), &cur);
    }
    println!("From 1: {:?}", unwrap_pointers(&pointers, 1));

}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    //part1("389125467");
    part1(&input);


    Ok(())
}
