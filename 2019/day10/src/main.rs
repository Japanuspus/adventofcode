#![allow(unused)]

extern crate num;

use num::integer::{Integer, gcd};

use std::collections::{HashSet, HashMap};
use std::iter;

fn direction(p0: &(usize, usize), p1: &(usize, usize)) -> (isize, isize) {
    let dx = p1.0 as isize - p0.0 as isize;
    let dy = p1.1 as isize - p0.1 as isize;
    let g = gcd(dx, dy);
    if g>0 {
        (dx/g, dy/g)
    } else {
        (dx, dy)
    }
}

fn count_directions(p: &(usize, usize), pts: &Vec<(usize, usize)>) -> usize {
    let dset: HashSet<_> = pts.iter()
        .map(|p1| direction(p, p1)).collect();
    return dset.len()
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");
    let poss: Vec<(usize, usize)> = input
        .lines().enumerate()
        .flat_map(
            |(y, ln)| ln.chars().enumerate().filter_map(
                move |(x, c)| if c=='#' {Some((x,y))} else {None}))
        .collect();
    
    // part 1
    let max_view = poss.iter().map(|p| count_directions(p, &poss)).max().unwrap();
    println!("Part 1: {}", max_view-1); //includes self
}
