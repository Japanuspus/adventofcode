extern crate itertools;
use itertools::Itertools;

//use std::collections::BTreeMap;
use std::collections::VecDeque;
//use std::cmp::Ordering;

// x is i -- row number starting from top
// y is j -- column number starting from left
type Pt = (usize, usize);

fn pt_shift(pt: & Pt, (di,dj): &(isize, isize)) -> Pt {
    (
        ((pt.0 as isize) + di) as usize, 
        ((pt.1 as isize) + dj) as usize, 
    )
}
const NB_DIJ: &[(isize, isize); 4] = &[(-1,0), (0, -1), (0, 1), (1, 0)];
fn pt_neighbors(p: & Pt) -> Vec<Pt> {
    NB_DIJ.iter().map(|dij| pt_shift(p, dij)).collect()
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Team {
    Elf = 0,
    Goblin = 1,
}

#[derive(Debug, Clone)]
struct Actor {
    team: Team,
    att: i32, 
    hp: i32,
    pos: Option<Pt>,
}

#[derive(Debug, Clone)]
enum Tile {
    Blank,
    Wall,
    Mark,
    Actor(usize),
}

#[derive(Debug, Clone)]
struct Board {
    tiles: Vec<Vec<Tile>>,
    actors: Vec<Actor>
}

impl Board {
    // Apply attack to piece at position p, return final hp
    // Panics if no actor found at site of attack
    fn actor_attack(self: & mut Self, ia: usize, p: &Pt) -> i32{
        let i = self.index_at(p).unwrap();
        let n = self.actors[ia].att;
        if self.actors[i].hp <= n {
            { 
                let mut a = & mut self.actors[i];
                a.pos = None;
                a.hp = 0;
            }
            self.set_at(p, Tile::Blank);
        } else {
            self.actors[i].hp -= n;
        };
        self.actors[i].hp
    }

    // Move actor i to position p
    fn actor_move(self: & mut Self, i: usize, p: &Pt) {
        //let a: & mut Actor =  &mut self.actors[i];
        // = Some(p.clone()); 
        self.set_at(&self.actors[i].pos.unwrap(), self.get(p).clone());
        self.set_at(p, Tile::Actor(i));
        self.actors[i].pos = Some(p.clone());
    }


    fn get<'a>(self: &'a Self, p: &'_ Pt) -> &Tile {
        &self.tiles[p.0][p.1]
    }


    fn index_at<'a>(self: &'a Self, p: &'_ Pt) -> Option<usize> {
        match self.get(p) {
            Tile::Actor(i) => Some(*i), 
            _ => None,
        }
    }

    fn actor_at<'a>(self: &'a Self, p: &'_ Pt) -> &'a Actor {
        &self.actors[self.index_at(p).unwrap()]
    }

    fn set_at(self: & mut Self, p: &Pt, t: Tile) { self.tiles[p.0][p.1] = t; }
    fn mark(self: & mut Self, p: &Pt) { self.set_at(p, Tile::Mark) }

    //Indices of active actors sorted by position of actors
    fn actors_sorted(self: & Self) -> Vec<usize> {
        self.actors.iter().enumerate()
        .filter_map(|(i, a)| a.pos.and_then(|p| Some( (p, i) )) )
        .sorted().map(|(_,i)| i).collect()
    }

    fn hp(self: & Self, team: Team) -> i32 {
        self.actors.iter().map(|a| if a.team==team {a.hp} else {0}).sum()
    }

    fn count(self: & Self, team: Team) -> usize {
        self.actors.iter().filter(|a| a.team==team && a.hp>0).count()
    }


    fn set_attack(self: & mut Self, t: &Team, att: i32) {
        for a in &mut self.actors {
            if a.team==*t {
                (*a).att = att;
            }
        }
    }
}

