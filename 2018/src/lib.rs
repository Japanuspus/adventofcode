use std::collections::HashMap;

pub mod utils;
pub mod day99;
pub mod day01;
static DAYRUNS: [(&str, fn(&str));2] = [
    ("day99", day99::run),
    ("day01", day01::run),
];

// TODO: use &'static str for name
pub struct Day {
    pub name: String,
    pub runner: fn(& str),
}

//    Day {name: "day99", runner: day99::run},
pub fn days() -> HashMap<String,Day> {
        DAYRUNS
        .iter()
        .map(|(n, r)| (String::from(*n), Day {name: String::from(*n), runner: *r}))
        .collect()
}
