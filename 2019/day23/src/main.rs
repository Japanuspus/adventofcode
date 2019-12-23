#![allow(unused)]

// use std::collections::HashSet;
// use std::collections::HashMap;
use std::collections::VecDeque;
// use std::iter;
use day11::State; // dep: day11 = {path="../day11"}

struct Machine {
    state: State,
    inputs: VecDeque<isize>,
}

impl Machine {
    fn step(&mut self) -> Option<Vec<isize>> {
        let mut state = &mut self.state;
        let mut inputs = &mut self.inputs;
        let packet = state.next_numbers(3, || inputs.pop_front().or(Some(-1)));
        packet.unwrap()
    }
}

fn main_loop(machines: &mut Vec<Machine>) -> isize {
    loop {
        for idx in 0..machines.len() {
            if let Some(packet) = machines[idx].step() {
                let addr = packet[0];
                let x = packet[1];
                let y = packet[2];
                println!("Packet {} --> {}: ({}, {})", idx, addr, x, y);
                if addr == 255 {return y}
                machines[addr as usize].inputs.push_back(x);
                machines[addr as usize].inputs.push_back(y);
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");
    
    let n_machine = 50;
    let mut machines: Vec<_> = (0..n_machine).map(|i|
        Machine{
            state:  State::from(&input),
            inputs: {let mut v=VecDeque::new(); v.push_back(i); v}
        })
        .collect();
    println!("Part 1: {}", main_loop(&mut machines));
}