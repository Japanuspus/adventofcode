extern crate nom;

use nom::character::complete::{digit1};
use nom::bytes::complete::{tag};
use nom::{
    combinator::map_res,
    sequence::tuple};
use std::collections::HashSet;

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
pub struct Interval {
    skip: u32,
    size: u32
}

impl Interval {
    pub fn includes(&self, v:u32) -> bool {
        v > self.skip && v <= self.index_max()
    }

    pub fn index_max(&self) -> u32 {
        self.skip + self.size
    }
}

#[derive(Debug)]
pub struct Claim {
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

pub fn in_claims(claims: &[Claim], i_row: u32, i_col: u32) -> usize {
    claims.iter()
    .filter(|c| c.rows.includes(i_row) && c.cols.includes(i_col))
    .count()
}

pub fn part1_01(d: &str) -> i64 {
    let claims : Vec<Claim> = d.lines().map(parse_claim_all).collect();
    let row_max = claims.iter().map(|c| c.rows.index_max()).max().unwrap();
    let col_max = claims.iter().map(|c| c.cols.index_max()).max().unwrap();
    let mut disputed = 0;

    for i_row in 0..row_max+1 {
        for i_col in 0..col_max+1 {
            if in_claims(&claims, i_row, i_col) > 1 {
                disputed += 1;
            }
        }
    }
    disputed
}

pub fn part2_01(d: &str) -> i64 {
    let claims : Vec<Claim> = d.lines().map(parse_claim_all).collect();
    let row_max = claims.iter().map(|c| c.rows.index_max()).max().unwrap();
    let col_max = claims.iter().map(|c| c.cols.index_max()).max().unwrap();

    let mut has_clash: HashSet::<u32> = HashSet::with_capacity(claims.len()); 
    let all_claims: HashSet::<_> = claims.iter().map(|c| c.id).collect();

    for i_row in 0..row_max+1 {
        for i_col in 0..col_max+1 {
            let cs: Vec::<_> = claims
                    .iter()
                    .filter(|c| c.rows.includes(i_row) && c.cols.includes(i_col))
                    .collect();
            if cs.len()>1 {
                for c in cs {
                    has_clash.insert(c.id);
                }
            }
        }
    }

    let undisputed: Vec::<_> = all_claims.difference(&has_clash).clone().collect();
    println!("Undisputed: {:?}", undisputed);
    assert_eq!( undisputed.len(), 1 );
    *undisputed[0] as i64
}

pub fn run(data: &str) {
    println!("Part 1: {}", part1_01(&data));
    println!("Part 2: {}", part2_01(&data));
}
