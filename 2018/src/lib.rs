#[macro_use]
extern crate nom;

use std::collections::HashMap;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day99;
pub mod utils;
static DAYRUNS: [(&str, fn(&str)); 5] = [
    ("day99", day99::run),
    ("day01", day01::run),
    ("day02", day02::run),
    ("day03", day03::run),
    ("day04", day04::run),
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
