use anyhow::{Result}; //, Error, anyhow};
use std::fs;
use std::collections::HashSet;

// use parse_display::{Display, FromStr};

// #[derive(Display, FromStr, PartialEq, Debug)]
// enum Direction {
//     #[display("forward")]
//     Forward,
// }

// #[derive(Debug, FromStr)]
// #[display("{direction} {distance}")]
// struct Step {
//     direction: Direction,
//     distance: i32,
// }

// struct Line {
//     board: isize,
//     members: HashSet<u8>,
// }


struct Board {
    rows: Vec<Vec<u8>>,
    cols: Vec<Vec<u8>>,
    lines: Vec<HashSet<u8>>,
}

impl Board {
    fn new(rows: Vec<Vec<u8>>) -> Self {
        let n = rows[0].len();
        let cols: Vec<Vec<_>> = (0..n).map(|i| rows.iter().map(|row| row[i]).collect()).collect();
        let lines: Vec<HashSet<_>> = cols.iter().chain(rows.iter()).map(|v| v.iter().cloned().collect()).collect();
        Self {rows, cols, lines}
    }
}

fn find_board(boards: &Vec<Board>, draws: &Vec<u8>) -> Option<(usize, usize)> {
    let mut drawn: HashSet<u8> = HashSet::new();
    for (i_draw, d) in draws.iter().cloned().enumerate() {
        drawn.insert(d);
        for (i, board) in boards.iter().enumerate() {
            for line in board.lines.iter() {
                if drawn.is_superset(line) {
                    return Some((i_draw, i));
                }
            }
        }   
    }
    None
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
    let board = &boards[draw_board.1];
    let picked = draws.iter().take(draw_board.0+1).cloned().collect::<HashSet<_>>();
    let last_pick = draws[draw_board.0];
    let board_all = board.rows.iter().flat_map(|r| r.iter()).cloned().collect::<HashSet<_>>();
    let p1: usize = (last_pick as usize) * board_all.difference(&picked).map(|v| *v as usize).sum::<usize>();
    println!("Part 1: {}", p1);
    //println!("Part 2: {}", input.len());
    Ok(())
}    