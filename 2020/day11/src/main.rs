use std::{collections::HashSet, fs};
use anyhow::{Result, Error};
// use itertools::Itertools;

fn nb_count(seat: &(i16, i16), occ: &HashSet<(i16, i16)>) -> usize {
    [(-1,-1), (-1,0), (-1, 1), (0,1), (1,1), (1,0), (1, -1), (0,-1)].iter()
    .filter_map(|(dx, dy)| occ.get(&(seat.0+dx, seat.1+dy))).count()
}


fn occ_step(seats: &Vec<(i16, i16)>, occ: &HashSet<(i16, i16)>) -> HashSet<(i16,i16)> {
    seats.iter().filter(|xy| {
        if occ.contains(xy) {
            !(nb_count(xy, occ)>=4)
        } else {
            nb_count(xy, occ)==0
        }
    }).cloned().collect()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let seat_coords: Vec<(i16, i16)> = input.lines().enumerate()
    .flat_map(|(y, ln)| ln.chars().enumerate().filter_map(move |(x, c)| match c {
        '.' => None,
        'L' => Some(Ok((x as i16, y as i16))),
        _ => Some(Err(Error::msg("Bad char in input"))),
    })).collect::<Result<_,_>>()?;

    // part1
    let mut occ = HashSet::new();
    loop {
        let next_occ = occ_step(&seat_coords, &occ);
        if next_occ==occ { break; }
        occ = next_occ;
    }
    
    println!("Part 1: {}", occ.len());

    Ok(())
}
