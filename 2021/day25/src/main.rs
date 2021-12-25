#![allow(dead_code)]

use anyhow::{Result, Context};
use ndarray::{Array, Array2, ArrayViewMut2, ArrayView2};
use std::fs;

fn move_east(a: &mut ArrayViewMut2<u8>, b: &ArrayView2<u8>) -> bool {
    let m = a.shape()[0];
    let n = a.shape()[1];
    let mut moved = false;
    for i in 0..m {
        let mut ra = a.row_mut(i);
        let rb = b.row(i);
        let mut t = ra[n-1];
        let mut it = n-1;
        let mut carry = false;
        for j in 0..n {
            let t2=ra[j];
            if t>0 && ra[j]==0 && rb[j]==0 {
                moved = true;
                ra[j] = 1;
                if j==0 {
                    carry = true;
                } else {
                    ra[it] = 0;
                }
            }
            t = t2;
            it = j;
        }
        if carry {
            ra[n-1] = 0;
        }
    }
    moved
}

fn make_array(v: &Vec<Vec<u8>>, c: u8) -> Result<Array2<u8>> {
    let shape = (v.len(), v[0].len());
    let data = v.iter().flat_map(|row| row.iter().map(|cc| if *cc == c {1} else {0})).collect();
    Array::from_vec(data).into_shape(shape).context("Making array")
}

fn solution(input_s: &str) -> Result<()> {
    let input: Vec<Vec<u8>> = input_s
        .trim()
        .split("\n")
        .map(|s| s.as_bytes().iter().cloned().collect())
        .collect();
    let mut east: Array2::<u8> = make_array(&input, b'>')?;
    let mut south = make_array(&input, b'v')?;
    
    let mut i = 0usize;
    loop {
        i+=1;
        let b1 = move_east(&mut east.view_mut(), &south.view());
        let b2 = move_east(&mut south.view_mut().reversed_axes(), &east.view().reversed_axes());
        if !b1 && !b2 {break}
    }
    println!("Part 1: {}", i);
    Ok(())
}

fn main() -> Result<()> {
    println!("** TEST **");
    solution(&fs::read_to_string("test_man.txt")?)?;
    println!("\n** INPUT **");
    solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
