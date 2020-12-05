#![allow(unused)]

use std::collections::{HashSet, HashMap};

#[derive(Debug)]
struct Segment {
    dir: u8,
    n: i32
}

fn parse_segment(s: &str) -> Segment {
    Segment{dir: s.as_bytes()[0], n: s[1..].parse().unwrap()}
}

fn wire_path(w: &Vec<Segment>) -> Vec<(i32, i32)> {
    let mut r = Vec::new();
    let mut row = 0;
    let mut col = 0;
    for segment in w {
        let dd = match segment.dir {
            b'R' => (0, 1),
            b'L' => (0, -1),
            b'U' => (-1, 0),
            b'D' => (1, 0),
            _ => {panic!("Unexpected direction")}
        };
        for k in 0..segment.n {
            row += dd.0;
            col += dd.1;
            r.push((row, col));
        }
    };
    r
}

fn wire_distance(w: &Vec<Segment>) -> HashMap<(i32, i32), usize> {
    // Cannot just collect to get correct duplicate semantics
    let mut r:HashMap<_,_> = HashMap::new();
    for (i, p) in wire_path(w).into_iter().enumerate() {
        r.entry(p).or_insert(i+1);
    }
    r
}

fn main() {
    let input: Vec<Vec<Segment>> = std::fs::read_to_string("input.txt")
        .expect("Error reading input file")
        .lines()
        .map(|r| r.split(',').map(|s| parse_segment(s)).collect())
        .collect();

    // Part 1
    let wire0: HashSet<_> = wire_path(&input[0]).drain(..).collect();
    let wire1: HashSet<_> = wire_path(&input[1]).drain(..).collect();
    let intersections = wire0.intersection(&wire1);
    let min_dist = intersections.clone().map(|(dr,dc)| dr.abs()+dc.abs()).min().unwrap();
    println!("Part 1: {}", min_dist);

    // Part 2
    let d0 = wire_distance(&input[0]);
    let d1 = wire_distance(&input[1]);
    let min_wire_dist = intersections.map(|p| d0.get(p).unwrap()+d1.get(p).unwrap()).min().unwrap();
    println!("Part 2: {}", min_wire_dist);
}
