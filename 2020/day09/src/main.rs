use std::{collections::HashSet, fs};
use anyhow::{Error, Result};
use itertools::iterate;


fn xmas(c: usize, v: &[usize]) -> bool {
    let s: HashSet<usize> = v.iter().cloned().collect();
    v.iter().find_map(|v0| c.checked_sub(*v0).and_then(|v1| s.get(&v1))).is_some()
}

fn main() -> Result<()> {
    let input: Vec<usize> = fs::read_to_string("input.txt")?
    .lines().map(|s| s.parse::<usize>())
    .collect::<Result<_,_>>()?;
 
    let n = 25usize;

    let p1: usize = input.windows(n+1)
    .find_map(|v| if !xmas(v[n], &v[..n]) {Some(v[n])} else {None})
    .ok_or(Error::msg("No solution for part 1"))?;

    println!("Part 1: {}", p1);

    println!("Part 2: {}", 
        iterate(
            (0,0,0),
            |(i, j, s)| if *s<p1 {
                (*i, j+1, *s+input[*j])
            } else {
                (i+1, *j, *s-input[*i])
            } 
        ).find(|(_, _, s)| *s==p1)
        .map(|(i, j, _)| input[i..j].iter().min().unwrap()
                    +input[i..j].iter().max().unwrap())
        .unwrap_or(0));

    Ok(())
}
