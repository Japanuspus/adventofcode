#![allow(unused)]

// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::iter;
use day11::State; // dep: day11={path="../day11"}

fn probe(s0: &State, x: isize, y: isize) -> isize {
    let mut s = s0.clone();
    let mut xy = std::iter::once(x)
    .chain(std::iter::once(y))
    .map(|v| v as isize);
    s.next_number_callback(|| xy.next()).unwrap().unwrap()
}

fn part1(s0: &State) {
    let mut count: isize = 0;
    for y in 0..50 {
        for x in 0..50 {
            print!("{}", if probe(&s0, x, y)==1 {
                count += 1;
                "#"
            } else {
                "."
            });
        }
        println!("")
    }
    println!("Part 1: {}", count);
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");
    let s0 = State::from(&input);

    part1(&s0);

    let mut x_bot_left = 0isize;
    for y_bot_left in 99.. {
        while probe(&s0, x_bot_left, y_bot_left)==0 {
            x_bot_left+=1;
        }
        let x_top_r = x_bot_left+99;
        let y_top_r = y_bot_left-99;
        let res_top_r = probe(&s0,x_top_r, y_top_r);
        println!("Candidate BL: ({}, {}), TR: ({}, {}) -> {}", x_bot_left, y_bot_left, x_top_r, y_top_r, res_top_r);
        if res_top_r>0 {
            let res = x_bot_left*10000+y_top_r;
            println!("Candiate viable: TL.x*10000 + TL.y: {}", res);
            break;
        }
    }
}