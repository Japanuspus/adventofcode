use anyhow::{Result, Context};
use ndarray::Array2;
use ndarray::ArrayView2;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{fs, collections::HashMap};
use ndarray::ArrayView;
use ndarray::arr2;
use ndarray::ShapeBuilder;

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


#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Bounds {
    b: (usize, usize),
}

impl Bounds {
    fn add(&self, p: (usize, usize), dp: (i32, i32)) -> Option<(usize, usize)> {
        let i = p.0 as isize + dp.0 as isize;
        let j = p.1 as isize + dp.1 as isize;
        if i>=0 && i<(self.b.0 as isize) && j>=0 && j< (self.b.1 as isize) {Some((i as usize, j as usize))} else {None}
    }
}



fn solution(input_s: &str) -> Result<()> {
    // let input: HashMap<(i8, i8), i8> = input_s
    //     .trim()
    //     .split("\n").enumerate()
    //     .flat_map(|(i, ln)| ln.as_bytes().iter().enumerate().map(move |(j, c)| ((i as i8, j as i8), ((*c-b'0') as i8))))
    //     .collect();
    let input: Vec<Vec<i8>> = input_s
        .trim()
        .split("\n")
        .map(|ln| ln.as_bytes().iter().map(|c| ((*c-b'0') as i8)).collect())
        .collect();
    let bounds = Bounds{b: (input.len(), input[0].len())};
    let map_buf: Vec<i8> = input.iter().flat_map(|v| v.iter()).cloned().collect();
    let map = ArrayView2::from_shape(bounds.b, &map_buf)?;
    // println!("Map:\n  {:?}", &map);

    let dirs: [(i32, i32);4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut dist = Array2::from_elem(bounds.b, usize::MAX);
    let mut heap = BinaryHeap::new();
    heap.push(State{cost: 0, position: (0,0)});
    while let Some(State{cost, position}) = heap.pop() {
        if cost>dist[position] {continue}
        for p2 in dirs.iter().filter_map(|d| bounds.add(position, *d)) {
            let c2 = cost + map[p2] as usize;
            if c2<dist[p2] {
                dist[p2] = c2;
                heap.push(State{cost: c2, position: p2})
            }
        }
        dist[position] = cost;
    }
    println!("Distances \n {:?}", &dist);
    println!("Part 1: {}", dist[bounds.add(bounds.b, (-1, -1)).unwrap()]);
    println!("Part 2: {}", 0);
    Ok(())
}

fn main() -> Result<()> {
    println!("** TEST **");
    solution(&fs::read_to_string("test00.txt")?)?;
    println!("\n** INPUT **");
    solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
