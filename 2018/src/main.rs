use std::env;
use std::fs;

extern crate aoc2018;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: Specify day number");
        return;
    }

    let day_name = &args[1];

    let days = aoc2018::days();

    let dd = days.get(day_name).expect("Unknown day name");
    let dayfile = format!("inputs/{}.txt", day_name);
    println!("Attempting to read: {}", dayfile);
    let data = fs::read_to_string(dayfile).expect("Error reading file");

    println!("Running advent of code for day {}", day_name);
    (dd.runner)(&data);
}
