#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use itertools::Itertools;
use ndarray::{s, ArrayView2, Array2, Array};
use std::fs;
// use ndarray_linalg::solve::Determinant; // compile errors galore 

fn compute_det2(m: &ArrayView2<i32>) -> i32 {
    m[[0,0]]*m[[1,1]] - m[[0,1]]*m[[1,0]]
}

fn make_rot(a: Vec<&[i32;3]>) -> Option<Array2<i32>> {
    let m = Array::from_iter(a.iter().flat_map(|r| r.iter()).cloned());
    let m = m.into_shape((3,3)).unwrap();
    // massive compile issues for ndarray-linalg
    // also ndarray-linalg only supports floats...
    let det = 
    m[[0,0]]*compute_det2(&m.slice(s![1.., 1..]))
    -m[[0,1]]*compute_det2(&m.slice(s![1.., 0..;2]))
    +m[[0,2]]*compute_det2(&m.slice(s![1.., ..2]));
    if det==1 {Some(m)} else {None}
}

fn solution(input_s: &str) -> Result<()> {
    let input: Vec<Vec<i32>> = input_s
        .trim()
        .split("\n\n")
        .map(|s| {
            s.split(&['\n', ','][..])
            .skip(1)
            .map(|v| v.parse())
            .collect::<Result<_,_>>()
            .with_context(|| format!("Parsing {}", s))
        })
        .collect::<Result<_,_>>()?;

    let pts: Vec<ArrayView2<i32>> = input.iter()
    .map(|r| ArrayView2::from_shape((r.len()/3, 3), r))
    .collect::<Result<_,_>>()?;

    //rotations
    // let units = [[0.,0.,1.],[0.,1.,0.],[1.,0.,0.],[0.,0.,-1.],[0.,-1.,0.],[-1.,0.,0.]];
    let units = [[0,0,1],[0,1,0],[1,0,0],[0,0,-1],[0,-1,0],[-1,0,0]];
    let rots: Vec<_>  = (0..3).map(|_| units.iter()).multi_cartesian_product().filter_map(make_rot).collect();
    assert_eq!(rots.len(), 24);

    println!("Part 1: {}", 0);
    println!("Part 2: {}", 0);
    Ok(())
}

fn main() -> Result<()> {
    println!("** TEST **");
    solution(&fs::read_to_string("test00.txt")?)?;
    println!("\n** INPUT **");
    solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
