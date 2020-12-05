#![allow(unused)]
use std::str;

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");
    let line0 = input.lines().next().unwrap();
    let layers: Vec<_> = line0.as_bytes().chunks(6*25).collect();
        
    let idx = layers.iter().min_by_key(|l| l.iter().filter(|c| **c==b'0').count()).unwrap();
    let res: Vec<_> = [b'1', b'2'].iter().map(|cc| idx.iter().filter(|c| **c==*cc).count()).collect();
    println!("Part 1: {}", res[0]*res[1]);

    //part2
    let mut res: Vec<u8> = layers[0].iter().cloned().collect();
    for l in layers.iter() {
        res = res.into_iter().zip(l.iter()).map(|(top, c)| 
            if top==b'2' {*c} else {top}
        ).collect();
    }
    println!("Part 2:");
    for l in res.chunks(25) {
        println!("{}", str::from_utf8(l).unwrap());
    }
}
