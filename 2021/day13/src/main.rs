use anyhow::{Result, Context};
use std::{fs, collections::HashSet};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display(style="lowercase")]
enum Axis {
    X,
    Y,
}

 #[derive(Debug, Display, FromStr)]
 #[display("fold along {axis}={at}")]
 struct Fold {
     axis: Axis,
     at: i32,
 }

fn apply_fold(pts: &HashSet<(i32, i32)>, fold: &Fold) -> HashSet<(i32, i32)> {
    pts.iter().map(|(x, y)| match fold.axis {
        Axis::X => (if *x>fold.at {2*fold.at - x} else {*x}, *y),
        Axis::Y => (*x, if *y>fold.at {2*fold.at - y} else {*y}),
    })
    .collect()
}

fn solution(input_s: &str) -> Result<()> {
    let mut input = input_s.trim().split("\n\n");
    let pts0: Vec<(i32, i32)> = input.next().unwrap().split("\n")
        .map(|s| 
            s.split(",")
            .map(|n| n.parse::<i32>())
            .collect::<Result<Vec<_>,_>>()
            .and_then(|v| Ok((v[0], v[1])))
        )
        .collect::<Result<_,_>>()?;
    
    let folds: Vec<Fold> = input.next().unwrap().split("\n")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_,_>>()?;

    // Part 1
    let pts: HashSet<(i32, i32)> = pts0.iter().cloned().collect();
    let p1 = apply_fold(&pts, &folds[0]).len();
    println!("Part 1: {}", p1);

    // Part 2
    let pts = folds.iter().fold(pts, |acc, fold| apply_fold(&acc, fold));
    println!("Part 2:");
    let max_x = *pts.iter().map(|(x, _y)| x).max().unwrap();
    let max_y = *pts.iter().map(|(_x, y)| y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            print!("{}", if pts.contains(&(x, y)) {"\u{2588}"} else {" "});
        }
        println!();
    }
    Ok(())
}

fn main() -> Result<()> {
    println!("** TEST **");
    solution(&fs::read_to_string("test01.txt")?)?;
    println!("\n** INPUT **");
    solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
