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
        let res: Vec<_> = (1..=n).map(|r| {
            let mut rval: isize = dv.iter()
                .zip(rbase.iter().cycle().flat_map(|m| std::iter::repeat(m).take(r)).skip(1))
                .map(|(a,b)| a*b)
                .sum();
            rval = rval.abs() % 10;
            rval 
        }).collect();
        dv = res;
        println!("{:?}", &dv[..8]);
    }
}
