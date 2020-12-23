#![allow(dead_code)]

use anyhow::Result;
// use apply::Apply;
use itertools::{iterate, Itertools};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, one_of};
use nom::combinator::{map, map_res, recognize};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, separated_pair};
use nom::{character::complete::multispace1, Finish, IResult};
use std::{collections::HashSet, fs};

#[derive(Debug, Clone)]
struct RawTile {
    id: usize,
    pixels: Vec<Vec<bool>>,
}

fn transpose_vectors<T>(mut vectors: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut res = Vec::new();
    let mut iters = vectors.iter_mut().map(|v| v.drain(..)).collect::<Vec<_>>();
    while let Some(v) = iters.iter_mut().map(|i| i.next()).collect() {
        res.push(v);
    }
    res
}

impl RawTile {
    fn flip(&mut self) {
        self.pixels.reverse();
    }

    fn rotate(&mut self) {
        self.pixels = transpose_vectors(self.pixels.clone());
        self.flip();
    }
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_pixel_line(input: &str) -> IResult<&str, Vec<bool>> {
    map(recognize(many1(one_of("#."))), |s: &str| {
        s.chars().map(|c| c == '#').collect()
    })(input)
}

fn parse_raw_tiles(s: &str) -> Result<Vec<RawTile>> {
    let id_line = delimited(tag("Tile "), parse_number, tag(":"));
    let pixels = separated_list1(multispace1, parse_pixel_line);
    let field = map(
        separated_pair(id_line, multispace1, pixels),
        |(id, pixels)| RawTile { id, pixels },
    );
    let mut tiles = separated_list1(multispace1, field);

    let res = tiles(s)
        .finish()
        .map(|(_, r)| r)
        // https://github.com/Geal/nom/blob/master/doc/nom_recipes.md#implementing-fromstr
        .map_err(|nom::error::Error { input, code }| nom::error::Error {
            input: input.to_string(),
            code,
        });
    Ok(res?)
}

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    pixels: Vec<Vec<bool>>, // includes edges
    sides: [Vec<bool>; 4],  //clockwise from top
}

impl Tile {
    fn new(r: RawTile) -> Self {
        let sides = [
            r.pixels[0].clone(),                                          //top LR
            r.pixels.iter().map(|r| r[r.len() - 1]).collect(),            //right TB
            r.pixels[r.pixels.len() - 1].iter().rev().cloned().collect(), //bot RL
            r.pixels.iter().rev().map(|r| r[0]).collect(),                //left BT
        ];
        Self {
            id: r.id,
            pixels: r.pixels,
            sides,
        }
    }

    /// other matches in direction from self
    /// direction 0: other matches above self
    fn matches(&self, other: &Self, direction: usize) -> bool {
        self.sides[direction]
            .iter()
            .rev()
            .zip(other.sides[(direction + 2) % 4].iter())
            .all(|(&a, &b)| a == b)
    }
}

fn all_symmetric(r: &RawTile) -> Vec<Tile> {
    let mut v: Vec<Tile> = Vec::new();
    let mut buf = r.clone();
    for _ in 0..2 {
        for _ in 0..4 {
            v.push(Tile::new(buf.clone()));
            buf.rotate();
        }
        buf.flip();
    }
    v
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
    let t_above = if idx >= game.n {
        Some((2usize, board[idx - game.n]))
    } else {
        None
    };
    let t_left = if (idx % game.n) >= 1 {
        Some((1usize, board[idx - 1]))
    } else {
        None
    };
    game.tiles
        .iter()
        .filter(|&t| !used_ids.contains(&t.id))
        .filter(|&t| {
            [t_above, t_left]
                .iter()
                .filter_map(|onb| onb.as_ref())
                .all(|(dir, nb)| nb.matches(t, *dir))
        }) //).iter().chain(t_left.iter()).all(|(direction, nb)| nb.matches(t, *direction)))
        .collect()
}

