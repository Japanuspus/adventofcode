#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use std::{fs, time::Instant};

type Block = [u8;4];
// fn rot_r(bl: &mut Block, n: u32) {
//     bl.iter_mut().map(|b| b.rotate_right(n))
// }

// blocks in binary encoding. lowest row first
const BLOCKS: [[u8;4];5] = [
    // ####
    [0b1111_0000, 0, 0, 0],
    // .#.
    // ###
    // .#.
    [0b0100_0000, 0b1110_0000, 0b0100_0000, 0b0000_0000],
    // ..#
    // ..#
    // ###
    [0b1110_0000, 0b0010_0000, 0b0010_0000, 0b0000_0000],
    // #
    // #
    // #
    // #
    [0b1000_0000, 0b1000_0000, 0b1000_0000, 0b1000_0000],

    // ##
    // ##
    [0b1100_0000, 0b1100_0000, 0b0000_0000, 0b0000_0000],
];


fn shift_block(bl: &mut Block, dir: u8) {
    match dir {
        b'<' => if bl.iter().all(|b| (b&0b0100_0000)==0) {
                for b in bl.iter_mut() {*b=*b<<1;};
            },
        b'>' => if bl.iter().all(|b| (b&0b0000_0001)==0) {
                for b in bl.iter_mut() {*b=*b>>1;};
            },
        _ => panic!(),
    }
}

fn shifted_block(bl: & Block, dir: u8)  -> Block {
    let mut b = bl.clone();
    shift_block(&mut b, dir);
    b
}


#[test]
fn test_rot_block() {
    let mut b = BLOCKS[1].clone(); //+
    shift_block(&mut b, b'<');
    assert!(b==BLOCKS[1]);
    shift_block(&mut b, b'>');
    assert!(b==[0b0010_0000, 0b0111_0000, 0b0010_0000, 0b0000_0000]);
}


fn print_tower(t: &Vec<u8>) {
    for (y, ln) in t.iter().enumerate().rev() {
        println!("{:4} >{:08b}", y, ln);
    }
}


fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: &[u8] = input_s.trim().as_bytes();

    // last entry is top of tower
    // MSB is not used. Next 7 bits are blocks
    let mut tower: Vec<u8> = Vec::new();
    let mut winds = input.iter().cycle();
    for block in BLOCKS.iter().cycle().take(2022) {
        let mut bl = block.clone();
        for b in bl.iter_mut() {*b=*b>>3;}; // initial pos, free of MSB
        // Each rock appears .. its bottom edge is three units above the highest rock in the room (or the floor, if there isn't one).
        let mut y:usize = tower.len()+3;
        loop {
            // shift sideways
            let next_bl = shifted_block(&bl, *winds.next().unwrap());
            if tower.iter().skip(y).zip(next_bl.iter()).all(|(t, b)| t&b==0) {
                bl=next_bl;
            }
            // shift down
            if let Some(next_y) = y.checked_sub(1) {
                if tower.iter().skip(next_y).zip(bl.iter()).all(|(t, b)| t&b==0) {
                    y=next_y;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        // tower insert
        // println!("Insert at {}", y);
        let mut bl_rows = bl.into_iter().take_while(|b| *b>0);
        for (r,b) in tower.iter_mut().skip(y).zip(bl_rows.by_ref()) {*r|=b;}
        tower.extend(bl_rows);        
    }
    if tower.len()<50 {print_tower(&tower);};
    let part1 = tower.len();
    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test01.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "3068");
    assert!(res[1] == "0");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    for _ in 0..20 {solution(&input)?;} //warmup
    let start = Instant::now();
    let res = solution(&input)?;
    println!(
        "({} us)\nPart 1: {}\nPart 2: {}",
        start.elapsed().as_micros(), res[0], res[1],
    );
    Ok(())
}


// // Make it simple to compare timing for multiple solutions
// type Solution = dyn Fn(&str) -> Result<[String; 2]>;
// const SOLUTIONS: [(&str, &Solution); 1] = [("Original", &solution)];

// #[test]
// fn test_solution() -> Result<()> {
//     let input = &fs::read_to_string("test00.txt")?;
//     for (name, solution) in SOLUTIONS {
//         let res = solution(&input).with_context(|| format!("Running solution {}", name))?;
//         println!("---\n{}\nPart 1: {}\nPart 2: {}", name, res[0], res[1]);
//         assert!(res[0] == "0");
//         assert!(res[1] == "0");
//     }
//     Ok(())
// }

// fn main() -> Result<()> {
//     let input = &fs::read_to_string("input.txt")?;
//     for (_, solution) in SOLUTIONS.iter().cycle().take(10) {
//         solution(&input)?;
//     } //warmup
//     for (name, solution) in SOLUTIONS {
//         let start = Instant::now();
//         let res = solution(&input)?;
//         println!(
//             "---\n{} ({} us)\nPart 1: {}\nPart 2: {}",
//             name, start.elapsed().as_micros(), res[0], res[1],
//         );
//     }
//     Ok(())
// }
