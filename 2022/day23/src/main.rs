#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use vecmath::vec2_add;
use std::{fs, time::Instant, collections::{HashSet, HashMap}};
use itertools::{Itertools, MinMaxResult};
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
// struct Dir {
//     shift: [i16;2],
//     check: [[i16;2];3],
// }

// const DIRECTIONS: [Dir;4] = [
//     Dir{shift:[ 0, -1], check: [[-1, -1], [ 0, -1], [ 1, -1]]}, //N
//     Dir{shift:[ 0,  1], check: [[-1,  1], [ 0,  1], [ 1,  1]]}, //S
//     Dir{shift:[-1,  0], check: [[-1, -1], [-1,  0], [-1,  1]]}, //W
//     Dir{shift:[ 1,  0], check: [[ 1, -1], [ 1,  0], [ 1,  1]]}, //E
// ];

// 0 7 6 
// 1   5
// 2 3 4
const NEIGHBORS: [Pos;8] = [[-1, -1], [-1, 0], [-1,1], [0, 1], [1, 1], [1, 0], [1, -1], [0, -1]];
struct Dir {
    shift: usize,
    check: [usize; 3],
}
const DIRECTIONS: [Dir;4] = [
    Dir{shift:7, check: [6, 7, 0]}, //N
    Dir{shift:3, check: [2, 3, 4]}, //S
    Dir{shift:1, check: [0, 1, 2]}, //W
    Dir{shift:5, check: [4, 5, 6]}, //E
];

type Pos = [i16;2];
type Elfs = HashSet::<Pos>;
fn move_1(elfs: &Elfs, d0: usize) -> Elfs {
    let mut props: HashMap<Pos, usize> = HashMap::new();
    let mut props_by_elf: Vec<(Pos, Option<Pos>)> = Vec::new();
    for elf in elfs.iter() {
        let to_check: Vec<(Pos, bool)> = NEIGHBORS.iter().map(|nb| {let p = vec2_add(*elf, *nb); (p, elfs.contains(&p))}).collect();
        let od: Option<&Dir> = if to_check.iter().all(|(_, occ)| !occ) {
            // All nbs unocc
            None
        } else {
            DIRECTIONS.iter().cycle().skip(d0).take(4).find(|d| to_check.iter().cycle().skip(d.check[0]).take(3).all(|(_, occ)| !occ))
        };
        match od {
            None => props_by_elf.push((*elf, None)),
            Some(d) => {
                let p = to_check[d.shift].0;
                *(props.entry(p).or_default())+=1;
                props_by_elf.push((*elf, Some(p)));
            }
        };
    };
    props_by_elf.into_iter().map(|(elf, m)| {
        if let Some(p) = m {
            if props[&p]<2 {p} else {elf}
        } else {elf}
    }).collect()
}

fn print_elfs(elfs: &Elfs) {
    for r in -3..15 {
        for c in -3..15 {
            print!("{}", if elfs.contains(&[c as i16, r as i16]) {'#'} else {'.'});
        }
        println!();
    }
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: HashSet::<[i16;2]> = input_s.trim_end()
        .split("\n").enumerate()
        .flat_map(|(r, s)| s.chars().enumerate()
            .filter_map(move |(c, v)| if v=='#' {Some([c as i16, r as i16])} else {None})
        ).collect();

    // coords are col, row
    let mut elfs = input.clone();
    //print_elfs(&elfs);
    for id in 0..10 {
        elfs = move_1(&elfs, id%4);
        //println!("\n After Iteration {}", id+1);
        //print_elfs(&elfs);
    }
    let part1 = match (elfs.iter().map(|p| p[0]).minmax(), elfs.iter().map(|p| p[1]).minmax()) {
        (MinMaxResult::MinMax(x1, x2), MinMaxResult::MinMax(y1, y2)) => {
            let dx = 1+x2-x1;
            let dy = 1+y2-y1;
            dx as isize * dy as isize - elfs.len() as isize
        },
        _ => panic!(),
    };

    let mut elfs = input.clone();
    let mut id: usize = 0;
    let part2 = loop {
        let elfs2 = move_1(&elfs, id%4);
        if elfs2 == elfs {break id+1};
        id+=1;
        elfs = elfs2;
    };    

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "110");
    assert!(res[1] == "20");
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
