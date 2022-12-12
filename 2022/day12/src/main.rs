use anyhow::Result;
use std::{collections::HashMap, fs, time::Instant};
use vecmath::vec2_add;

fn solution(input_s: &str) -> Result<[String; 2]> {
    let mut s = [0; 2];
    let mut e = [0; 2];
    let mut h: HashMap<[i32; 2], u8> = HashMap::new();
    for (i, ln) in input_s.trim_end().split("\n").enumerate() {
        for (j, c) in ln.as_bytes().iter().enumerate() {
            let v = match c {
                b'S' => {s = [i as i32, j as i32]; b'a'}
                b'E' => {e = [i as i32, j as i32]; b'z'}
                _ => *c,
            };
            h.insert([i as i32, j as i32], v);
        }
    }
    let nbs = [[0, 1], [1, 0], [0, -1], [-1, 0]];

    // flood fill distance to end
    let mut work = vec![(0usize, e.clone())];
    let mut visited: HashMap<[i32; 2], usize> = HashMap::new();
    visited.insert(e.clone(), 0);
    while let Some((d, p)) = work.pop() {
        let d2 = d + 1;
        for p2 in nbs.iter().map(|nb| vec2_add(p, *nb)).filter(|p2| {
            h.get(&p)
                .and_then(|hp| h.get(p2).and_then(|hp2| Some(*hp <= hp2 + 1)))
                .unwrap_or(false)
        }) {
            if visited
                .get(&p2)
                .and_then(|&d2_old| Some(d2 < d2_old))
                .unwrap_or(true)
            {
                visited.insert(p2.clone(), d2);
                work.push((d2, p2));
            };
        }
    }

    let part1 = visited.get(&s).unwrap();
    let part2 = h
        .iter()
        .filter_map(|(p, v)| if *v == b'a' { visited.get(p) } else { None })
        .min()
        .unwrap();
    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "31");
    assert!(res[1] == "29");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    for _ in 0..10 {
        solution(&input)?;
    } //warmup
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
