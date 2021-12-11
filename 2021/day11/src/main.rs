use anyhow::{Result};
use std::fs;

fn idx(u: &(i8, i8)) -> Option<usize> {
    if 0<=u.0&&u.0<10&&0<=u.1&&u.1<10 {Some((u.0*10+u.1) as usize)} else {None}
}

fn get(b: &[i8],u: &(i8, i8)) -> Option<i8> {
    idx(u).and_then(|i| Some(b[i]))
}

fn prop(b: &mut [i8]) -> usize {
    let dirs: [(i8, i8);8] = [(1,0), (1,1), (0,1), (-1,1), (-1,0),(-1, -1), (0, -1), (1, -1)];
    let mut work: Vec<(i8, i8)> = Vec::new();
    for i in 0..10 {
        for j in 0..10 {
            let ij = idx(&(i,j)).unwrap();
            if b[ij]==9 {
                work.push((i,j)); //will increase
            } else {
                b[ij]+=1;
            }
        }
    }
    let mut flashes: usize = 0;
    while let Some((i, j)) = work.pop() {
        if let Some(idx) = idx(&(i, j)) {
            if b[idx]>0 {b[idx]+=1};
            if b[idx]>9 {
                flashes+=1;
                b[idx]=0;    
                work.extend(dirs.iter().map(|d| (d.0+i, d.1+j)));
            }
        }
    };
    flashes
}

fn print_board(b: &[i8]) {
    println!();
    for i in 0..10 {
        for j in 0..10 {
            print!("{}", get(b, &(i,j)).unwrap())
        }
        println!();
    }
}

fn solution(input_s: &str) -> Result<()> {
    let input: Vec<i8> = input_s
        .trim()
        .split("\n")
        .flat_map(|s| s.as_bytes().iter().map(|c| (*c-b'0') as i8))
        .collect();

    let mut p1=0;
    let mut board = input.clone();
    for _ in 0..100 {
        p1 += prop(&mut board);
    }
    //print_board(&board);
    println!("Part 1: {}", p1);

    let mut board = input.clone();
    let mut p2=0;
    loop {
        p2+=1;
        if prop(&mut board)==100 {
            break;
        }
    }
    println!("Part 2: {}", p2);
    Ok(())
}

fn main() -> Result<()> {
    println!("** TEST **");
    solution(&fs::read_to_string("test00.txt")?)?;
    println!("\n** INPUT **");
    solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
