use anyhow::{Result};
use ndarray::{Array2, s};
use std::fs;
use bitvec::prelude::{BitVec};


fn read(m: &Array2<u8>, i: usize, j: usize) -> usize {
    let v = m.slice(s![i..i+3, j..j+3]);
    v.iter()
    .fold(0usize, |mut a, x| {a<<=1; if *x>0 {a|=1}; a})
}

fn apply(m: &Array2<u8>, alg: &BitVec, fill_addr: usize) -> Array2<u8> {
    let fill_value: u8 = if alg[fill_addr] {1} else {0};
    let mut m2 = Array2::from_elem(m.raw_dim(), fill_value);
    let d = m.raw_dim();
    let n_i = d[0]-2;
    let n_j = d[1]-2;
    for i in 0..n_i {
        for j in 0..n_j {
            m2[[i+1, j+1]]=if alg[read(m, i, j)] {1} else {0};
        }
    }
    m2
}

fn apply_twice(m0: &Array2<u8>, alg: &BitVec) -> Array2<u8> {
    let fill_addr_0 = 0;
    let m1 = apply(&m0, &alg, fill_addr_0);
    let fill_addr_1 = if alg[fill_addr_0] {0b111_111_111} else {0};
    let m2 = apply(&m1, &alg, fill_addr_1);
    m2
}

fn solution(input_s: &str) -> Result<()> {
    let (alg_s, m_s) = input_s.trim().split_once("\n\n").unwrap();
    let m_v: Vec<Vec<u8>> = m_s
        .split("\n")
        .map(|ln| ln.chars().map(|c| if c=='#' {1} else {0}).collect())
        .collect();

    // alg as bitvec
    let alg: BitVec = alg_s.chars().map(|c| c=='#').collect();
    assert_eq!(alg.len(), 512);

    // map as Array2<u8> with padding
    // TODO: if allocating everything dynamically, size could grow 1 per round instead of pre-padding
    // TODO: better alternative: pre-allocate two buffers and move back and forth
    let pad = 2+50;
    let n_i = m_v.len()+2*pad;
    let n_j = m_v[0].len()+2*pad;
    let mut m0: Array2<u8> = Array2::zeros((n_i, n_j));
    for (i, row) in m_v.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            m0[[i+pad, j+pad]] = *val;
        }
    }
    let m0 = m0;
    //dbg!(&m0);
    let m1 = apply_twice(&m0, &alg);
    //dbg!(&m1);

    println!("Part 1: {}", m1.iter().filter(|&v| *v>0).count());

    let m2 = (0..25).fold(m0, |m, _| apply_twice(&m, &alg));
    println!("Part 2: {}", m2.iter().filter(|&v| *v>0).count());
    Ok(())
}

fn main() -> Result<()> {
    println!("** TEST **");
    solution(&fs::read_to_string("test00.txt")?)?;
    println!("\n** INPUT **");
    solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
