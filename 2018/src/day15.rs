extern crate cgmath;
use cgmath::Point2;
use cgmath::Vector2;
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

#[derive(Debug, Clone)]
enum Team {
    Elf = 0,
    Goblin = 1,
}

#[derive(Debug, Clone)]
struct Actor {
    team: Team, 
    hitpoints: i8,
}

#[derive(Debug, Clone)]
enum Tile {
    Blank,
    Wall,
    Mark,
    Actor(Actor),
}


const NB_POINTS: [Vector2<isize>; 4] = [
    Vector2::new(0, -1),
    Vector2::new(-1, 0),
    Vector2::new(1, 0),
    Vector2::new(0, 1),
]; 

struct Pt {
    pub pt: Point2<isize>,
}
impl Pt {
    fn pt_cmp(a: &Pt, b: &Pt) -> Ordering {
        if a.pt.y == b.pt.y {a.pt.x.cmp(&b.pt.x)} else {a.pt.y.cmp(&b.pt.y)} 
    }
    fn neighbors(&self) -> &Iterator<Item=Pt> {
        &((0..4).map(|i| Pt {pt: self.pt+NB_POINTS[i]}))
    }
}


struct Board {
    b: Vec<Vec<Tile>>, 
}
impl Board {
    fn get<'a>(&'a self, p: &'_ Pt) -> &'a Tile {
        &self.b[p.pt.y][p.pt.x]
    }
    fn board_mark(b: &mut Board, p: &Pt) {
        &self.b[p.pt.y][p.pt.x] = Tile::Mark;
    }
}
fn parse_input(d: &str) -> Board {
    d.lines().map(|l| l.as_bytes().iter().map(|c| match c {
        _ => Tile::Blank
    }).collect()).collect()
} 

fn find_move(board: &Board, a: &Actor)->Option<Pt> {
    // We could keep the board pristine and save visited
    // status elsewhere, but here we use the board for marking
    // directly.Actor
    // All actors must must be marked on board
    let mut b = *board.clone();
    let mut visit: VecDeque<(Pt, usize)> = VecDeque::new(); 
    a.location.
    let v: Vec<(&Pt, usize)> = def.iter().enumerate().filter_map(|(i, a)| match a {
        Actor {location: Some(pt), hitpoints: hp} => Some((pt, i)),
        _ => None
    }).collect();
    v.sort_unstable_by(|a, b| pt_cmp(&a.0, &b.0));

}

fn battle_round(board: &Board, att: &mut Vec<Actor>, def: &mut Vec<Actor>) {
    /*
    The board as delivered only holds walls.
    - First thing is to mark the live targets with their indices
    - 
    */

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
    //for d in v {fill.push_back(*d.clone());}
    //.sorted_by(|a,b|  pt_cmp(a.0, b0))    
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