extern crate aoc2018;
use aoc2018::day99;
// use aoc2018::utils::Response;


fn main() {
    // let mut t: Vec<Response> = Vec::new();

    let data = include_str!("../inputs/day99.txt").trim_right();
    day99::run(&data);


    let days = aoc2018::days();
    let dd = days.get("day99").unwrap();
    (dd.runner)(data);
}
