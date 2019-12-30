#![allow(unused)]

use std::collections::HashMap;
use std::collections::VecDeque;
// use std::iter;
use day11::State;

#[derive(Debug)]
struct Prompt {
    room: String,
    doors: Vec<String>,
    items: Vec<String>,
    hl: isize
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
                section_items.push(line[2..].to_string());
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
                hl = 1;
            }
            if line.starts_with("A loud, robotic voice says \"Alert! Droids on this ship are lighter") {
                hl = -1;
            }
        }
    }
    let doors = sections.remove("Doors here lead").unwrap_or_else(|| Vec::new());
    let items = sections.remove("Items here").unwrap_or_else(|| Vec::new());

    if let Some(room)=room {
        Some(Prompt{room: room.to_string(), doors, items, hl})
    } else {
        None
    }
}

struct Player {
    prompt: String,
    parsed: Prompt,
    
}

impl Player {

    fn respond(&mut self, prompt: String) -> Vec<u8> {
        println!("{}", &prompt);
        if let Some(parsed) = parse_prompt(&prompt) {
            self.prompt = prompt;
            self.parsed = parsed;
        }
        println!("{:?}", self.parsed);
        println!("Command (north, south, east, or west, take, drop, inv)");
        let mut input = String::new();
        let input_count = std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut resp = String::new();
        if input.len()<3 {
            let append = match input.chars().next() {
                Some('n') => "north",
                Some('s') => "south",
                Some('e') => "east",
                Some('w') => "west",
                Some('i') => "inv",
                Some('t') => {
                    resp = format!("take {}", self.parsed.items.get(0).unwrap_or(&"x".to_string())).to_string();
                    ""
                }
                _ => "x"
            };
            resp.extend(append.chars());
        } else {
            resp = input.trim().to_string();
        }
        resp+="\n";
        println!("> {}", &resp);
        resp.into_bytes().into_iter().collect()
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");

    let mut s = State::from(&input);
    let mut input_buffer = VecDeque::new();
    let mut output_buffer = Vec::new();

    let mut player = Player{
        prompt: "".to_string(), 
        parsed: Prompt{room: "None".to_string(), items: Vec::new(), doors: Vec::new(), hl: 0}
    };

    while let Some(o) = s.next_number_callback(|| {
        if input_buffer.len()==0 {
            let prompt: String = output_buffer.drain(..).collect();
            let resp = player.respond(prompt);
            //let resp = poll_joystick(prompt);
            input_buffer.extend(resp.into_iter());
         }
         input_buffer.pop_front().map(|v| v as isize)
    }).unwrap() {
        output_buffer.push((o as u8) as char);
    }
}