#![allow(unused_imports, unused_variables, dead_code)]

use anyhow::{anyhow, Context, Result};
use std::{fs, time::Instant, collections::HashSet};
use nom;
use nom::IResult;

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

// Card   1: 66 92  4 54 39 76 49 27 61 56 | 66 59 85 54 61 86 37 49  6 18 81 39  4 56  2 48 76 72 71 25 27 67 10 92 13


// struct Card {
//     win: HashSet<u16>,
//     have: HashSet<u16>,
// }



// fn parse(s: &str) -> Result<Vec<(u16, (Vec<u16>, Vec<u16>))>> {
//     //nom::character::complete
//         //nom::bytes::complete::tag(""))
//     let u16_list = nom::multi::separated_list1(nom::character::complete::space1, nom::character::complete::u16);
//     let win_have = nom::sequence::separated_pair(u16_list, nom::bytes::complete::tag(" | "), u16_list);
//     let id = nom::sequence::delimited(
//                 nom::sequence::pair(nom::bytes::complete::tag("Game"), nom::character::complete::space1),
//                 nom::character::complete::u16,
//                 nom::bytes::complete::tag(": "));
//     let card = nom::sequence::pair(id, win_have);
//     let all = nom::multi::separated_list1(nom::character::complete::newline, card);
//     let (rest, cards) = all(s)?;
//     Ok(cards)
// }

fn solution(input_s: &str) -> Result<[String; 2]> {
    // let input: Vec<i32> = input_s
    //     .trim_end()
    //     .split("\n")
    //     .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
    //     .collect::<Result<_, _>>()?;

    //let mut u16_list = nom::multi::separated_list1(nom::bytes::complete::tag(" "), nom::character::complete::u16::<&str, nom::error::VerboseError<_>>);
    let win_have = nom::sequence::separated_pair(
        nom::multi::separated_list1(nom::character::complete::space1::<&str, nom::error::Error<_>>, nom::character::complete::u16::<&str, nom::error::Error<_>>),
        nom::sequence::pair(nom::bytes::complete::tag(" |"), nom::character::complete::space1::<&str, nom::error::Error<_>>),
        nom::multi::separated_list1(nom::character::complete::space1::<&str, nom::error::Error<_>>, nom::character::complete::u16::<&str, nom::error::Error<_>>));
    let id = nom::sequence::delimited(
                nom::sequence::pair(nom::bytes::complete::tag("Card"), nom::character::complete::space1::<&str, nom::error::Error<_>>),
                nom::character::complete::u16,
                nom::sequence::pair(
                    nom::bytes::complete::tag(":"),
                    nom::character::complete::space1::<&str, nom::error::Error<_>>
                ));
    let mut card = nom::sequence::pair(id, win_have);
    // let mut all = nom::multi::separated_list1(nom::character::complete::newline, card);
    // let (rest, input): (&str, Vec<(u16, (Vec<u16>, Vec<u16>))>) = all(input_s.trim_end()).map_err(|e| e.to_owned())?;
    // println!("Rest: \n{}\nEND OF REST", rest);
    //println!("Input: {:?}", input);

    let mut part1: usize = 0;
    for ln in input_s.trim_end().split("\n") {
        let (rest, (_, (wins, have)))= card(ln).map_err(|e| e.to_owned())?;
        let n = have.iter().filter(|h| wins.contains(h)).count();
        let score = if n==0 {0} else {1usize<<(n-1)};
        println!("Score: {}, Line; {}", score, ln);
        part1 +=score;
    }

    // let part1: usize = input.iter().map(|(_, (wins, have))|{
    //     let wins: HashSet<u16> = HashSet::from_iter(wins.iter().cloned());
    //     let n = have.iter().filter(|h| wins.contains(h)).count();
    //     if n==0 {0} else {1usize<<(n-1)}
    // }).sum();

    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "13");
    assert_eq!(res[1], "0");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    // let input = &fs::read_to_string("test00.txt")?;
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
