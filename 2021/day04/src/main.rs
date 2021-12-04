use anyhow::{Result};
use itertools::Itertools;
use std::fs;
use std::collections::HashSet;


struct Board {
    lines: Vec<HashSet<u8>>,
}

impl Board {
    fn new(rows: &Vec<Vec<u8>>) -> Self {
        let n = rows[0].len();
        let cols: Vec<Vec<_>> = (0..n).map(|i| rows.iter().map(|row| row[i]).collect()).collect();
        let lines: Vec<HashSet<_>> = cols.iter().chain(rows.iter()).map(|v| v.iter().cloned().collect()).collect();
        Self {lines}
    }

    fn has_line(&self) -> bool {
        self.lines.iter().filter(|line| line.len()==0).next().is_some()
    }

    fn mark(&mut self, v: u8) {
        for line in self.lines.iter_mut() {
            line.remove(&v);
        }
    }

    fn sum_unmarked(&self) -> usize {
        self.lines.iter().flat_map(|line| line.iter()).map(|v| *v as usize).sum::<usize>()/2
    }
}

fn main() -> Result<()> {
    let input_s = fs::read_to_string("input.txt")?;
    let mut ii = input_s.trim().split("\n\n");
    let draws: Vec<u8> = ii.next().unwrap().split(",").map(|s| s.parse()).collect::<Result<_,_>>()?;
    let board_vectors: Vec<Vec<Vec<u8>>> = ii
        .map(|b| {
            b.split("\n").map(|ln| {
                ln.split_whitespace().map(|s| s.parse()).collect::<Result<_,_>>()
            }).collect::<Result<_,_>>()
        }).collect::<Result<_,_>>()?;
    println!("inputs: {}, {}", draws.len(), board_vectors.len());

    // Part 1
    let mut boards = board_vectors.iter().map(|b| Board::new(b)).collect_vec();
    for d in draws.iter() {
        for b in boards.iter_mut() { b.mark(*d); }
        if let Some(b) = boards.iter().filter(|b| b.has_line()).next() {
            println!("Part 1: {}", (*d as usize)*b.sum_unmarked());
            break;
        }
    }

    // Part 2
    let mut boards = board_vectors.iter().map(|b| Board::new(b)).collect_vec();
    for d in draws.iter() {
        for b in &mut boards {b.mark(*d)}
        // on unstable, https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain_filter
        // let winning_boards = boards.drain_filter(|b| b.has_line()).collect_vec();
        let mut winning_boards: Vec<Board> = Vec::new();
        let mut i = 0;
        while i < boards.len() {
            if boards[i].has_line() {
                winning_boards.push(boards.remove(i));
            } else {
                i += 1;
            }
        };
        // end of drain_filter
        if boards.len()==0 {
            println!("Part 2: {}", (*d as usize)*winning_boards[0].sum_unmarked());
            break;
        };
    }

    Ok(())
}