use std::fmt;
use std::fmt::Write;
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.tiles.iter() {
            for tile in row.iter() {
                f.write_char(match tile {
                    Tile::Blank => '.',
                    Tile::Wall => '#',
                    Tile::Mark => 'X',
                    Tile::Actor(i) if self.actors[*i].team == Team::Elf  => 'E',
                    Tile::Actor(i) if self.actors[*i].team == Team::Goblin  => 'G',
                    _ => '?',
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

use std::str::FromStr;
impl FromStr for Board {
    type Err = std::char::ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut actors : Vec<Actor> = Vec::new();
        let tiles = s.lines().enumerate().map(|(i, l)| 
            l.chars().enumerate().map(|(j, c)| match c {
                'E'|'G' => {
                    actors.push(Actor {
                        team: if c=='E' {Team::Elf} else {Team::Goblin},
                        att: 3, 
                        hp: 200,
                        pos: Some((i, j))
                        });
                    Tile::Actor(actors.len()-1)
                },
                '#' => Tile::Wall,
                _ => Tile::Blank,
            }).collect()).collect();
        Ok(Board{tiles, actors})
    }
}


// Maybe position of blank neighbor position
// Assumes that there are no enemies as neighbors
// None if no enemies can be reached or if this tile is dead
fn propose_step_old(board: &Board, i: usize) -> Option<Pt> {
    //if let Tile::Actor(Actor {team, hitpoints: _}) = board.get(&p0) {
    if let Actor{ref team, pos: Some(p0), ..} = board.actors[i] {        
        let mut b = board.clone();
        let p1s: Vec<Pt> = pt_neighbors(&p0);

        // Enumerate starting directions to know where to go
        let mut visit: VecDeque<(usize, Pt)> = VecDeque::new();
        visit.extend(p1s.clone().into_iter().enumerate());
        loop {
            if let Some((i, p)) = visit.pop_front() {
                match b.get(&p)  {
                    Tile::Blank => {
                        b.mark(&p); 
                        // Add possible continuations, marked by
                        // corresponging starting direction
                        visit.extend(pt_neighbors(&p).into_iter().map(|n| (i, n)));
                    },
                    Tile::Actor(idx) if !(b.actors[*idx].team==*team) => {
                        // Found an actor from the other team
                        // break position out of loop
                        break Some(p1s[i].clone());
                    },
                    _ => {}
                }
            } else {
                // No more places to go
                break None
            }
        }
    } else {
        // our actor was already dead 
        None
    }
}

// Maybe position of blank neighbor position
// Assumes that there are no enemies as neighbors
// None if no enemies can be reached or if this tile is dead
fn propose_step(board: &Board, i: usize) -> Option<Pt> {
    //if let Tile::Actor(Actor {team, hitpoints: _}) = board.get(&p0) {
    if let Actor{ref team, pos: Some(p0), ..} = board.actors[i] {        
        let mut b = board.clone();
        let p1s: Vec<Pt> = pt_neighbors(&p0);

        // Enumerate starting directions to know where to go
        let mut next_visit: Vec<(usize, Pt)> = p1s.clone().into_iter().enumerate().collect();
        let mut targets: Vec<(Pt, usize)> = Vec::new();
        while targets.is_empty() && !next_visit.is_empty(){
            let visit: Vec<(usize, Pt)> = next_visit.clone();
            next_visit.clear();
            for (i, p) in visit {
                match b.get(&p)  {
                    Tile::Blank => {
                        b.mark(&p); 
                        // Add possible continuations, marked by
                        // corresponging starting direction
                        next_visit.extend(pt_neighbors(&p).into_iter().map(|n| (i, n)));
                    },
                    Tile::Actor(idx) if !(b.actors[*idx].team==*team) => {
                        // Found a possbile target
                        targets.push((p, i));
                    },
                    _ => {}
                }
            };
        };
        if targets.is_empty() {
            None
        } else {
            // Pick closes target by reading order
            targets.sort();
            Some(p1s[targets[0].1])
        }
    } else {
        // our actor was already dead 
        None
    }
}


#[test]
fn test_propose_step() {
let tt1 = &"#######
#.E..G#
#.#####
#G#####
#######";

    let b = Board::from_str(tt1).expect("Failed parsing");
    assert_eq!(propose_step(&b, 0), Some((1, 3)) );
}


fn propose_attack(board: &Board, i: usize) -> Option<Pt> {
    // actor at p0 may have died since call was planned
    if let Actor{ref team, pos: Some(p0), ..} = board.actors[i] {
        //let team = board.get(p0).get_team().unwrap();
        // min_by_key return first entry if multiple are present
        pt_neighbors(&p0).iter().filter_map(|p| 
            match board.get(p) {
                Tile::Actor(j) if !(board.actors[*j].team == *team) => Some((board.actors[*j].hp, p)),
                _ => None,
            }
        ).min_by_key(|hpp| hpp.0).and_then(|(_hp, p)| Some(*p))
    } else {
        None
    }
}

/// Return true if this has been a full round:
/// - There has been activity, and
/// - Both teams have remaining pieces at the end of the round, or
/// - The last piece of a team was was removed as the final act 
fn battle_round(board: & mut Board) -> bool {
    let piece_idcs: Vec<usize> = board.actors_sorted();
    let mut activity = false;
    let mut attack_last_round = false;
    for idx in piece_idcs {
        if board.actors[idx].pos.is_none() {
            continue; 
        }
        attack_last_round = if let Some(pa) = 
            propose_attack(board, idx)
            .or_else(|| { 
                propose_step(board, idx) //safe to call, just checked for enemy neighbors
                .and_then(|p| {
                    activity=true;
                    board.actor_move(idx, &p);
                    propose_attack(board, idx)    
                })
            }) 
        {
            board.actor_attack(idx, &pa);
            activity = true;
            true
        } else {
            false
        };
    }
    activity && (attack_last_round || (board.hp(Team::Elf)*board.hp(Team::Goblin))>0)
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
    let b: Board = Board::from_str(tt0).expect("Failed parsing");
    let p: Pt = (1,1);
    let p1: Pt = (1,2);
    assert_eq!(b.actor_at(&p).team, Team::Goblin);
    assert_eq!(propose_step(&b, 0), Some(p1.clone()));

    let mut b2 = b.clone();
    b2.set_attack(&Team::Goblin, 10);
    assert_eq!(b2.actor_at(&p).att, 10);
    battle_round(& mut b2);
    assert_eq!(b2.actor_at(&p1).team, Team::Goblin);
}




fn battle(board: & mut Board) -> usize {
    let mut rounds: usize = 0;
    print!("{}\n\n", board);
    while battle_round(board) {
        rounds+=1;
        print!("Full rounds completed {}\n{}\n\n", rounds, board);
    }
    print!("After final incomplete round\n{}\n\n", board);

    rounds    
}

fn battle_quiet(board: & mut Board) -> usize {
    let mut rounds: usize = 0;
    while battle_round(board) {
        rounds+=1;
    }
    rounds    
}

pub fn part1_01(d: &str) -> usize {
    let board0 = Board::from_str(d).expect("Parsing error");
    let mut board = board0.clone();

    let rounds = battle(& mut board);
    let hp=board.hp(Team::Goblin);
    println!("Final hp: {}", hp);

    (hp as usize) * rounds
}

fn save_the_elfs(board0: & Board, attack: i32) -> Option<usize> {
    let mut board = board0.clone();
    board.set_attack(&Team::Elf, attack);

    let n0 = board.count(Team::Elf); 
    let rounds = battle_quiet(& mut board);
    let n1 = board.count(Team::Elf);
    let hp=board.hp(Team::Elf);

    println!("At attack strength {}, {} of {} Elfs were alive (total hp: {}) when ended after {} full rounds",
        attack, n1, n0, hp, rounds);
    
    if n0==n1 {Some((hp as usize)*rounds)} else {None}
}

pub fn part2_01(d: &str) -> usize {
    let board0 = Board::from_str(d).expect("Parsing error");
    let mut att = 3;
    loop {
        if let Some(score) = save_the_elfs(&board0, att) {
            //let bb: mut Board = board0.clone();
            //bb.set_attack(&Team::Elf, att)
            //battle(& mut bb);
            break score
        }
        att +=1;  
    }
}

#[test]
fn test_2() {

assert_eq!(part2_01("#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"), 4988);

assert_eq!(part2_01("####### 
#E.G#.# 
#.#G..# 
#G.#.G# 
#G..#.# 
#...E.# 
#######"), 3478);

assert_eq!(part2_01("#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"),6474);

assert_eq!(part2_01("#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"),1140);

}

pub fn board_rounds(d: &str) -> usize {
    //let mut b = Board::from_str(d).expect("Parsing error");
    battle(& mut Board::from_str(d).expect("Parsing error"))
}
#[test]
fn test_reddit_easy_to_be_wrong() {
assert_eq!(board_rounds("####
##E#
#GG#
####"), 67);

assert_eq!(board_rounds("#####
#GG##
#.###
#..E#
#.#G#
#.E##
#####"), 71);
}


pub fn run(data: &str) {
    println!("Part 1: {}", part1_01(&data));
    println!("Part 2: {}. 47584 too high", part2_01(&data));
}