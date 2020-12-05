#![allow(unused)]

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::collections::{HashMap, BTreeSet, VecDeque};
// use std::iter;
use day11::State;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Prompt {
    room: Rc<String>,
    doors: Vec<Rc<String>>,
    items: Vec<Rc<String>>,
    hl: isize
}

impl Prompt {
    fn new() -> Self {
        Self{
            room: Rc::new("".to_string()),
            doors: Vec::new(),
            items: Vec::new(),
            hl: 0
        }
    }
}

fn parse_prompt(prompt: &str) -> Option<Prompt> {
    let mut lines = prompt.lines();
    let mut room = None;
    let mut sections = HashMap::new();
    let mut section_name = None;
    let mut section_items = Vec::new();
    let mut hl: isize = 0;

    while let Some(line) = lines.next() {
        if line.starts_with("==") {
            room = Some(&line[3..(line.len()-3)]);
            continue
        }
        if section_name.is_some() {
            if line.starts_with("- ") {
                section_items.push(Rc::new(line[2..].to_string()));
            } else {
                sections.insert(section_name.unwrap(), section_items);
                section_name = None;
                section_items = Vec::new();
            }
        } else {
            if line.ends_with(":") {
                section_name = Some(line[0..(line.len()-1)].to_string());
            }
            if line.starts_with("A loud, robotic voice says \"Alert! Droids on this ship are heavier") {
                hl = -1; // we are under weight
            }
            if line.starts_with("A loud, robotic voice says \"Alert! Droids on this ship are lighter") {
                hl = 1;
            }
        }
    }
    let doors = sections.remove("Doors here lead").unwrap_or_else(|| Vec::new());
    let items = sections.remove("Items here").unwrap_or_else(|| Vec::new());

    if let Some(room)=room {
        Some(Prompt{room: Rc::new(room.to_string()), doors, items, hl})
    } else {
        None
    }
}

fn log_line(line: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("command_log.txt")
        .unwrap();
    writeln!(file, "{}", line.trim()).expect("Error writing log!");
}

struct Player {
    prompt: String,
    parsed: Prompt,
    known_items: BTreeSet<Rc<String>>,
    known_overweight: Vec<BTreeSet<Rc<String>>>,
    known_underweight: Vec<BTreeSet<Rc<String>>>,
    inventory: BTreeSet<Rc<String>>,
    attempt_pending: bool,    
}

impl Player {

    fn new() -> Self {
        Self{
            prompt: "".to_string(), parsed: Prompt::new(), 
            known_items: BTreeSet::new(), 
            inventory: BTreeSet::new(), 
            known_overweight: Vec::new(), 
            known_underweight: Vec::new(),
            attempt_pending: false
        }
    }

    fn make_inventory_orders(&self, goal_inv: &BTreeSet<Rc<String>>) -> String {
        self.inventory.difference(goal_inv)
        .map(|item| format!("drop {}\n", item))
        .chain(
            goal_inv.difference(&self.inventory)
            .map(|item| format!("take {}\n", item))
        )
        .collect()
    }

    fn dump_knowledge(&self) {
        println!("Known items: {:?}", self.known_items);
        println!("Known overweight:");
        for i in self.known_overweight.iter() {println!(" - {:?}", i); }
        println!("Known underweight:");
        for i in self.known_underweight.iter() {println!(" - {:?}", i); }
    }

    fn propose_attempt(&mut self) -> String {
        self.dump_knowledge();
        let v: Vec<_> = self.known_items.iter().cloned().collect();
        let mut expanded = self.known_underweight.iter().flat_map(|uw| {
            self.known_items.difference(uw).map(move |i| {let mut r=uw.clone(); r.insert(i.clone()); r})
        });
        let mut powerset = (0..2usize.pow(v.len() as u32)).map(|i| {
            v.iter().enumerate()
            .filter(|&(t, _)| (i >> t) % 2 == 1)
            .map(|(_, element)| element.clone())
            .collect::<BTreeSet<_>>()
        });
        let mut valid_candidates = 
        expanded
        .chain(powerset)
        .filter(|ss| self.known_overweight.iter().filter(|ko| ko.is_subset(ss)).count()==0)
        .filter(|ss| self.known_underweight.iter().filter(|ku| ku.is_superset(ss)).count()==0);
        if let Some(ss) = valid_candidates.next() {
            println!("Target inventory: {:?}", &ss);
            let resp = self.make_inventory_orders(&ss)+"south\n";
            self.inventory = ss;
            self.attempt_pending = true;
            resp
        } else {
            println!("Unable to find unexplored weight!!");
            "".to_string()
        }
    }

