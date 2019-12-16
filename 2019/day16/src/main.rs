#![allow(unused)]

use std::collections::HashSet;
use std::collections::HashMap;
//use std::iter;
//use day11::State;

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");
    let digs: Vec<isize> = input.chars().filter(|c| c.is_numeric()).map(|c| c.to_string().parse().unwrap()).collect();
    let n = digs.len();
    let rbase: Vec<isize> = vec![0, 1, 0, -1];
    println!("n: {}", n);

    let mut dv = digs.clone();
    for _ in 0..100 {
        dv = (1..=n).map(|r| {
            let mut rval: isize = dv.iter()
                .zip(rbase.iter().cycle().flat_map(|m| std::iter::repeat(m).take(r)).skip(1))
                .map(|(a,b)| a*b)
                .sum();
            rval = rval.abs() % 10;
            rval 
        }).collect();
        // println!("{:?}", &dv[..8]);
    }
    println!("Part 1: {}", dv.iter().rev().take(8).map(|c| format!("{}", c)).collect::<Vec<_>>().join(""));

    // Part 2
    let nskip = digs.iter().take(7).fold(0, |acc, d| acc*10+d);
    println!("Part 2, skipping {}", nskip);
    // weights are all 1 out here, cumsum from rear end is all we need
    let dv: Vec<isize> = digs.iter().cycle().take(n*10000).skip(nskip as usize).cloned().collect();
    let mut dvr: Vec<isize> = dv.iter().rev().cloned().collect();
    for _ in 0..100 {
        dvr = dvr.iter().scan(0, |acc, d| {*acc = (*acc + d) % 10; Some(*acc)}).collect();
    }
    let res: Vec<_> = dvr.iter().rev().take(8).map(|c| format!("{}", c)).collect();
    println!("part 2: {}", res.join(""));
}
