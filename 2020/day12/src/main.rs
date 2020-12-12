use std::fs;
use anyhow::{Result, Error, Context};
// use itertools::Itertools;
use parse_display::{FromStr};
// use regex::Regex;
// use apply::Also;

#[derive(Debug, FromStr)]
#[display("{cmd}{arg}")]
struct InputItem {cmd: char, arg: isize}


fn main() -> Result<()> {
    println!("{}", "5".parse::<isize>()?);

    let input: Vec<InputItem> = fs::read_to_string("input.txt")?
    .lines()
    .map(|ln| ln.parse().context(format!("While parsing line '{}'", ln)))
    .collect::<Result<_,_>>()?;

    #[derive(Debug)]
    struct State {x: isize, y: isize, heading: isize};
    // heading: trigs angles 90 is north, positive y, 0 is east, positive x
    

    let s1 = input.iter()
    .fold(
        State{x:0, y:0, heading: 0},
        |mut s, c| {
            match c.cmd {
                'N' => s.y+=c.arg, // means to move north by the given value.
                // 'S' => State{x: s.x, y: y-c.arg, heading: heading}, // means to move south by the given value.
                // 'E' => State{x: s.x+c.arg, y: y, heading: heading}, // means to move east by the given value.
                // 'W' => State{x: s.x-c.arg, y: y, heading: heading}, // means to move west by the given value.
                // 'L' => State{x: s.x, y: y, heading: heading+c.arg}, // means to turn left the given number of degrees.
                // 'R' => State{x: s.x, y: y, heading: heading-c.arg}, // means to turn right the given number of degrees.
                // 'F' => State{
                //     x: x+((s.arg as f32)*(s.heading as f32).cos() as isize), 
                //     y: y+((s.arg as f32)*(s.heading as f32).cos() as isize), 
                //     heading: heading
                // }, // means to move forward by the given value in the direction the ship is currently facing.
                _ => panic!("Unexpected cmd")
            };
            s
        }
    );

    println!("Part 1: {}", s1.x.abs()+s1.y.abs());

    Ok(())
}
