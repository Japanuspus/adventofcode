#![allow(unused)]
use std::iter;
use std::collections::{HashSet, HashMap};


fn main() {
    let mut input = std::fs::read_to_string("input.txt").expect("Error reading input.txt");
    let edges: Vec<_> = input
        .lines().map(|s| {let mut r=s.split(')'); (r.next().unwrap(), r.next().unwrap())}).collect();

    let ancestor: HashMap<_,_> = edges.iter().map(|e| (e.1, e.0)).collect();
    let nodes:HashSet<_> = edges.iter().map(|e| e.0).chain(edges.iter().map(|e| e.1)).collect();

    // part 1
    let count:usize = nodes.iter().map(|r| 
        iter::successors(Some(r), |p| ancestor.get(*p)).count()
    ).sum();
    println!("Part 1: {}", &count);

    // part 2
    // All ancestors of YOU
    let mut dline: HashMap<_,_> = 
        iter::successors(Some(&"YOU"), |p| ancestor.get(*p))
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect();
    // search through SAN ancestors for for first common ancestor
    for (d, n) in iter::successors(Some(&"SAN"), |p| ancestor.get(*p)).enumerate() {
        if let Some(you_d) = dline.get(n) {
            println!("Part 2: {}", d+you_d);
            break;
        }
    }
}