extern crate nom;

use nom::character::complete::{digit1};
use nom::bytes::complete::{tag};
use nom::{
    combinator::map_res,
    sequence::tuple};


#[cfg(test)]
mod tests {
    use super::*;
    const TT: &str = "";
    #[test]
    fn part1() {
        assert_eq!(part1_01(TT), 0);
    }
}

#[derive(Debug)]
struct Interval {
    skip: u32,
    size: u32
}

impl Interval {
    pub fn includes(&self, v:u32) -> bool {
        v > self.skip && v <= self.skip + self.size
    }
}

#[derive(Debug)]
struct Claim {
    id: u32, 
    rows: Interval,
    cols: Interval,
}

fn number(input: &str) -> nom::IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

/// A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle 3 inches from the left edge, 2 inches from the top edge, 5 inches wide, and 4 inches tall. Visually, it claims the square inches of fabric represented by # (and ignores the square inches of fabric represented by .) in the diagram below:
/// ```
/// assert_eq!(
///     parse_claim_all("#123 @ 3,2: 5x4"), 
///     Claim{id: 123, rows: Interval{skip: 1, size: 2}, cols: Interval{skip: 3, size: 4}});
/// ```
fn parse_claim_all(input: &str) -> Claim {
    let (_, (_, id, _, x, _, y, _, w, _, h)) = tuple((
        tag("#"), number,
        tag(" @ "), number, tag(","), number,
        tag(": "), number, tag("x"), number
    ))(input).unwrap();
   
    Claim{id, rows: Interval{skip: y, size: h}, cols: Interval{skip: x, size: w}}
}


pub fn part1_01(d: &str) -> i64 {
    let claims : Vec<Claim> = d.lines().map(parse_claim_all).collect();
    claims.len() as i64
}

pub fn part2_01(_d: &str) -> i64 {
    0
}

pub fn run(data: &str) {
    println!("Part 1: {}", part1_01(&data));
    println!("Part 2: {}", part2_01(&data));
}
