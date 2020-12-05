#![allow(unused)]

// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::iter;
use day11::State; // dep: day11={path="../day11"}

fn run_springdroid(ascii: &str, codez: &str) -> isize {
    let mut s = State::from(&ascii);
    let trimmed: String = codez
        .lines()
        .filter_map(|ln| ln.split('#').next())
        .map(|ln| ln.trim())
        .filter(|ln| ln.len()>0)
        .collect::<Vec<&str>>()
        .join("\n") + "\n";
    println!("Code trimmed: \n{}<<<", trimmed);
    let mut codeiter = trimmed.as_bytes().iter().map(|b| *b as isize);
    let mut last_o = 0;
    while let Some(o) = s.next_number_callback(|| codeiter.next()).unwrap() {
        last_o = o;
        print!("{}", (o as u8) as char)
    }
    last_o
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");

    // Part 1
    let codez = "
    NOT J J # J <- T
    AND A J
    AND B J
    AND C J
    NOT J J # J <- !(A ^ B ^ C)
    AND D J 
    WALK
    ";
    println!("Part 1: {}", run_springdroid(&input, &codez));
    
    let codez = 
    "NOT E J
    NOT H T
    AND T J
    NOT J J  # J <- !(!E ^ !H) = E v H
    NOT A T
    NOT T T
    AND B T
    AND C T
    NOT T T
    AND T J
    AND D J # J <- D ^ !(A ^ B ^ C) ^ (E v H)
    RUN
    ";
    println!("Part 2: {}", run_springdroid(&input, &codez));
}