#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use std::{fs, time::Instant, collections::{BTreeMap, BTreeSet}};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug)]
#[display("Sensor at {sensor}: closest beacon is at {beacon}")]
struct Row {
    sensor: Position,
    beacon: Position,
}

#[derive(Debug, Display, FromStr)]
#[display("x={x}, y={y}")]
struct Position {
    x: i32,
    y: i32,
}

fn solution(input_s: &str, y0: i32) -> Result<[String; 2]> {
    let input: Vec<Row> = input_s.trim_end()
        .split("\n")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_, _>>()?;

    // // map start of intervals to ends. both inclusive, i.e. 0,0 means an interval with only 0
    // let mut starts: BTreeMap::<i32, i32> = input.iter().filter_map(|r| {
    //     let range = r.beacon.x.abs_diff(r.sensor.x) + r.beacon.y.abs_diff(r.sensor.y);
    //     let dy = r.sensor.y.abs_diff(y0);
    //     range.checked_sub(dy).and_then(|row_range| Some((r.sensor.x-row_range as i32, r.sensor.x+row_range as i32)))
    // }).collect();
    // let mut ends: BTreeSet::<i32> = BTreeSet::new();
    
    // let mut x=*(starts.first_entry().unwrap().get());
    
    // first_key_value().unwrap();
    // loop {
    //     if starts.first_entry().and_then(|&v| v==x).unwrap_or(false) {}
    // }

    // intervals -> [start, 1], [end,-1]
    let mut edges: BTreeSet::<[i32;2]> = BTreeSet::new();
    for r in &input {
        let range = r.beacon.x.abs_diff(r.sensor.x) + r.beacon.y.abs_diff(r.sensor.y);
        let dy = r.sensor.y.abs_diff(y0);
        if let Some(row_range) = range.checked_sub(dy) {
            edges.insert([r.sensor.x-row_range as i32, 1]);
            edges.insert([r.sensor.x+row_range as i32, -1]);
        }
    };
    let edges = edges;

    let mut active = 0;
    let mut x0 = 0;
    let mut cover = 0;
    for [x,v] in edges.iter() {
        if active>0 {cover+=x-x0;}
        active+=v;
        x0=*x;
    }
    let part1 = cover;
    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input, 10)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "26");
    assert!(res[1] == "0");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    for _ in 0..20 {solution(&input, 2000000)?;} //warmup
    let start = Instant::now();
    let res = solution(&input, 2000000)?;
    println!(
        "({} us)\nPart 1: {}\nPart 2: {}",
        start.elapsed().as_micros(), res[0], res[1],
    );
    Ok(())
}


// // Make it simple to compare timing for multiple solutions
// type Solution = dyn Fn(&str) -> Result<[String; 2]>;
// const SOLUTIONS: [(&str, &Solution); 1] = [("Original", &solution)];

// #[test]
// fn test_solution() -> Result<()> {
//     let input = &fs::read_to_string("test00.txt")?;
//     for (name, solution) in SOLUTIONS {
//         let res = solution(&input).with_context(|| format!("Running solution {}", name))?;
//         println!("---\n{}\nPart 1: {}\nPart 2: {}", name, res[0], res[1]);
//         assert!(res[0] == "0");
//         assert!(res[1] == "0");
//     }
//     Ok(())
// }

// fn main() -> Result<()> {
//     let input = &fs::read_to_string("input.txt")?;
//     for (_, solution) in SOLUTIONS.iter().cycle().take(10) {
//         solution(&input)?;
//     } //warmup
//     for (name, solution) in SOLUTIONS {
//         let start = Instant::now();
//         let res = solution(&input)?;
//         println!(
//             "---\n{} ({} us)\nPart 1: {}\nPart 2: {}",
//             name, start.elapsed().as_micros(), res[0], res[1],
//         );
//     }
//     Ok(())
// }
