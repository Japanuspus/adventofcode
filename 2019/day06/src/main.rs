#![allow(unused)]
use std::iter;
use std::collections::HashMap;


fn main() {
    let mut input = std::fs::read_to_string("input.txt").expect("Error reading input.txt");
    let edges: Vec<_> = input
        .lines().map(|s| {let mut r=s.split(')'); (r.next().unwrap(), r.next().unwrap())}).collect();

    let ancestor: HashMap<_,_> = edges.iter().map(|e| (e.1, e.0)).collect();
    let mut child_count: HashMap<&str,usize> = HashMap::new();
    for e in edges.iter() {
        child_count.entry(e.1).or_insert(0);
        child_count.entry(e.0).and_modify(|c| *c+=1).or_insert(1);
    }

    // part 1
    let mut count: usize = 0;
    let all = child_count.iter().map(|(n, c)| *n);
    for r in all {
        let mut p = r;
        loop {
            if let Some(pn) = ancestor.get(p) {
                p = pn;
                count+=1;
            } else {
                break
            }
        }
    }
    println!("Part 1: {}", &count);

    // part 2
    let mut dline = HashMap::new();
    let mut p:&str = "YOU";
    let mut d = 0;
    loop {
        if let Some(pn) = ancestor.get(p) {
            p = pn;
            dline.insert(*pn, d);
            d+=1;
        } else {
            break
        }
    }
    let mut p:&str = "SAN";
    let mut d = 0;
    loop {
        if let Some(pn) = ancestor.get(p) {
            p = pn;
            if let Some(you_d) = dline.get(pn) {
                println!("Part 1: {}", d+you_d);
                break;
            }
            d+=1;
        } else {
            break
        }
    }
}