#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use std::{fs, time::Instant, collections::{HashMap, HashSet}};

type Ring = HashMap<i32, [i32;2]>;

fn traverse(ring: &Ring, start: i32, d: usize) -> impl Iterator<Item=i32> + '_ {
    assert!(d<2);
    itertools::unfold(start, move |i| {
        *i = ring[&i][d];
        Some(*i)
    })
}

fn print_ring(ring: &Ring) {
    for v in traverse(ring, 0, 1).take(ring.len()) {
        print!("{}, ", &v);
    }
    println!();
}

fn check_ring(ring: &Ring) -> bool {
    [0, 1].iter().all(|d| 
        Some(0) == traverse(ring, 0, *d).nth(ring.len()-1)
    )
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<i32> = input_s.trim_end()
        .split("\n")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_, _>>()?;


    let n = input.len();
    let mut ring: Ring = input.windows(3).map(|w| (w[1], [w[0], w[2]])).collect();
    let vals: HashSet<i32> = input.iter().cloned().collect();
    assert!(vals.len()==input.len());
    
    ring.insert(input[n-1], [input[n-2], input[0]]);
    ring.insert(input[0], [input[n-1], input[1]]);
    for &i in &input {
        if i==0 {continue};
        let (d, dbar) = if i<0 {(0,1)} else {(1, 0)};
        // fuse
        let nbs = *ring.get(&i).unwrap();
        ring.get_mut(&nbs[0]).unwrap()[1] = nbs[1];
        ring.get_mut(&nbs[1]).unwrap()[0] = nbs[0];
        // find insertion point
        let (prev, cur) = itertools::unfold((nbs[dbar], nbs[d]), |(prev, cur)| {
            *prev = *cur;
            *cur = ring[&cur][d];
            Some((*prev, *cur))
        }).skip((i.abs()-1) as usize).next().unwrap();
        // insert
        ring.get_mut(&prev).unwrap()[d]=i;
        ring.get_mut(&cur).unwrap()[dbar]=i;
        ring.insert(i, if d>0 {[prev, cur]} else {[cur, prev]});
        if !check_ring(&ring) {
            print_ring(&ring);
            panic!();
        };
    }

    println!("{:?}", traverse(&ring, 0, 1).skip(999).step_by(1000).take(3).collect::<Vec<i32>>());
    let part1:i32 = traverse(&ring, 0, 1).skip(999).step_by(1000).take(3).sum();
    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "3");
    assert!(res[1] == "0");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    // for _ in 0..20 {solution(&input)?;} //warmup
    let start = Instant::now();
    let res = solution(&input)?;
    println!(
        "({} us)\nPart 1: {}\nPart 2: {}",
        start.elapsed().as_micros(), res[0], res[1],
    );
    Ok(())
}

//7160 low

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
