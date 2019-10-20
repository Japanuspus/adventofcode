#[macro_use]
extern crate nom;

use std::collections::HashMap;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;
pub mod day99;

pub mod utils;
static DAYRUNS: [(&str, fn(&str)); 26] = [
    ("day99", day99::run),
    ("day01", day01::run),
    ("day02", day02::run),
    ("day03", day03::run),
    ("day04", day04::run),
    ("day05", day05::run),
    ("day06", day06::run),
    ("day07", day07::run),
    ("day08", day08::run),
    ("day09", day09::run),
    ("day10", day10::run),
    ("day11", day11::run),
    ("day12", day12::run),
    ("day13", day13::run),
    ("day14", day14::run),
    ("day15", day15::run),
    ("day16", day16::run),
    ("day17", day17::run),
    ("day18", day18::run),
    ("day19", day19::run),
    ("day20", day20::run),
    ("day21", day21::run),
    ("day22", day22::run),
    ("day23", day23::run),
    ("day24", day24::run),
    ("day25", day25::run),
];

// TODO: use &'static str for name
pub struct Day {
    pub name: String,
    pub runner: fn(&str),
}

//    Day {name: "day99", runner: day99::run},
pub fn days() -> HashMap<String, Day> {
    DAYRUNS
        .iter()
        .map(|(n, r)| {
            (
                String::from(*n),
                Day {
                    name: String::from(*n),
                    runner: *r,
                },
            )
        })
        .collect()
}
