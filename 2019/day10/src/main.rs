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

fn dist2(p0: &(usize, usize), p1: &(usize, usize)) -> isize {
    let dx = p1.0 as isize - p0.0 as isize;
    let dy = p1.1 as isize - p0.1 as isize;
    dx*dx + dy*dy
}

fn count_directions(p: &(usize, usize), pts: &Vec<(usize, usize)>) -> usize {
    let dset: HashSet<_> = pts.iter()
        .map(|p1| direction(p, p1)).collect();
    return dset.len()
}

fn direction_angle(dp: &(isize, isize)) -> f64 {
    // clockwise from top
    let dy=dp.0 as f64;
    let dx=-dp.1 as f64;
    let phi = dy.atan2(dx);
    if phi<0.0 {phi+2.0*std::f64::consts::PI} else {phi}
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");
    
    let poss: Vec<(usize, usize)> = input
        .lines().enumerate()
        .inspect(|(i, l)| println!("Line {}: {}", i, l))
        .flat_map(
            |(y, ln)| ln.chars().enumerate().filter_map(
                move |(x, c)| if c=='#' {Some((x,y))} else {None}))
        .collect();
    
    // part 1
    let (p0, c_max) = poss.iter().map(|p| (p, count_directions(p, &poss))).max_by_key(|(p, c)| *c).unwrap();
    println!("Part 1: {} from {:?}", c_max-1, &p0); //includes self

    // part 2
    let mut by_dir = HashMap::new();
    for p in poss.iter().filter(|pp| *pp != p0) {
        by_dir.entry(direction(p0, p)).or_insert_with(Vec::new).push(p);
    }
    // order directions by clockwise angle from top
    let mut dirs: Vec<_> = by_dir.keys().cloned().collect();
    dirs.sort_by(|a,b| direction_angle(a).partial_cmp(&direction_angle(b)).unwrap());
    // order entries by distance from p0
    for vp in by_dir.values_mut() {
        vp.sort_by_key(|p| -dist2(p, p0)); //smallest dist at back for pop
    }

    let mut pts = Vec::new();
    let mut i = 0;
    for d in dirs.iter().cycle() {
        if let Some(v) = by_dir.get_mut(d).unwrap().pop() {
            pts.push(v);
        }
        if pts.len()>=poss.len()-1 {
            break
        }
    }
    //for (i, p) in pts.iter().enumerate() {println!("{}: {:?} -- {:?}", i, &p0, p);}
    println!("Part 2: {:?}", pts[199]);
}
