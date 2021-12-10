use anyhow::{Result, Context};
use itertools::Itertools;
use std::fs;

fn check_line(ln: &str) -> Option<usize> {
    let mut stack: Vec<char> = Vec::new();
    for c in ln.chars() {
        match c {
            '(' => {stack.push(')');},
            '[' => {stack.push(']');},
            '{' => {stack.push('}');},
            '<' => {stack.push('>');},
            _ => {
                if stack.pop().and_then(|cs| Some(cs!=c)).unwrap_or(false) {
                    return Some(match c {
                        ')' => 3, 
                        ']' => 57, 
                        '}' => 1197, 
                        '>' => 25137,
                        _ => panic!(), 
                    })
                };
            }
        }
    };
    None
}

fn complete_line(ln: &str) -> Option<usize> {
    let mut stack: Vec<char> = Vec::new();
    for c in ln.chars() {
        match c {
            '(' => {stack.push(')');},
            '[' => {stack.push(']');},
            '{' => {stack.push('}');},
            '<' => {stack.push('>');},
            _ => {
                if stack.pop().and_then(|cs| Some(cs!=c)).unwrap_or(false) {
                    return None
                };
            }
        }
    };
    let v = stack.iter().rev()
    .fold(0usize, |a, c| a*5+match c {')' => 1, ']' => 2, '}' => 3, '>' => 4, _ => 0});
    Some(v)
}


fn solution(input_s: &str) -> Result<()> {
    let input: Vec<&str> = input_s
        .trim().split("\n").collect();

    let p1: usize = input.iter().cloned().filter_map(check_line).sum();
    println!("Part 1: {}", p1);
    let p2: Vec<usize> = input.iter().cloned().filter_map(complete_line).sorted().collect();
    println!("Part 2: {}", p2[(p2.len()-1)/2]);
    Ok(())
}

fn main() -> Result<()> {
    println!("** TEST **");
    solution(&fs::read_to_string("test01.txt")?)?;
    println!("\n** INPUT **");
    solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
