#![allow(unused)]

use day11::State;
use num_traits::cast::ToPrimitive;
use num_bigint::BigInt;

fn oo(a: std::result::Result<std::option::Option<BigInt>, ()>)-> isize {
    a.unwrap().unwrap().to_isize().unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
    .expect("Error reading input file");

    let mut s = State::from(input);
    let inputs: Vec<isize> = vec![0];
    let color = oo(s.next_output(&inputs));
    let turn = oo(s.next_output(&inputs));
    println!("Color: {}, turn: {}", color, turn);
}