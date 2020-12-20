use anyhow::Context;
use nom::multi::many1;
use nom::character::complete::one_of;
use nom::Finish;
use nom::IResult;
use nom::character::complete::digit1;
use nom::combinator::{map_res, map};
use nom::combinator::recognize;
use nom::sequence::separated_pair;
use nom::bytes::complete::tag;
use nom::sequence::delimited;
use std::fs;
use anyhow::Result;
use nom::{multi::separated_list1, sequence::preceded};
// use anyhow::Context;
// use itertools::Itertools;
// use apply::Also;
// use num::{BigInt, Integer};


#[derive(Debug)]
struct RawTile {
    id: usize,
    pixels: Vec<Vec<bool>>
}

fn parse_number(input : &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}
fn parse_pixel_line(input: &str) -> IResult<&str, Vec<bool>> {
    map(
        recognize(many1(one_of("#."))), 
        |s: &str| s.chars().map(|c| c=='#').collect()
    )(input)
}
fn parse_raw_tile(s: &str) -> Result<Vec<RawTile>> {
    let id_line = delimited(tag("Tile "), parse_number, tag(":"));
    let pixels = separated_list1(tag("\n"), parse_pixel_line);
    let mut field = map(
        separated_pair(id_line, tag("\n"), pixels),  
        |(id, pixels)| RawTile{id, pixels}
    );
    let mut tiles = separated_list1(tag("\n\n"), field);

    let res = tiles(s)
    .finish()
    .map(|(_, r)| r)
    // https://github.com/Geal/nom/blob/master/doc/nom_recipes.md#implementing-fromstr
    .map_err(|nom::error::Error { input, code }| nom::error::Error{input: input.to_string(), code});
    Ok(res?)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let raw_tiles: Vec<RawTile> = parse_raw_tile(&input[..])?;
    println!("Part 1: {}", raw_tiles.len());

    Ok(())
}
