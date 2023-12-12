#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
    time::Instant,
};
use vecmath::{vec2_add, vec2_len, vec2_scale, vec2_sub};
use rayon::prelude::*;

type V = [i16; 2];
const NESW: [V; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];

fn next_direction(s: u8, d: usize) -> Option<usize> {
    match (s, (d + 2) % 4) {
        (b'|', 0) => Some(2),
        (b'|', 2) => Some(0),
        (b'-', 1) => Some(3),
        (b'-', 3) => Some(1),
        (b'7', 2) => Some(3),
        (b'7', 3) => Some(2),
        (b'F', 1) => Some(2),
        (b'F', 2) => Some(1),
        (b'L', 0) => Some(1),
        (b'L', 1) => Some(0),
        (b'J', 0) => Some(3),
        (b'J', 3) => Some(0),
        _ => None,
    }
}

fn follow_pipe(map: &HashMap<V, u8>, p0: V, d0: usize) -> Option<Vec<V>> {
    let mut p = p0;
    let mut d = d0;
    let mut i: Vec<V> = Vec::new();

    loop {
        i.push(p);
        p = vec2_add(p, NESW[d]);
        if let Some(&c) = map.get(&p) {
            if c == b'S' {
                return Some(i)
            }
            if let Some(d_new) = next_direction(c, d) {
                d = d_new;
                continue;
            }
        }
        break
    };
    None
}

/// redoing https://insignificancegalore.net/2008/10/implementing-fast-point-in-polygon/
/// If there is only one connected component, then it is optional to repeat the first vertex at the end. It's also optional to surround the component with zero vertices.
/// https://web.archive.org/web/20100430183237/http://www.ecse.rpi.edu/Homepages/wrf/Research/Short_Notes/pnpoly.html
fn pnpoly(edge: &Vec<V>, test: V) -> bool {
    // edge.iter().circular_tuple_windows().filter(|(vi, vj)| // is three times slower 
    std::iter::once(&[edge[edge.len()-1], edge[0]][..])
    .chain(edge.windows(2))
    .filter(|w| 
        ((w[0][1] > test[1]) != (w[1][1] > test[1]))
        && (test[0] < (w[1][0] - w[0][0]) * (test[1] - w[0][1]) / (w[1][1] - w[0][1]) + w[0][0])
    ).count()%2!=0
}

// stretch goal: use &Vec in this trait and implementation (I tried and failed) 
trait PolyTester {
    fn new(edge: Vec<V>) -> Self;
    fn point_in_poly(&self, test: V) -> bool;
}

struct PNPolyTester {
    edge: Vec<V>,
}

// 47ms
impl PolyTester for PNPolyTester {
    fn new(edge: Vec<V>) -> Self { Self { edge } }
    fn point_in_poly(&self, test: V) -> bool { pnpoly(&self.edge, test)}
}

struct Segment {
    x1: i16,
    y1: i16,
    y2: i16,
    dx: i16,
    dy: i16,
}
struct VecPolyTester {
    edge: Vec<V>,
    segments: Vec<Segment>,
}

// 47ms
impl PolyTester for VecPolyTester {
    fn new(edge: Vec<V>) -> Self {
        let segments: Vec<_> = 
        std::iter::once(&[edge[edge.len()-1], edge[0]][..]).chain(edge.windows(2))
        .map(|w| Segment{
            x1: w[0][0],
            y1: w[0][1],
            y2: w[1][1],
            dx: w[1][0]-w[0][0],
            dy: w[1][1]-w[0][1],
        }).collect();
        Self{edge, segments}
    }
    fn point_in_poly(&self, test: V) -> bool {
        let (x, y) = (test[0], test[1]);
        self.segments.iter().filter(
            |s| ((s.y1>y) != (s.y2>y)) && ((x-s.x1)<s.dx*(y-s.y1)/s.dy)
        ).count()%2 !=0
    }
}

fn solution<T>(input_s: &str) -> Result<[String; 2]> 
where
    T: PolyTester + std::marker::Sync
{
    let map: HashMap<V, u8> = input_s
        .trim_end()
        .split("\n")
        .enumerate()
        .flat_map(|(y, ln)| {
            ln.as_bytes()
                .iter()
                .enumerate()
                .map(move |(x, c)| ([x as i16, y as i16], *c))
        })
        .collect();
    let p0 = map
        .iter()
        .find_map(|(k, v)| if *v == b'S' { Some(*k) } else { None })
        .unwrap();

    let max_loop  = (0..4usize)
        .filter_map(|d0| follow_pipe(&map, p0, d0))
        .max_set_by_key(|l| l.len()).pop().unwrap();
    let part1 = (max_loop.len() + 1) / 2;

    let x_max = max_loop.iter().map(|p| p[0]).max().unwrap();
    let y_max = max_loop.iter().map(|p| p[1]).max().unwrap();
    let on_edge: HashSet<V> = max_loop.iter().cloned().collect();
    let pt = T::new(max_loop);
    let part2: usize = (0..x_max).into_par_iter()
        .map(|x| 
            (0..y_max)
            .map(move |y| [x, y])
            .filter(|p| !on_edge.contains(p) && pt.point_in_poly(*p))
            .count()
        ).sum();

    Ok([part1.to_string(), part2.to_string()])
}

type Solution = dyn Fn(&str) -> Result<[String; 2]>;
const SOLUTIONS: [(&str, &Solution); 2] = [
    ("Original", &solution::<PNPolyTester>),
    ("Vec", &solution::<VecPolyTester>),
];

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test07.txt")?;
    for (name, solution) in SOLUTIONS {
        let res = solution(&input).with_context(|| format!("Running solution {}", name))?;
        println!("---\n{}\nPart 1: {}\nPart 2: {}", name, res[0], res[1]);
        assert_eq!(res[0], "8");
    }

    let input = &fs::read_to_string("test14.txt")?;
    for (name, solution) in SOLUTIONS {
        let res = solution(&input).with_context(|| format!("Running solution {}", name))?;
        println!("---\n{}\nPart 1: {}\nPart 2: {}", name, res[0], res[1]);
        assert_eq!(res[1], "10");
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
