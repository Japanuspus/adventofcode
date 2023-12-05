#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use intervaltree::IntervalTree;
use std::{fs, time::Instant};
use nom;
use parse_display::{Display, FromStr};

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

#[derive(Debug, Display, FromStr)]
#[display("{dst} {src} {length}")]
struct MapRange {
    dst: usize,
    src: usize,
    length: usize,
}


#[derive(Debug, Display, FromStr)]
#[display("{from}-to-{to} map:")]
struct MapHead {
    from: String,
    to: String,
}


fn solution(input_s: &str) -> Result<[String; 2]> {
    let mut blocks = input_s
        .trim_end()
        .split("\n\n");

    let seeds: Vec<usize> = blocks
        .next().unwrap().split(" ").skip(1)
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_, _>>()?;

    let maps: Vec<(MapHead, Vec<MapRange>)> = blocks.map(|bs| {
        let mut lns = bs.split('\n');
        let head = lns.next().unwrap().parse().unwrap();
        let ranges = lns.map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_, _>>().unwrap();
        (head, ranges)
    }).collect();

    let maptrees: Vec<IntervalTree<isize, isize>> = maps.iter().map(|(_, ranges)| {
        IntervalTree::from_iter(ranges.iter().map(|r| intervaltree::Element{
            range: (r.src as isize)..(r.src as isize +r.length as isize+1),
            value: r.dst as isize - r.src as isize,
        }))
    }).collect();

    let part1: isize = seeds.iter().map(|seed0| {
        maptrees.iter().fold(*seed0 as isize, |seed, map| {
            //println!("Seed 0: {} -> {}", seed0, seed);
            seed+map.query_point(seed).next().and_then(|el| Some(&el.value)).unwrap_or(&0)
        })
    }).min().unwrap();

    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "35");
    assert_eq!(res[1], "0");
    Ok(())
}

fn main() -> Result<()> {
    //let input = &fs::read_to_string("test00.txt")?;
    let input = &fs::read_to_string("input.txt")?;
    // for _ in 0..20 {
    //     solution(&input)?;
    // } //warmup
    let start = Instant::now();
    let res = solution(&input)?;
    println!(
        "({} us)\nPart 1: {}\nPart 2: {}",
        start.elapsed().as_micros(),
        res[0],
        res[1],
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
//         assert_eq!(res[0], "0");
//         assert_eq!(res[1], "0");
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
