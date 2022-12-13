use anyhow::{Result, Context};
use std::{collections::{HashMap, VecDeque}, fs, time::Instant};
use vecmath::vec2_add;

struct Map {
    start: [i32;2],
    end: [i32;2],
    height: HashMap<[i32;2], u8>,
}

impl std::str::FromStr for Map {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = [0; 2];
        let mut end = [0; 2];
        let mut height: HashMap<[i32; 2], u8> = HashMap::new();
        for (i, ln) in s.split("\n").enumerate() {
            for (j, c) in ln.as_bytes().iter().enumerate() {
                let v = match c {
                    b'S' => {start = [i as i32, j as i32]; b'a'}
                    b'E' => {end = [i as i32, j as i32]; b'z'}
                    _ => *c,
                };
                height.insert([i as i32, j as i32], v);
            }
        }
        Ok(Self{start, end, height})
    }
}

const DIRECTIONS: [[i32;2];4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];


fn solution(input_s: &str) -> Result<[String; 2]> {
    let map: Map = input_s.trim_end().parse()?;

    let mut work = vec![(0usize, map.end.clone())];
    let mut visited: HashMap<[i32; 2], usize> = HashMap::new();
    visited.insert(map.end.clone(), 0);
    while let Some((d, p)) = work.pop() {
        let d2 = d + 1;
        for p2 in DIRECTIONS.iter().map(|nb| vec2_add(p, *nb)).filter(|p2| {
            map.height.get(&p)
                .and_then(|hp| map.height.get(p2).and_then(|hp2| Some(*hp <= hp2 + 1)))
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

    let part1 = visited.get(&map.start).unwrap();
    let part2 = map.height
        .iter()
        .filter_map(|(p, v)| if *v == b'a' { visited.get(p) } else { None })
        .min()
        .unwrap();
    Ok([part1.to_string(), part2.to_string()])
}

/// BFS: First time we visit a node will always be shortest path
fn solution_bfs(input_s: &str) -> Result<[String; 2]> {
    let map: Map = input_s.trim_end().parse()?;

    let mut work: VecDeque<(usize, [i32;2])> = VecDeque::new();
    let mut visited: HashMap<[i32; 2], usize> = HashMap::new();
    visited.insert(map.end.clone(), 0);
    work.push_back((0usize, map.end.clone()));
    while let Some((d2, p)) = work.pop_front().map(|(d,p)| (d+1, p)) {
        for p2 in DIRECTIONS.iter().map(|nb| vec2_add(p, *nb)).filter(|p2| {
            map.height.get(&p)
                .and_then(|hp| map.height.get(p2).and_then(|hp2| Some(*hp <= hp2 + 1)))
                .unwrap_or(false)
        }) {
            visited.entry(p2).or_insert_with(|| {work.push_back((d2, p2)); d2});
        }
    }

    let part1 = visited.get(&map.start).unwrap();
    let part2 = map.height.iter()
        .filter_map(|(p, v)| if *v == b'a' { visited.get(p) } else { None })
        .min().unwrap();
    Ok([part1.to_string(), part2.to_string()])
}


// Make it simple to compare timing for multiple solutions
type Solution = dyn Fn(&str) -> Result<[String; 2]>;
const SOLUTIONS: [(&str, &Solution); 2] = [
    ("Dijkstra", &solution),
    ("BFS", &solution_bfs),
];

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    for (name, solution) in SOLUTIONS {
        let res = solution(&input).with_context(|| format!("Running solution {}", name))?;
        println!("---\n{}\nPart 1: {}\nPart 2: {}", name, res[0], res[1]);
        assert!(res[0] == "31");
        assert!(res[1] == "29");
    }
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    for (_, solution) in SOLUTIONS.iter().cycle().take(10) {
        solution(&input)?;
    } //warmup
    for (name, solution) in SOLUTIONS {
        let start = Instant::now();
        let res = solution(&input)?;
        println!(
            "---\n{} ({} us)\nPart 1: {}\nPart 2: {}",
            name, start.elapsed().as_micros(), res[0], res[1],
        );
    }
    Ok(())
}