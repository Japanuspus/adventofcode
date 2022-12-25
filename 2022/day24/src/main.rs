#![allow(unused_imports, dead_code)]
use anyhow::{Context, Result};
use vecmath::vec2_add;
use std::{fs, time::Instant, collections::{HashSet, BTreeSet}, ptr::swap_nonoverlapping};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
enum Direction {
    #[display("^")]
    N,
    #[display(">")]
    E,
    #[display("v")]
    S,
    #[display("<")]
    W,
    #[display(".")]
    Pause,
}

impl Direction {
    const VALUES:[Self;5] = [Self::N, Self::E, Self::S, Self::W, Self::Pause];
}

type Pos = [i8;2];

const fn directions(d: Direction) -> Pos {
    match d {
        Direction::N => [0, -1],
        Direction::E => [1, 0],
        Direction::S => [0, 1],
        Direction::W => [-1, 0],
        Direction::Pause => [0,0],
    }
}

#[derive(Debug, Clone, Copy)]
struct Blizzard {
    pos: Pos,
    dir: Direction,
}
type Valley = Pos; //Inner size

fn blizzard_move(blizzards: &mut Vec<Blizzard>, v: &Valley) {
    for b in blizzards.iter_mut() {
        let dp = directions(b.dir);
        b.pos = [(b.pos[0]+dp[0]).rem_euclid(v[0]), (b.pos[1]+dp[1]).rem_euclid(v[1])]
    }
}

struct MetOffice {
    blizzards: Vec<Blizzard>,
    valley: Valley,
    snapshots: Vec<HashSet::<Pos>>,
}

impl MetOffice {
    fn snapshot_at(&mut self, t: usize) -> &HashSet::<Pos> {
        while t>=self.snapshots.len() {
            blizzard_move(&mut self.blizzards, &self.valley);
            self.snapshots.push(self.blizzards.iter().map(|b| b.pos).collect());
        }
        &self.snapshots[t]
    }

    fn new(blizzards: Vec<Blizzard>, valley: Valley) -> Self {
        MetOffice{blizzards, valley, snapshots: Vec::new()}
    }
}


fn solution(input_s: &str) -> Result<[String; 2]> {
    let mut input = input_s.trim_end().split("\n").enumerate();
    let nc = input.next().unwrap().1.len()-2;
    let blizzards: Vec<Blizzard> = input.by_ref()
    .take_while(|(_, ln)| ln.as_bytes()[1]!=b'#')
    .flat_map(|(rp, ln)| ln.chars().enumerate().filter_map( move |(cp, c)| {
        match c {
            '^' => Some(Direction::N),
            '>' => Some(Direction::E),
            '<' => Some(Direction::W),
            'v' => Some(Direction::S),
            '#'|'.' => None,
            _ => panic!(),
        }.and_then(|dir| Some(Blizzard{pos: [cp as i8 -1, rp as i8 - 1], dir}))
    })).collect();
    let nr = input_s.trim_end().split("\n").count()-2;
    let valley: Valley = [nc as i8, nr as i8];
    // println!("Valley width: {}, height: {}", valley[0], valley[1]);
    // Blizzard positions: 0-indexed from top
    // nr, nc: Number of internal positions
    let mut met = MetOffice::new(blizzards, valley.clone());

    // best finish time, time, pos
    let mut front: BTreeSet<(u16, u16, Pos)> = BTreeSet::new();
    front.insert((1, 0, [0, -1])); // wrong distance -- ok here
    let mut best: u16 = u16::MAX;
    let p_out = [valley[0]-1, valley[1]-1]; // last position is at valley[0]-1, valley[1]-1 (above exit)
    while let Some((opt_finish_t, t, pos)) = front.pop_first() {
        //println!("@t={}, d: {}, visiting {:?}. Front len: {}", t, d, pos, front.len());
        if pos==p_out {best = best.min(t); continue};
        if opt_finish_t >= best {continue};
        let snapshot = met.snapshot_at(t as usize);
        for p2 in Direction::VALUES.iter()
            .map(|d| vec2_add(pos, directions(*d)))
            .filter(|p| *p==[0,-1] || p.iter().zip(valley.iter()).all(|(c, l)| *c>=0 && c<l))
            //.inspect(|p| println!("> {:?} in snap: {}", p, snapshot.contains(p)))
            .filter(|p| !snapshot.contains(p)) {
            // last position is at valley[0]-1, valley[1]-1 (above exit)
            let opt_finish_t2 = t+(p_out[0]-p2[0]) as u16 + (p_out[1]-p2[1]) as u16;
            front.insert((opt_finish_t2, t+1, p2));
        }
    }
    let part1 = best+1;
    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test06.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "18");
    assert!(res[1] == "0");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    //for _ in 0..20 {solution(&input)?;} //warmup
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
