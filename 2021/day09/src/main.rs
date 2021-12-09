use anyhow::{Result, Context};
use std::fs;
use std::collections::HashMap;

// use parse_display::{Display, FromStr};

// #[derive(Display, FromStr, PartialEq, Debug)]
// enum Direction {
//     #[display("forward")]
//     Forward,
// }

// #[derive(Debug, Display, FromStr)]
// #[display("{direction} {distance}")]
// struct Step {
//     direction: Direction,
//     distance: i32,
// }

fn solution(input_s: &str) -> Result<()> {
    let input: Vec<Vec<i8>> = input_s
        .trim()
        .split("\n")
        .map(|s| s.as_bytes().iter().map(|c| (*c-b'0') as i8).collect())
        .collect();

    let map: HashMap<(i32, i32), i8> = input.iter().enumerate()
    .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, v)| ((i as i32, j as i32), *v)))
    .collect();

    let dirs: [(i32, i32);4] = [(-1,0), (0, 1), (1, 0), (0, -1)];

    let p1: usize = map.iter()
    .filter(|(loc, val)| {
        dirs
        .iter()
        .filter_map(|d| {
            map
            .get(&(d.0+loc.0, d.1+loc.1))
            .and_then(|val_n| Some(val_n>val))
        })
        .all(|b| b)
    })
    .map(|(loc, val)| {
        //println!("{:?}", loc);
        1+(*val as usize)
    })
    .sum();


    println!("Part 1: {}", p1);
    println!("Part 2: {}", 0);
    Ok(())
}

// #[test]
// fn test_solution() -> Result<()> {
//     let (a, b) = solution(&fs::read_to_string("test00.txt")?)?;
//     assert!(a==1);
//     assert!(b==0);
//     Ok(())
// }

fn main() -> Result<()> {
    solution(&fs::read_to_string("test00.txt")?)?;
    solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
