use anyhow::{Result};
use std::fs;
use ndarray::{ArrayView2, ArrayViewMut2};

fn prop(b: &mut [i8]) -> Result<usize> {
    let dirs: [(i8, i8);8] = [(1,0), (1,1), (0,1), (-1,1), (-1,0),(-1, -1), (0, -1), (1, -1)];
    let mut m = ArrayViewMut2::from_shape((10, 10), b)?;
    let mut work: Vec<(i8, i8)> = Vec::new();
    for i in 0..10 {
        for j in 0..10 {
            if m[[i, j]]==9 {
                work.push((i as i8, j as i8)); //will increase
            } else {
                m[[i, j]]+=1;
            }
        }
    }
    let mut flashes: usize = 0;
    while let Some((i, j)) = work.pop() {
        if 0<=i&&i<10&&0<=j&&j<10 {
            let idx = [i as usize, j as usize];
            if m[idx]>0 {m[idx]+=1};
            if m[idx]>9 {
                flashes+=1;
                m[idx]=0;    
                work.extend(dirs.iter().map(|d| (d.0+i, d.1+j)));
            }
        }
    };
    Ok(flashes)
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
        p1 += prop(&mut board)?;
    }
    println!("{:?}", ArrayView2::from_shape((10,10), &board)?);
    println!("Part 1: {}", p1);

    let mut board = input.clone();
    let mut p2=0;
    loop {
        p2+=1;
        if prop(&mut board)?==100 {
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
