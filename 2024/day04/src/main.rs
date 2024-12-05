#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use vecmath::{vec2_add, vec2_scale};
use std::{collections::{HashMap, HashSet}, fs, time::Instant};
use itertools::Itertools;

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

// mod nm {
//     pub use nom::multi::*;
//     pub use nom::sequence::*;
//     pub use nom::character::complete::*;
//     pub use nom::bytes::complete::*;
//     pub use nom::error::*;
//     pub use nom::combinator::*;
//     pub use nom::IResult;

//     /// Ignore leading and trailing whitespace around `inner`
//     pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
//         inner: F,
//     ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
//     where
//         F: Fn(&'a str) -> IResult<&'a str, O, E>,
//     {
//         delimited(multispace0, inner, multispace0)
//     }
// }
// fn parse(s: &str) -> nm::IResult<&str, (Vec<WF>, Vec<Material>)> {
//     nm::separated_pair(
//         parse_wf, 
//         nm::tag("\n\n"), 
//         parse_material
//     )(s)
// }
// let (rest, (wf, materials)) = parse(input_s).map_err(|e| e.to_owned())?;

type V=[i16;2];

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: HashMap<V,u8> = input_s.trim_end().split("\n").enumerate()
        .flat_map(|(y, s)| s.as_bytes().iter().enumerate().map(move |(x, c)| ([x as i16,y as i16], *c)))
        .collect();

    let dirs: [V;8]= [[1,0],[1,-1],[0,-1],[-1,-1],[-1,0],[-1,1],[0,1],[1,1]]; 
    let part1:usize = input.keys().cloned()
    .map(|p0| dirs.iter().cloned()
        .filter(|d| 
            b"XMAS".iter().enumerate()
            .map(|(s, c)| input.get(&vec2_add(p0, vec2_scale(*d, s as i16))).and_then(|mc| Some(mc==c)))
            .all(|cc| cc==Some(true))
        ).count()).sum();
    
    let dirs: [V;2]= [[1,-1],[1,1]]; 
    let ms: HashSet<u8> = b"MS".into_iter().cloned().collect();
    let part2:usize = input.iter()
    .filter(|(_,&c)| c==b'A')
    .filter(|(&p0,_)| dirs.iter().cloned()
        .all(|d| 
            ms == [-1,1].into_iter().filter_map(|s| input.get(&vec2_add(p0, vec2_scale(d, s as i16)))).cloned().collect::<HashSet<u8>>()
        )).count();

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test01.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "18");
    assert_eq!(res[1], "9");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    let start = Instant::now();
    let (res, time) = loop { // run warmup for 100ms
        let lap = Instant::now();
        let res = solution(&input)?;
        if start.elapsed().as_millis()>100 {break (res, lap.elapsed())};
    };
    println!( "({} us)\nPart 1: {}\nPart 2: {}", time.as_micros(), res[0], res[1]);
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
