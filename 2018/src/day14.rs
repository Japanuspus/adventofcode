extern crate cgmath;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::cmp::Ordering;

#[cfg(test)]
mod tests {
    use super::*;
    const TT:& str = "";
    #[test]
    fn part1() {
        assert_eq!(part1_01(TT),0);
    }
}

enum Tile {
    Blank,
    Wall,
    Mark,
    Target(usize),
}
type Pt = cgmath::Point2<usize>;
fn pt_cmp(a: &Pt, b: &Pt) -> Ordering {
    if a.y == b.y {a.x.cmp(&b.x)} else {a.y.cmp(&b.y)} 
}


#[derive(Debug)]
struct Actor {
    location: Option<Pt>,
    hitpoints: i8,
}

type Board = Vec<Vec<Tile>>; 
fn board_get<'a>(b: &'a Board, p: &'_ Pt) -> &'a Tile {&b[p.y][p.x]}
fn board_mark(b: &mut Board, p: &Pt) {b[p.y][p.x] = Tile::Mark;}



fn parse_input(d: &str) -> (Board, Vec<Actor>, Vec<Actor>) {
    let mut elfs: Vec<Actor> = Vec::new();
    let mut goblins: Vec<Actor> = Vec::new();
    let b: Board = d.lines().map(|l| l.as_bytes().iter().map(|c| match c {
        _ => Tile::Blank
    }).collect()).collect();
    (b, elfs, goblins)
} 


fn battle_round(board: &Board, att: &mut Vec<Actor>, def: &mut Vec<Actor>) {
    let mut actions: BTreeMap<usize, usize> = BTreeMap::new();

    // Fifo of attack targets (and generation)
    let mut fill: VecDeque<(Pt, usize)> = VecDeque::new(); 
    //Next attack target to process      
    // add live defense actors in read order 
    let v: Vec<(&Pt, usize)> = def.iter().enumerate().filter_map(|(i, a)| match a {
        Actor {location: Some(pt), hitpoints: hp} => Some((pt, i)),
        _ => None
    }).collect();
    v.sort_unstable_by(|a, b| pt_cmp(&a.0, &b.0));
    for d in v {fill.push_back(*d.clone());}


    .sorted_by(|a,b|  pt_cmp(a.0, b0))
    
}

pub fn part1_01(d: &str) -> i64 {
    let (board0, mut elfs, mut gobs) = parse_input(d);

    0
}

pub fn part2_01(_d: &str) -> i64 {
    0
}

pub fn run(data: &str) {
    println!("Part 1: {}", part1_01(&data));
    println!("Part 2: {}", part2_01(&data));
}