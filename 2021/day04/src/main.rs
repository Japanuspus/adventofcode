use anyhow::{Result};
use itertools::Itertools;
use std::fs;
use std::collections::HashSet;


struct Board {
    rows: Vec<Vec<u8>>,
    lines: Vec<HashSet<u8>>,
}

impl Board {
    fn new(rows: Vec<Vec<u8>>) -> Self {
        let n = rows[0].len();
        let cols: Vec<Vec<_>> = (0..n).map(|i| rows.iter().map(|row| row[i]).collect()).collect();
        let lines: Vec<HashSet<_>> = cols.iter().chain(rows.iter()).map(|v| v.iter().cloned().collect()).collect();
        Self {rows, lines}
    }

    fn has_match(&self, drawn: &HashSet<u8>) -> bool {
        self.lines.iter().filter(|line| drawn.is_superset(line)).next().is_some()
    }
}

fn find_board<'a>(boards: &'a Vec<Board>, draws: &Vec<u8>) -> Option<(usize, &'a Board)> {
    let mut drawn: HashSet<u8> = HashSet::new();
    for (i_draw, d) in draws.iter().cloned().enumerate() {
        drawn.insert(d);
        for board in boards.iter() {
            if board.has_match(&drawn) {
                return Some((i_draw, board));
            }
        }   
    }
    None
}

fn find_last_board<'a>(boards: &'a Vec<Board>, draws: &Vec<u8>) -> Option<(usize, &'a Board)> {
    let mut drawn: HashSet<u8> = HashSet::new();
    let mut rboards: Vec<&Board> = boards.iter().collect_vec();

    for (i_draw, d) in draws.iter().cloned().enumerate() {
        drawn.insert(d);
        let new_boards = rboards.iter().cloned().filter(|b| !b.has_match(&drawn)).collect_vec();
        if new_boards.len()==0 {
            if rboards.len()>1 {
                panic!("More than one board gone in last round");
            }
            return Some((i_draw, rboards[0]));
        }
        rboards=new_boards;
    }
    None
}

fn score_board(draws: &Vec<u8>, draw_board: (usize, &Board)) -> usize {
    let board = draw_board.1;
    let picked = draws.iter().take(draw_board.0+1).cloned().collect::<HashSet<_>>();
    let last_pick = draws[draw_board.0];
    let board_all = board.rows.iter().flat_map(|r| r.iter()).cloned().collect::<HashSet<_>>();
    (last_pick as usize) * board_all.difference(&picked).map(|v| *v as usize).sum::<usize>()
}

fn main() -> Result<()> {
    let input_s = fs::read_to_string("input.txt")?;
    let mut ii = input_s.trim().split("\n\n");
    let draws: Vec<u8> = ii.next().unwrap().split(",").map(|s| s.parse()).collect::<Result<_,_>>()?;
    let boards: Vec<Board> = ii
        .map(|b| {
            b
            .split("\n").map(|ln| {
                ln.split_whitespace().map(|s| s.parse()).collect::<Result<_,_>>()
            })
            .collect::<Result<_,_>>()
            .and_then(|vv| Ok(Board::new(vv)))
        }).collect::<Result<_,_>>()?;
    println!("inputs: {}, {}", draws.len(), boards.len());

    let draw_board = find_board(&boards, &draws).unwrap();
    println!("Part 1: {}", score_board(&draws, draw_board));

    println!("Part 2: {}", score_board(&draws, find_last_board(&boards, &draws).unwrap()));
    Ok(())
}    