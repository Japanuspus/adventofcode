#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use nom::{self, error::ErrorKind, IResult};
use std::{fs, time::Instant, collections::HashMap};

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

type Pos = [u16;2];
type Tile = [Pos;4]; //>v<^

fn parse_board(s: &str) -> (Pos, HashMap<Pos, Tile>) {
    // min and max values in each column
    let mut col_min: HashMap<u16, u16> = HashMap::new();
    let mut col_max: HashMap<u16, u16> = HashMap::new();
    for (ym, ln) in s.split('\n').enumerate() {
        let y = ym as u16 + 1;
        for (xm, _) in ln.as_bytes().iter().enumerate().filter(|(_, &c)| c!=b' ') {
            let x = xm as u16 + 1;
            col_min.entry(x).or_insert_with(|| y);
            col_max.insert(x, y);
        }
    };

    let mut m = HashMap::new();
    for (ym, ln) in s.split('\n').enumerate() {
        // inner cells
        let y = ym as u16 + 1;
        let x_max = ln.as_bytes().len() as u16;
        let mut x_left = x_max;
        let mut x_min: Option<u16> = None;
        for (xm, &c) in ln.as_bytes().iter().enumerate().filter(|(_, &c)| c!=b' ') {
            let x = xm as u16 +1;
            if x_min.is_none() {x_min = Some(x);}
            if c==b'.' {
                let x_right = if x==x_max {x_min.unwrap()} else {x+1};
                let (y_min, y_max) = (col_min[&x], col_max[&x]);
                let y_up = if y==y_min {y_max} else {y-1};
                let y_down = if y==y_max {y_min} else {y+1};

                m.insert([x, y], [//>v<^
                    [x_right, y],
                    [x, y_down],
                    [x_left, y],
                    [x, y_up],
                ]);
            }
            x_left = x;
        }
    };
    // start pos
    let (xm,_) = s.as_bytes().iter().enumerate().find(|(_, &c)| c!=b' ').unwrap();
    ([xm as u16 +1, 1], m)
}

fn parse_path(s: &str) -> IResult<&str, Vec<(i32, char)>> {
    nom::multi::many1(
        nom::sequence::pair(
            nom::character::complete::i32,
            nom::character::complete::one_of("LRE")
        )
    )(&s)
}

fn next_facing(facing: u8, rot: char) -> u8 {
    (facing + match rot {'R' => 1, 'L' => 3, 'E' => 0,  _ => panic!()}) % 4
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let mut input= input_s.trim_end().split("\n\n");
    let (p0, board) = parse_board(input.next().unwrap());
    let path = parse_path(&format!("{}E", input.next().unwrap())).unwrap().1;
    // Facing is 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^)

    let mut p = p0;
    let mut t = board[&p];
    let mut facing: u8 = 0;
    for (step, rot) in &path {
        for _ in 0..*step {
            let p2 = t[facing as usize];
            if let Some(t2) = board.get(&p2) {
                p = p2;
                t = *t2;    
            } else {
                break
            };
            //println!("{:?}", p);
        }
        facing = next_facing(facing, *rot);
        // println!("Turn to {}", facing);
    }
    // The final password is the sum of 1000 times the row, 4 times the column, and the facing
    let part1 = 1000*(p[1] as usize)+4*(p[0] as usize)+ (facing as usize);
    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "6032");
    assert!(res[1] == "0");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    for _ in 0..20 {solution(&input)?;} //warmup
    let start = Instant::now();
    let res = solution(&input)?;
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
