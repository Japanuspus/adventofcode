#![allow(unused)]

// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::iter;
use day11::State; // dep: day11={path="../day11"}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");

    // Part 1
    let mut s = State::from(&input);
    let codez: Vec<_> = 
    "NOT J J
    AND A J
    AND B J
    AND C J
    NOT J J
    AND D J
    WALK
    ".as_bytes().iter().map(|b| *b as isize).collect();
    let codez: Vec<_> = 
    "NOT E J
    NOT H T
    AND T J
    NOT J J
    NOT A T
    NOT T T
    AND B T
    AND C T
    NOT T T
    AND T J
    AND D J
    RUN
    ".as_bytes().iter().map(|b| *b as isize).collect();

    let mut codeiter = codez.into_iter();
    let mut last_o = 0;
    while let Some(o) = s.next_number_callback(|| codeiter.next()).unwrap() {
        last_o = o;
        print!("{}", (o as u8) as char)
    }
    println!("Part 1: {}", last_o);
}