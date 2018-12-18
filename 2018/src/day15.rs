extern crate cgmath;
use cgmath::Point2;
use cgmath::Vector2;
//use std::collections::BTreeMap;
use std::collections::VecDeque;
//use std::cmp::Ordering;

#[cfg(test)]
mod tests {
    use super::*;
    const TT:& str = "";
    #[test]
    fn part1() {
        assert_eq!(part1_01(TT),0);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pt {
    // x is i -- row number starting from top
    // y is j -- column number starting from left
    pub pt: Point2<isize>,
}
const NB_DXY: &[(isize, isize); 4] = &[(-1,0), (0, -1), (0, 1), (1, 0)];

struct PtIter {
    pt: Pt,
    i: usize,
}

impl Iterator for PtIter {
    type Item = Pt;
    fn next(&mut self) -> Option<Pt> {
        if self.i>3 {None} else {let j=self.i; self.i+=1; Some(self.pt.shift(NB_DXY[j]))}
    }
}

impl Pt {
    //fn pt_cmp(a: &Pt, b: &Pt) -> Ordering {
    //    if a.pt.y == b.pt.y {a.pt.x.cmp(&b.pt.x)} else {a.pt.y.cmp(&b.pt.y)} 
    //}

    fn new((x, y): (isize, isize)) -> Pt {Pt {pt: Point2::<isize>{x, y}}}
    fn shift(&self, (x,y): (isize, isize)) -> Pt { Pt {pt: self.pt + Vector2 {x, y}}}
    fn neighbors<'a>(&'a self) -> PtIter {
        PtIter {pt: self.clone(), i:0}
    }
}

#[test]
fn test_pt() {
    let pt = Pt::new((7,5));
    let mut nn = pt.neighbors();
    assert_eq!(nn.next(), Some(Pt::new((6,5))));
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Team {
    Elf = 0,
    Goblin = 1,
}

#[derive(Debug, Clone)]
struct Actor {
    team: Team, 
    hitpoints: i32,
}

#[derive(Debug, Clone)]
enum Tile {
    Blank,
    Wall,
    Mark,
    Actor(Actor),
}
impl Tile {
    fn get_team(&self) -> Option<&Team> {
        match self {
            Tile::Actor(Actor {team: t, hitpoints: _}) => Some(t),
            _ => None,
        }
    }
    fn get_hp(&self) -> Option<i32> {
        match self {
            Tile::Actor(Actor {team: _, hitpoints: hp}) => Some(*hp),
            _ => None,
        }
    }
    fn aply_attack(&self) -> Option<Tile> {
        if let Tile::Actor(a) = &self {
            let n = 3;
            Some(if a.hitpoints < n {
                Tile::Blank
            } else {
                Tile::Actor(Actor {hitpoints: a.hitpoints-n, team: a.team.clone()})
            }) 
        } else { None }
    }
    fn as_char(&self) -> char {
        match self {
            Tile::Blank => '.',
            Tile::Wall => '#',
            Tile::Mark => 'X',
            Tile::Actor(Actor {team: Team::Elf, hitpoints: _}) => 'E',
            Tile::Actor(Actor {team: Team::Goblin, hitpoints: _}) => 'G',
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    b: Vec<Vec<Tile>>, 
}
impl Board {
    fn get<'a>(self: &'a Self, p: &'_ Pt) -> &'a Tile {
        &self.b[p.pt.x as usize][p.pt.y as usize]
    }
    fn mark(self: & mut Self, p: &Pt) {
        self.b[p.pt.x as usize][p.pt.y as usize] = Tile::Mark;
    }
    fn actor_locations(self: & Self) -> Vec<Pt> {
        let mut actor_locations: Vec<Pt> = Vec::new();
        for (i, row) in self.b.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                if let Tile::Actor(_) = tile {
                    actor_locations.push(Pt::new((i as isize, j as isize)));
                }
            }
        }
        actor_locations
    }
}

use std::fmt;
use std::fmt::Write;
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.b.iter() {
            for tile in row.iter() {
                f.write_char(tile.as_char())?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn parse_input(d: &str) -> Board {
    let b = d.lines().map(|l| l.chars().map(|c| match c {
        'E' => Tile::Actor(Actor {team: Team::Elf, hitpoints: 200}),
        'G' => Tile::Actor(Actor {team: Team::Goblin, hitpoints: 200}),
        '#' => Tile::Wall,
        _ => Tile::Blank,
    }).collect()).collect();
    Board {b}
} 

// Return a optional neighbor position that is either a
// blank tile or an enemy to attach
fn propose_step(board: &Board, p0: &Pt) -> Option<Pt> {
    if let Tile::Actor(Actor {team, hitpoints: _}) = board.get(&p0) {
        let mut b = board.clone();
        let p1s: Vec<Pt> = p0.neighbors().collect();
        let mut visit: VecDeque<(usize, Pt)> = VecDeque::new();
        visit.extend(p1s.clone().into_iter().enumerate());
        loop {
            if let Some((i, p)) = visit.pop_front() {
                let tile = b.get(&p);
                match tile {
                    Tile::Blank => {
                        b.mark(&p); 
                        visit.extend(p.neighbors().map(|n| (i, n)));
                    },
                    Tile::Actor(Actor {team: t, hitpoints: _}) if !(t==team) => {
                        break Some(p1s[i].clone());
                    },
                    _ => {}
                }
            } else {
                break None;
            }
        }
    } else { None }
}

fn propose_attack(board: &Board, p0: &Pt) -> Option<Pt> {
    // actor at p0 may have died since call was planned
    if let Tile::Actor(Actor {team, hitpoints: _}) = board.get(&p0) {
        //let team = board.get(p0).get_team().unwrap();
        // min_by_key return first entry if multiple are present
        p0.neighbors().filter_map(|p| 
            match board.get(&p) {
                Tile::Actor( Actor {team: t, hitpoints: hp}) if !(t==team) => Some((hp, p)),
                _ => None,
            }
        ).min_by_key(|hpp| hpp.0).and_then(|(_hp, p)| Some(p))
    } else {
        None
    }
}

fn board_move(board: & mut Board, p0: &Pt, p1: &Pt) {
    let t = &board.b[p0.pt.x as usize][p0.pt.y as usize];
    board.b[p1.pt.x as usize][p1.pt.y as usize] = t.clone();
    board.b[p0.pt.x as usize][p0.pt.y as usize] = Tile::Blank;
}

fn board_attack(board: & mut Board, p: &Pt) {
    let tr = &board.b[p.pt.x as usize][p.pt.y as usize];
    board.b[p.pt.x as usize][p.pt.y as usize] = tr.aply_attack().unwrap();
}

fn battle_round(board: & mut Board) -> bool {
    let actor_locations = board.actor_locations();
    let mut activity = false;

    for p0 in actor_locations {
        let maybe_target = propose_attack(board, &p0)
        .or_else(|| { 
            propose_step(board, &p0)
            .and_then(|p| {
                activity=true;
                board_move(board, &p0, &p);
                propose_attack(board, &p)    
            })
        });
        if let Some(pa) = maybe_target {
            activity=true;
            board_attack(board, &pa);
        };
    }
    activity
}

#[test]
fn test_board() {
let tt0 = &"#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########";
    let b = parse_input(tt0);
    let p = Pt::new((1,1));
    let p1 = Pt::new((1,2));
    assert_eq!(b.get(&p).get_team(), Some(&Team::Goblin));
    assert_eq!(propose_step(&b, &p), Some(p1.clone()));

    let mut b2 = b.clone();
    battle_round(& mut b2);
    assert_eq!(b2.get(&p1).get_team(), Some(&Team::Goblin));
}



pub fn part1_01(d: &str) -> i64 {
    let board0 = parse_input(d);
    let mut board = board0.clone();

    print!("{}\n\n", &board);
    while battle_round(& mut board) {
        print!("{}\n\n", &board);
    }

    0
}

pub fn part2_01(_d: &str) -> i64 {
    0
}

pub fn run(data: &str) {
    println!("Part 1: {}", part1_01(&data));
    println!("Part 2: {}", part2_01(&data));
}