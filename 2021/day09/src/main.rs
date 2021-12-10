use anyhow::{Result};
use itertools::Itertools;
use std::fs;
use std::collections::{HashMap, HashSet};

fn fill_size(map: &HashMap<(i32, i32), i8>, loc_val: ((i32, i32), i8)) -> usize {
    let dirs: [(i32, i32);4] = [(-1,0), (0, 1), (1, 0), (0, -1)];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut work: Vec<((i32, i32), i8)> = vec![loc_val];
    while let Some((loc, v)) = work.pop() {
        for d in dirs.iter() {
            let loc2 = (d.0+loc.0, d.1+loc.1);
            if let Some(v2) = map.get(&loc2) {
                if !visited.contains(&loc2) && v2>&v && *v2<9i8 {
                    work.push((loc2, *v2))
                }
            }
        }
        visited.insert(loc);
    }
    visited.len()
}

fn solution(input_s: &str) -> Result<()> {
    let map: HashMap<(i32, i32), i8> = input_s
    .trim()
    .split("\n").enumerate()
    .flat_map(|(i, row)| row.as_bytes().iter().enumerate().map(move |(j, c)| ((i as i32, j as i32),  (*c-b'0') as i8)))
    .collect();

    let dirs: [(i32, i32);4] = [(-1,0), (0, 1), (1, 0), (0, -1)];

    let lows: Vec<((i32, i32), i8)>= map.iter()
    .filter(|(loc, val)| {
        dirs
        .iter()
        .filter_map(|d| map.get(&(d.0+loc.0, d.1+loc.1)))
        .all(|val_n| val_n>val)
    })
    .map(|(loc, val)| (*loc, *val)).collect();
    
    let p1: usize = lows.iter().map(|(_loc, val)| 1+(*val as usize)).sum();
    println!("Part 1: {}", p1);

    let bsize: Vec<usize> = lows.iter().map(|lv| fill_size(&map, *lv)).sorted().collect();
    let p2: usize = bsize[(bsize.len()-3)..].iter().product();
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
