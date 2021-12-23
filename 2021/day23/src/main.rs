#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use std::{fs, fmt};

// #############  #############    
// #...........#  #01.2.3.4.56#
// ###B#C#B#D###  ###0# # # ###    
//   #A#D#C#A#      # # # # #    
//   #########      #########    
//                   0 1 2 3

// dist from outside room i to pos in hall
// 2*pos ; 0 2 . 4 . 6 . 8 . 10 12
// 2*i+3       3   5   7   9
// dist = abs_diff(match pos {0=>1, 6=>11, _ => 2*pos}, 2*i+3)

fn distance(i: usize, j: usize, pos: usize) -> usize {
    j+1+abs_diff(match pos {0=>1, 6=>11, _ => 2*pos}, 2*i+3)
}

#[derive(Debug, Clone)]
struct Board {
    rooms: [[u8;4];4], //[room][slot]
    hall: [u8;7],
}

fn byte_for(i: u8) -> u8 {
    if i==0 {b'.'} else {i+b'A'-1}
}

impl fmt::Display for Board {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let b = [
            b"#############",
            b"#...........#",
            b"###.#.#.#.###",
            b"  #.#.#.#.#  ",
            b"  #.#.#.#.#  ",
            b"  #.#.#.#.#  ",
            b"  #########  ",
        ];
        let mut buf: Vec<Vec<u8>> = b.iter().map(|r| r.iter().cloned().collect()).collect();
        for i in 0..4 {
            for j in 0..4 {
                buf[2+j][3+2*i] = byte_for(self.rooms[i][j]);
            }
        }
        for (pos, idx) in (0..7).zip([1,2,4,6,8,10,11].iter()) {
            buf[1][*idx] = byte_for(self.hall[pos]);
        }
        for row in buf.iter() {
            write!(f, "{}\n", std::str::from_utf8(row).unwrap())?;
        }
        Ok(())
    }
}

fn uc(i: usize) -> usize {
    match i {
        1 => 1,
        2 => 10, 
        3 => 100,
        4 => 1000,
        _ => panic!(),
    }
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {a - b} else {b - a}
}

impl Board {
    fn new(rooms: [[u8;4];4]) -> Self {
        Board{rooms, hall: [0;7]}
    }

    fn move_in(&self, i: usize, pos: usize) -> (usize, Self) {
        let mut b = self.clone();
        b.hall[pos] = 0;
        for j in (0..4).rev() {
            if b.rooms[i][j] == 0 {
                b.rooms[i][j] = 1 + i as u8;
                return (uc(i+1)*distance(i,j,pos), b)
            }
        }
        panic!();
    }

    fn move_out(&self, i: usize, pos: usize) -> (usize, Self) {
        let mut b = self.clone();
        for j in 0..4 {
            if b.rooms[i][j] > 0 {
                b.hall[pos] = b.rooms[i][j];
                b.rooms[i][j] = 0;
                return (uc(b.hall[pos] as usize)*distance(i,j,pos), b)
            }
        }
        panic!()
    }
}

fn solve(b: &Board) -> Option<usize> {
    let mut ready_count = 0;
    (0..4).filter_map(|i| {
        let ready = b.rooms[i].iter().all(|&v| v==0 || v== i as u8 +1);
        if ready {ready_count+=1; solve_in(b, i) } else {solve_out(b, i)}
    })
    .min()
    .or_else(|| if ready_count==4 {Some(0)} else {None})
}

fn solve_in(b: &Board, i: usize) -> Option<usize> {
    let left = (0..=(i+1)).rev().zip(b.hall[0..=(i+1)].iter().rev().cloned()).filter(|(pos, v)| v>&0).next();
    let right = ((i+2)..7).zip(b.hall[(i+2)..7].iter().cloned()).filter(|(pos, v)| v>&0).next();
    [left, right].into_iter()
    .filter_map(|x| x)
    .filter_map(|(pos, v)|  if v==i as u8+1 {
        let (cost, board_new) = b.move_in(i, pos);
        solve(&board_new).and_then(|c| Some(c+cost))
    } else { None }
    ).min()
}

fn solve_out(b: &Board, i: usize) -> Option<usize> {
    let left = (0..(i+2)).rev().zip(b.hall[0..(i+2)].iter().rev().cloned()).take_while(|(pos, v)| v==&0);
    let right = ((i+2)..7).zip(b.hall[(i+2)..7].iter().cloned()).take_while(|(pos, v)| v==&0);
    left.chain(right)
    .filter_map(|(pos, v)| {
        let (cost, board_new) = b.move_out(i, pos);
        solve(&board_new).and_then(|c| Some(c+cost))
    })
    .min()
}



fn solution(input_s: &str) -> Result<()> {
    let input: Vec<u8> = input_s
    .trim().split("\n").skip(2)
    .take(2)
    .flat_map(|ln| ln.as_bytes()[3..10].iter().step_by(2).map(|b| (*b-b'A'+1))).collect();

    //part 1
    let mut rooms =  [[0u8;4];4];
    for i in 0..4 {
        let v = &mut rooms[i];
        v[0] = input[i];
        v[1] = input[i+4];
        v[2] = i as u8+1;
        v[3] = i as u8+1;
    }
    let board = Board::new(rooms);
    println!("{}", &board);
    // let (c, board) = board.move_out(2, 2);
    // println!("Cost {}->\n{}", c, &board);
    // let (c1, board) = board.move_out(1, 3);
    // let (c2, board) = board.move_in(2, 3);
    // println!("Cost {}->\n{}", c1+c2, &board);
    println!("Part 1: {}", solve(&board).unwrap());

    //part2
    let mut rooms =  [[0u8;4];4];
    let fill: Vec<u8> = "DCBADBAC".as_bytes().iter().map(|b| (*b-b'A'+1)).collect();
    for i in 0..4 {
        let v = &mut rooms[i];
        v[0] = input[i];
        v[1] = fill[i];
        v[2] = fill[i+4];
        v[3] = input[i+4];
    }
    let board = Board::new(rooms);
    println!("{}", &board);
    //println!("Part 2: {}", solve(&board).unwrap());

    Ok(())
}

fn main() -> Result<()> {
    println!("** TEST **");
    solution(&fs::read_to_string("test00.txt")?)?;
    println!("\n** INPUT **");
    solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
