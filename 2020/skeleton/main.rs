use std::fs;
use anyhow::Result;
// use itertools::Itertools;
// use parse_display::{FromStr};
// use regex::Regex;
// use apply::Also;

//#[derive(Debug, FromStr)]
//#[display("{key}:{value}")]
//struct InputItem {key: String, value: String}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
 
    println!("Part 1: {}", input.len());

    Ok(())
}
