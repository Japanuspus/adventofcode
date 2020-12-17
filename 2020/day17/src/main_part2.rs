use anyhow::Result;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
};
use vecmath;

// use anyhow::Context;
// use itertools::Itertools;
// use parse_display::{FromStr};
// use regex::Regex;
// use apply::Also;
// use num::{BigInt, Integer};

//#[derive(Debug, FromStr)]
//#[display("{key}:{value}")]
//struct InputItem {key: String, value: String}

type Coord = i16;
type Vec3 = vecmath::Vector4<Coord>;

fn directions() -> impl Iterator<Item=Vec3> {
    (0..4).map(|_| (-1..2)).multi_cartesian_product().filter(|v| v != &[0,0,0,0]).map(|v| [v[0], v[1], v[2], v[3]])
}

fn neighbors(v: &Vec3) -> impl Iterator<Item=Vec3> {
    let vc = v.clone();
    directions().map(move |d| vecmath::vec4_add(vc, d))
}

#[test]
fn test_dirs() {
    assert_eq!(directions().count(), 3*3*3-1);
}

type Grid = HashSet<Vec3>;

fn step1(t0: &Grid) -> Grid {
    let field: HashSet<Vec3> = t0.iter().flat_map(|v| neighbors(v)).collect();
    println!("Field: {} -> {}", t0.len(), field.len());

    field.into_iter().filter_map(|cell| {
        let nb_count = neighbors(&cell).filter(|v| t0.contains(v)).count();
        if t0.contains(&cell) {
            if (nb_count>=2) & (nb_count<=3) {Some(cell)} else {None}
        } else {
            if nb_count==3 {Some(cell)} else {None}
        }
    }).collect::<HashSet<Vec3>>()
}


fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let t0: HashSet<Vec3> = input
        .lines()
        .enumerate()
        .flat_map(|(y, ln)| {
            ln.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| [x as Coord, y as Coord, 0, 0])
        })
        .collect();

    let mut g = t0;
    println!("After {}: {}", 0, g.len());
    for i in 1..7 {
        g = step1(&g);
        println!("After {}: {}", i, g.len());
    }


    Ok(())
}
