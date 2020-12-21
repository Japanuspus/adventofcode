use anyhow::Result;
// use apply::Apply;
use itertools::Itertools;
use std::{collections::HashSet, fs};
use nom::{Finish, IResult};
use nom::multi::{many1, separated_list1};
use nom::character::complete::{digit1, one_of};
use nom::combinator::{map_res, map, recognize};
use nom::sequence::{separated_pair, delimited};
use nom::bytes::complete::tag;

#[derive(Debug)]
struct RawTile {
    id: usize,
    pixels: Vec<Vec<bool>>
}

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    sides: [Vec<bool>; 4], //clockwise from top
}

fn parse_number(input : &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_pixel_line(input: &str) -> IResult<&str, Vec<bool>> {
    map(
        recognize(many1(one_of("#."))), 
        |s: &str| s.chars().map(|c| c=='#').collect()
    )(input)
}

fn parse_raw_tiles(s: &str) -> Result<Vec<RawTile>> {
    let id_line = delimited(tag("Tile "), parse_number, tag(":"));
    let pixels = separated_list1(tag("\n"), parse_pixel_line);
    let field = map(
        separated_pair(id_line, tag("\n"), pixels),  
        |(id, pixels)| RawTile{id, pixels}
    );
    let mut tiles = separated_list1(tag("\n\n"), field);

    let res = tiles(s)
    .finish()
    .map(|(_, r)| r)
    // https://github.com/Geal/nom/blob/master/doc/nom_recipes.md#implementing-fromstr
    .map_err(|nom::error::Error { input, code }| nom::error::Error{input: input.to_string(), code});
    Ok(res?)
}

impl Tile {
    fn new(r: &RawTile) -> Self {
        Self{
            id: r.id,
            sides: [
                r.pixels[0].clone(), //top LR
                r.pixels.iter().map(|r| r[r.len()-1]).collect(), //right TB
                r.pixels[r.pixels.len()-1].iter().rev().cloned().collect(), //bot RL
                r.pixels.iter().rev().map(|r| r[0]).collect(), //left BT
            ]
        }
    }

    fn rotate_cw(&mut self) {self.sides.rotate_right(1);}

    fn flip(&mut self) {
        self.sides.reverse();
        for s in self.sides.iter_mut() {s.reverse();}
    }

    fn all(mut self) -> Vec<Self> {
        let mut v:Vec<Tile> = (0..4).scan((), |_,_| {self.rotate_cw(); Some(self.clone())}).collect();
        self.flip();
        v.extend((0..4).scan((), |_,_| {self.rotate_cw(); Some(self.clone())}));
        v
    }

    fn matches(&self, other: &Self, direction: usize) -> bool {
        self.sides[direction]
        .iter().rev()     
        .zip(other.sides[(direction+2)%4].iter())
        .all(|(&a, &b)| a==b)
    }
}


struct Game {
    tiles: Vec<Tile>,
    n: usize,
    n2: usize,
    ns: usize,
}

fn possible_tiles<'a>(game: &'a Game, board: &Vec<&'a Tile>) -> Vec<&'a Tile> {
    let idx = board.len();
    let used_ids: HashSet<_> = board.iter().map(|t| t.id).collect();
    let t_above = if idx>=game.n {Some((2usize, board[idx-game.n]))} else {None};
    let t_left = if (idx % game.n)>=1 {Some((1usize, board[idx-1]))} else {None};
    game
    .tiles
    .iter()
    .filter(|&t| !used_ids.contains(&t.id))
    .filter(|&t| t_above.iter().chain(t_left.iter()).all(|(direction, nb)| nb.matches(t, *direction)))
    .collect()
}

fn solve_rec<'a>(game: &'a Game, board: &mut Vec<&'a Tile>) -> Vec<Vec<&'a Tile>> {
    if board.len()==game.n2 {
        vec![board.clone()]
    } else {
        let mut res = Vec::new();
        let pos = possible_tiles(game, &board);
        for p in pos {
            board.push(p);
            res.extend(solve_rec(game, board));
            board.pop();
        }
        res
    }
}


fn check_game(game: &Game) {
    assert_eq!(game.n*game.n, game.n2);
    for t in game.tiles.iter() {
        if (!t.sides.len()==4) || (!t.sides.iter().all(|s| s.len()==game.ns)) {
            println!("{:?}", t);
        }
    }
    for t in game.tiles[..8].iter() {
        println!("{:?}", t);
    }
    assert!(game.tiles[0].matches(&game.tiles[5], 0));
}


fn main() -> Result<()> {
    let input = fs::read_to_string("input2.txt")?;
    let raw_tiles: Vec<RawTile> = parse_raw_tiles(&input[..])?;
    
    let game = Game {
        n2: raw_tiles.len(),
        n: (raw_tiles.len() as f64).sqrt() as usize,
        ns: raw_tiles[0].pixels.len(),
        tiles: raw_tiles.iter().map(Tile::new).map(|t| t.all()).concat()
    };
    assert_eq!(game.tiles.len(), raw_tiles.len()*8);
    // check_game(&game);

    let mut board = Vec::new();
    let solutions = solve_rec(&game, &mut board);
    println!("Part 1: {}", solutions.len());

    Ok(())
}