    fn register_take(&mut self, item: Rc<String>) {
        self.known_items.insert(item.clone());
        self.inventory.insert(item);
    }

    fn respond(&mut self, prompt: String) -> Vec<u8> {
        println!("{}", &prompt);
        if let Some(parsed) = parse_prompt(&prompt) {
            self.prompt = prompt;
            self.parsed = parsed;
        }
        println!("Parsed:\n{:?}", self.parsed);
        
        if self.attempt_pending && self.parsed.hl!=0 {
            self.attempt_pending = false;
            if self.parsed.hl == 1 {
                println!(">> Registering overweight attempt for inv={:?}", self.inventory);
                let new_set = self.inventory.clone();
                self.known_overweight = self.known_overweight.drain(..).filter(|uv| !uv.is_superset(&new_set)).collect();
                self.known_overweight.push(new_set);
            }
            if self.parsed.hl == -1 {
                println!(">> Registering underweight attempt for inv={:?}", self.inventory);
                let new_set = self.inventory.clone();
                self.known_underweight = self.known_underweight.drain(..).filter(|uv| !uv.is_subset(&new_set)).collect();
                self.known_underweight.push(new_set);
            }
            self.dump_knowledge();
        }

        let mut resp = String::new();
        loop {
            println!("Command (north, south, east, or west, take, drop, inv, x: SC bot, r: run commands.txt)");
            let mut input = String::new();
            let input_count = std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
            if input.len()<3 {
                let append = match input.chars().next() {
                    Some('n') => "north",
                    Some('s') => "south",
                    Some('e') => "east",
                    Some('w') => "west",
                    Some('i') => "inv",
                    Some('r') => {
                        println!("Running commands.txt");
                        resp = std::fs::read_to_string("commands.txt")
                            .expect("Error reading commands.txt");
                        for ln in resp.lines().filter(|s| s.starts_with("take ")) {
                            self.register_take(Rc::new(ln[5..].to_string()));
                        }
                        ""
                    }
                    Some('t') => {
                        if let Some(item) = self.parsed.items.get(0) {
                            let ic = item.clone();
                            self.register_take(ic.clone());
                            resp = format!("take {}", ic).to_string();
                        }
                        ""
                    }
                    Some('x') => {
                        resp = self.propose_attempt();
                        ""
                    }
                    _ => "x"
                };
                resp.extend(append.chars());
            } else {
                resp = input.trim().to_string();
            }
            if resp.len()>0 {break}
        }

        resp=resp.trim().to_string()+"\n";
        log_line(&resp);
        println!("> {}", &resp);
        resp.into_bytes().into_iter().collect()
    }
}

#[test]
fn test_player() {
    let mut player = Player::new();
    player.known_items.extend(["foo","bar"].iter().map(|s| Rc::new(s.to_string())));
    player.known_underweight.push(
        ["foo"].iter().map(|s| Rc::new(s.to_string())).collect()
    );
    let r = player.propose_attempt();
    assert_eq!(r, "take bar\ntake foo\nsouth\n");
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");

    let mut s = State::from(&input);
    let mut input_buffer = VecDeque::new();
    let mut output_buffer = Vec::new();

    let mut player = Player::new();
    
    log_line("**** Starting");

    while let Some(o) = s.next_number_callback(|| {
        if input_buffer.len()==0 {
            let prompt: String = output_buffer.drain(..).collect();
            let resp = player.respond(prompt);
            input_buffer.extend(resp.into_iter());
         }
         input_buffer.pop_front().map(|v| v as isize)
    }).unwrap() {
        output_buffer.push((o as u8) as char);
    }
    println!("Robot intcode has halted nominally. Final message:\n{}", 
        output_buffer.drain(..).collect::<String>());
}