#![allow(unused)]

// use std::collections::HashSet;
use std::collections::VecDeque;
// use std::iter;
use day11::State; // dep: day11={path="../day11"}

fn poll_joystick() -> Vec<u8> {
    println!("Command (north, south, east, or west, take, drop, inv)");
    let mut line = String::new();
    let input = std::io::stdin().read_line(&mut line).expect("Failed to read line");
    if line.len()<3 {
        line = match line.chars().next() {
            Some('n') => "north",
            Some('s') => "south",
            Some('e') => "east",
            Some('w') => "west",
            Some('i') => "inv",
            _ => "x"
        }.to_string();
    } else {
        line = line.trim().to_string();
    }
    line+="\n";
    line.into_bytes().into_iter().collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");

    let mut s = State::from(&input);
    let mut cmd = VecDeque::new();

    while let Some(o) = s.next_number_callback(|| {
        if cmd.len()==0 {
            let resp = poll_joystick();
            cmd.extend(resp.into_iter());
         }
         cmd.pop_front().map(|v| v as isize)
    }).unwrap() {
        print!("{}", (o as u8) as char)
    }
}