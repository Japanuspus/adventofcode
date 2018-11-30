use std::collections::HashMap;

pub mod utils;
pub mod day99;
static DAYRUNS: [(&str, fn(&str));1] = [
    ("day99", day99::run)
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
