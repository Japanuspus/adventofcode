use std::fs;
use anyhow::{Result, Error, Context};
// use itertools::Itertools;
use parse_display::{FromStr};
// use regex::Regex;
// use apply::Also;

const PIOVER180:f32 = 0.01745329251;

#[derive(Debug)] //, FromStr)]
//#[display("{cmd}{arg}")]
struct InputItem {cmd: char, arg: isize}
fn parse_input_item(s: &str) -> Result<InputItem> {
    let mut ch = s.chars();
    let cmd = ch.next().ok_or(Error::msg("No cmd"))?;
    let arg = ch.collect::<String>().parse::<isize>()?;
    Ok(InputItem{cmd, arg})
}

fn main() -> Result<()> {
    let input: Vec<InputItem> = fs::read_to_string("input.txt")?
    .lines()
    .map(|ln| parse_input_item(ln).context(format!("While parsing line '{}'", ln)))
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
                'S' => s.y-=c.arg, // means to move south by the given value.
                'E' => s.x+=c.arg, // means to move east by the given value.
                'W' => s.x-=c.arg, // means to move west by the given value.
                'L' => s.heading+=c.arg, // means to turn left the given number of degrees.
                'R' => s.heading-=c.arg, // means to turn right the given number of degrees.
                'F' => {                 // means to move forward by the given value in the direction the ship is currently facing.
                    s.x+=((c.arg as f32)*((s.heading as f32)*PIOVER180).cos()) as isize;
                    s.y+=((c.arg as f32)*((s.heading as f32)*PIOVER180).sin()) as isize;
                }, 
                _ => panic!("Unexpected cmd")
            };
            s
        }
    );

    println!("Part 1: {}", s1.x.abs()+s1.y.abs());

    Ok(())
}
