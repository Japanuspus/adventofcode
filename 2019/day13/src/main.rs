#![allow(unused)]

use day11::State;
use day13::Canvas;
use std::fmt;

#[derive(Debug)]
struct Machine {
    canvas: Canvas, 
    score: isize
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Score: {}", self.score)?;
        write!(f, "{}", self.canvas)
    }
}

impl Machine {
    fn update1(&mut self, p: &Vec<isize>) {
        if p[0]<0 {
            self.score = p[2];
        } else {
            self.canvas.set(p[0], p[1], p[2]);
        }
    }
    fn update(&mut self, o: &Vec<Vec<isize>>) {
        for p in o {
            self.update1(p)
        }
    }
}

fn poll_joystick() -> isize {
    println!("a: left, d: right, empty: stay");
    let mut line = String::new();
    let input = std::io::stdin().read_line(&mut line).expect("Failed to read line");
    let v = match line.chars().next() {
        Some('a') => {-1}
        Some('d') => {1}
        _ => {0}
    };
    println!("Line {} -> v: {}", line, &v);
    v
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");
    let mut s = State::from(&input);

    // part 1
    let screen0: Vec<_> = (0..)
        .map(|_| s.next_numbers(3, || None).unwrap())
        .take_while(|r| r.is_some())
        .map(|r| r.unwrap())
        .collect();
    println!("Part 1: {}", screen0.iter().filter(|c| c[2]==2).count());

    // Part 2
    let mut m = Machine{
        canvas: Canvas::for_points(
            screen0.iter().filter_map(|c| if c[0]>=0 {Some((c[0], c[1]))} else {None}),
            " #+-o   "),
        score: 0};
    let mut s = State::from(&input);
    s.poke(0, 2); // insert coin
    loop {
        if let Some(p) = s.next_numbers(3, || {
                println!("{}", m);
                Some(poll_joystick())
            }).unwrap() {
            m.update1(&p);
        }
    }
}