fn print_tile(t: &Tile) {
    print!("{:05}: ", t.id);
    for v in t.sides.iter() {
        print!(
            "{}, ",
            v.iter().map(|b| if *b { '#' } else { '.' }).join("")
        )
    }
    println!();
}

fn solve_rec<'a>(game: &'a Game, board: &mut Vec<&'a Tile>) -> Vec<Vec<&'a Tile>> {
    if board.len() == game.n2 {
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
    assert_eq!(game.n * game.n, game.n2);
    for t in game.tiles.iter() {
        if (!t.sides.len() == 4) || (!t.sides.iter().all(|s| s.len() == game.ns)) {
            println!("{:?}", t);
        }
    }
    assert!(game.tiles[0].matches(&game.tiles[4], 0));
}

fn find_monsters(solution: &Vec<Vec<bool>>, monster: &Vec<Vec<bool>>) -> Option<usize> {
    let sm = solution.len();
    let sn = solution[0].len();
    let mm = monster.len();
    let mn = monster[0].len();

    let mut monster_count = 0;
    for i in 0..(sm - mm + 1) {
        for j in 0..(sn - mn + 1) {
            // check mask at i,j
            if solution[i..(i + mm)]
                .iter()
                .zip(monster)
                .flat_map(|(row, mrow)| row[j..(j + mn)].iter().zip(mrow))
                .all(|(pixel_val, monster_val)| !(monster_val & !pixel_val))
            {
                println!("Monster at {}, {}", i, j);
                monster_count += 1;
            }
        }
    }

    fn count_true(v: &Vec<Vec<bool>>) -> usize {
        v.iter().map(|r| r.iter().filter(|&v| *v).count()).sum()
    }

    if monster_count > 0 {
        let n_solution = count_true(solution);
        let n_monster = count_true(monster);
        let res = n_solution - monster_count * n_monster;
        println!(
            "Roughness: {} - {}*{} = {}",
            n_solution, monster_count, n_monster, res
        );
        Some(res)
    } else {
        None
    }
}

fn solution_bitmap(game: &Game, s: Vec<&Tile>) -> Vec<Vec<bool>> {
    s.chunks_exact(game.n)
        .flat_map(|row| {
            (1..(game.ns - 1)).map(move |ri| {
                row.iter()
                    .flat_map(|t| t.pixels[ri][1..(game.ns - 1)].iter())
                    .cloned()
                    .collect()
            })
        })
        .collect()
}

fn print_bitmap(b: &Vec<Vec<bool>>) {
    for row in b.iter() {
        println!(
            "{}",
            row.iter()
                .map(|c| if *c { '#' } else { '.' })
                .collect::<String>()
        );
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let raw_tiles: Vec<RawTile> = parse_raw_tiles(&input[..])?;

    let game = Game {
        n2: raw_tiles.len(),
        n: (raw_tiles.len() as f64).sqrt() as usize,
        ns: raw_tiles[0].pixels.len(),
        tiles: raw_tiles.iter().map(|t| all_symmetric(t)).concat(),
    };
    assert_eq!(game.tiles.len(), raw_tiles.len() * 8);
    check_game(&game);

    let mut board = Vec::new();
    let solutions = solve_rec(&game, &mut board);
    println!(
        "Part 1: {}",
        [0, game.n - 1, game.n * (game.n - 1), game.n2 - 1]
            .iter()
            .map(|&i| solutions[0][i].id)
            .product::<usize>()
    );

    print_bitmap(&solution_bitmap(&game, solutions[0].clone()));

    let monster_s: &str = "..................#.
     #....##....##....###
     .#..#..#..#..#..#...";
    let monster: Vec<Vec<bool>> = monster_s
        .lines()
        .map(|ln| {
            ln.chars()
                .filter_map(|c| {
                    if c == '#' {
                        Some(true)
                    } else {
                        if c == '.' {
                            Some(false)
                        } else {
                            None
                        }
                    }
                })
                .collect()
        })
        .collect();

    for s in solutions {
        find_monsters(&solution_bitmap(&game, s), &monster);
    }
    Ok(())
}
