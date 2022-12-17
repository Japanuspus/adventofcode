#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use std::{fs, time::Instant, slice::Windows, collections::HashMap};

type Block = [u8;4];

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
    // This could be u32-operations with bitfields?
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

    let input_period = BLOCKS.len()*input.len();
    // last entry is top of tower
    // MSB is not used. Next 7 bits are blocks
    let mut tower: Vec<u8> = Vec::new();
    let mut winds = input.iter().cycle();
    let mut blocks = BLOCKS.iter().cycle().cloned();
    let mut epoch_states: HashMap<[u8;32], (usize, usize)> = HashMap::new(); // signature -> (n_bl, tower.len())
    let mut heights: Vec<usize> = Vec::new();
    let (n1, n2, dh12) = loop {
        let mut bl = blocks.next().unwrap();
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
            match y.checked_sub(1) {
                Some(next_y) if tower.iter().skip(next_y).zip(bl.iter()).all(|(t, b)| t&b==0) => {y=next_y;},
                _ => {break;},
            }   
        }
        // tower insert
        let mut bl_rows = bl.into_iter().take_while(|b| *b>0);
        for (r,b) in tower.iter_mut().skip(y).zip(bl_rows.by_ref()) {*r|=b;}
        tower.extend(bl_rows);

        // look for cycles in height...
        heights.push(tower.len());
        if heights.len() % input_period == 0 {
            // state "hash": topmost 8 rows as u64...
            // let hash = tower.iter().rev().take(10).fold(0u64, |a, b| (a<<7)|(*b as u64));
            // not enough state...
            let top: [u8;32] = tower.iter().rev().take(32).cloned().collect::<Vec<_>>().try_into().unwrap();
            if let Some((last_n_bl, last_h)) = epoch_states.insert(top, (heights.len(), tower.len())) {
                let n_bl = heights.len();
                println!("Found period: nb {}->{}, dh={}", last_n_bl, n_bl, tower.len()-last_h);
                break (last_n_bl, n_bl, tower.len()-last_h);
            }
        }
    };

    let ns = [2022usize, 1_000_000_000_000];
    let n12 = n2-n1;
    let hs: Vec<String> = ns.iter().map(|n| {
        let h = if *n>heights.len() {
            let shifted_r = (n-n1)%n12;
            let shifted_q = (n-n1)/n12;
            heights[shifted_r-1+n1] + shifted_q*dh12    
        } else {
            heights[n-1]
        };
        format!("{}",h)
    }).collect();

    Ok(hs.try_into().unwrap())
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test01.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "3068");
    assert!(res[1] == "1514285714288");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    //for _ in 0..20 {solution(&input)?;} //warmup
    let start = Instant::now();
    let res = solution(&input)?;
    println!(
        "({} us)\nPart 1: {}\nPart 2: {}",
        start.elapsed().as_micros(), res[0], res[1],
    );
    Ok(())
}
