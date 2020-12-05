#![allow(unused)]

use std::collections::BTreeSet;
// use std::collections::HashMap;
use std::iter;
use either::Either;
// use day11::State; // dep: day11={path="../day11"}

fn board_step(b: &Vec<u8>) -> (Vec<u8>, usize) {
    let mut res:Vec<u8> = iter::repeat(0).take(b.len()).collect();
    let npad = 7;
    let mut score: usize = 0;
    let mut exp: usize = 1;
    for y in 1..npad-1 {
        for x in 1..npad-1 {
            let i = y*npad+x;
            let ct = b[i-npad]+b[i+npad]+b[i-1]+b[i+1];
            let bug = b[i]==1;
            let bug_res = (bug && ct==1) || (!bug && (ct==1 || ct==2));
            if y>0 && y<npad-1 && x>0 && x<npad-1 {
                if bug_res {
                    score += exp;
                }
                exp=exp*2;
            }
            res[i] = bug_res as u8;
        }
    }
    (res, score)
}

fn print_board(b: &Vec<u8>) {
    let npad = 7;
    for y in 1..npad-1 {
        for x in 1..npad-1 {
            let i = y*npad+x;
            print!("{}", b[i]);
        }
        println!("");
    }
}

fn part1(input: &str) {
    let npad = 5+2;
    let board0: Vec<u8> = 
    iter::repeat(0).take(npad)
    .chain(
        input.lines().flat_map(|ln| 
            iter::once(0)
            .chain(ln.chars().map(|c| if c=='#' {1} else {0}))
            .chain(iter::once(0))
        ))
    .chain(iter::repeat(0).take(npad))
    .collect();
    assert_eq!(board0.len(), npad*npad);
    print_board(&board0);


    // Part 1
    let mut seen = BTreeSet::new();
    let mut board=board0.clone();
    loop {
        let (b, score) = board_step(&board);
        board = b;
        if seen.contains(&score) {
            println!("Part 1: {}", score);
            break
        }
        seen.insert(score);
    }
}


// Part 2 code below
// Connection lists are constructed as iterators which 
// turned out to be a bit clunky.
// In retrospect, it would have been easier to build the
// edge set once

type Loc = (isize, isize, isize);

// y>=x>=0, not (0, 0)
// coords: 0,0 is center pos x/y/z: right, up, inwards
fn base_connections(lc: &Loc) -> impl Iterator<Item=Loc> {
    iter::once((lc.0-1, lc.1, lc.2)) //W
    .chain(iter::once((lc.0+1, lc.1, lc.2))) //E
    .chain(if lc.1==2 { //N
        Either::Left(iter::once((0, 1, lc.2-1)))
    } else {
        Either::Right(iter::once((lc.0, lc.1+1, lc.2)))
    })
    .chain(if lc.0==0 && lc.1==1 { //S
        let z = lc.2+1;
        Either::Left((-2..=2).into_iter().map(move |x| (x, 2, z)))
    } else {
        Either::Right(iter::once((lc.0, lc.1-1, lc.2)))
    })
}

#[test]
fn test_base_connections() {
    assert_eq!(base_connections(&(1,2,0)).count(), 4);
    assert_eq!(base_connections(&(2,2,0)).count(), 4);
    assert_eq!(base_connections(&(1,1,0)).count(), 4);
    assert_eq!(base_connections(&(0,1,0)).count(), 8);
}

fn connections(lc: &Loc) -> impl Iterator<Item=Loc> {
    let sx = if lc.0<0 {-1} else {1};
    let sy = if lc.1<0 {-1} else {1};
    let lcp = (lc.0*sx, lc.1*sy, lc.2);
    let swp = lcp.0 > lcp.1;
    if swp {Either::Left(
        base_connections(&(lcp.1, lcp.0, lcp.2))
        .map(|(x,y,z)| (y,x,z))
    )} else {Either::Right(
        base_connections(&lcp)
    )}
    .map(move |(x,y,z)| (x*sx, y*sy, z))
}

#[test]
fn test_connections() {
    assert_eq!(connections(&(1,2,0)).count(), 4);
    assert_eq!(connections(&(2,2,0)).count(), 4);
    assert_eq!(connections(&(1,1,0)).count(), 4);
    assert_eq!(connections(&(0,1,0)).count(), 8);
    assert_eq!(connections(&(0,-1,0)).count(), 8);
    assert_eq!(connections(&(-1,0,0)).count(), 8);
    assert_eq!(connections(&(1,0,0)).count(), 8);
}

type BugSet = BTreeSet<Loc>;
fn step_bugs(bugs: &BugSet) -> BugSet {
    let new_candidates: BugSet = bugs.iter()
    .flat_map(|loc| connections(loc))
    .collect();

    let new_bugs: BugSet = new_candidates
    .difference(bugs)
    .filter(|loc| 
        connections(loc)
        .filter(|lc| bugs.contains(lc))
        .count()==1)
    .cloned().collect();
    
    let survivors: BugSet = bugs.iter()
    .filter(|loc| {
        let connection_count = 
        connections(loc)
        .filter(|lc| bugs.contains(lc))
        .count();
        connection_count==2 || connection_count==3
    })
    .cloned().collect();
    println!("New bugs: {}, survivors: {}", new_bugs.len(), survivors.len());

    new_bugs.union(&survivors).cloned().collect()
}


fn part2(input: &str) {
    let mut bugs: BTreeSet<Loc> = input
    .lines().enumerate()
    .flat_map(|(y, ln)|
        ln
        .chars().enumerate()
        .filter(|(_, c)| *c=='#')
        .map(move |(x, _)| (x as isize, y as isize, 0)))
    .collect();

    println!("Initial bugs: {}", bugs.iter().count());
    
    let mut bugs200 = bugs.clone();
    for idx in 0..200 {
        print!("Iter {:4}: ", idx);
        bugs200 = step_bugs(&bugs200);
    }
    //let bugs200 = (0..200).fold(bugs, |acc, _| step_bugs(&acc));
    println!("Part 2: {}", bugs200.len());
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");

    part1(&input);
    //dbg!(connections(&(0,1,0)).collect::<Vec<_>>());
    part2(&input);
}