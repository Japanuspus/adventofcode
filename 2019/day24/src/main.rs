#![allow(unused)]

use std::collections::BTreeSet;
// use std::collections::HashMap;
use std::iter;
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

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");

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