use std::{collections::HashSet, fs};
use anyhow::Result;
use std::iter::FromIterator;
// use itertools::Itertools;
// use apply::Also;


fn xmas(c: usize, v: &[usize]) -> bool {
    let s: HashSet<usize> = HashSet::from_iter(v.iter().cloned());
    v.iter().find_map(|v0| c.checked_sub(*v0).and_then(|v1| s.get(&v1))).is_some()
}

fn main() -> Result<()> {
    let input: Vec<usize> = fs::read_to_string("input.txt")?
    .lines()
    .map(|s| s.parse::<usize>())
    .collect::<Result<_,_>>()?;
 
    let n = 25usize;
    println!("Part 1: {}", 
    input.windows(n+1)
    .find_map(|v| if !xmas(v[n], &v[..n]) {Some(v[n])} else {None})
    .unwrap_or(0));

    Ok(())
}
