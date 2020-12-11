use anyhow::{Error, Result};
use apply::Apply;
use std::{
    collections::{HashMap, HashSet},
    fs,
};
// use itertools::Itertools;

type Seat = [i16; 2];

#[derive(Debug)]
struct Lobby {
    seats: HashSet<Seat>,
    size: Seat,
}

type Neighbors = HashMap<Seat, Vec<Seat>>;

fn parse_lobby(input: impl AsRef<str>) -> Result<Lobby> {
    let seats = input
        .as_ref()
        .lines()
        .enumerate()
        .flat_map(|(y, ln)| {
            ln.chars().enumerate().filter_map(move |(x, c)| match c {
                '.' => None,
                'L' => Some(Ok([x as i16, y as i16])),
                _ => Some(Err(Error::msg("Bad char in input"))),
            })
        })
        .collect::<Result<_, _>>()?;
    Ok(Lobby {
        seats,
        size: [
            input
                .as_ref()
                .lines()
                .next()
                .map(|ln| ln.chars().count())
                .ok_or(Error::msg("Empty input"))? as i16,
            input.as_ref().lines().count() as i16,
        ],
    })
}

const DIRECTIONS: [Seat; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
    [1, 0],
    [1, -1],
    [0, -1],
];

fn seat_neighbors(lobby: &Lobby, s: &Seat) -> Vec<Seat> {
    DIRECTIONS
        .iter()
        .map(|&dxy| [s[0] + dxy[0], s[1] + dxy[1]])
        .filter(|p| lobby.seats.contains(p))
        .collect()
}

fn seat_neighbors_los(lobby: &Lobby, s: &Seat) -> Vec<Seat> {
    DIRECTIONS
        .iter()
        .filter_map(|dxy| {
            (0..(lobby.size.iter().max().unwrap_or(&0).clone() as usize))
                .map(|dd| [s[0] + (1+dd as i16)*dxy[0], s[1] + (1+dd as i16)*dxy[1]])
                .filter(|p| lobby.seats.contains(p))
                .next()
        })
        .collect()
}

fn occ_step(nb: &Neighbors, occ: &HashSet<Seat>, leave_at: usize) -> HashSet<Seat> {
    nb.iter()
        .filter(|(&xy, nbs)| {
            let nb_count = nbs.iter().filter(|&p| occ.contains(p)).count();
            if occ.contains(&xy) {
                !(nb_count >= leave_at)
            } else {
                nb_count == 0
            }
        })
        .map(|(xy, _)| xy)
        .cloned()
        .collect()
}

fn find_stable(nb: &Neighbors, leave_at: usize) -> HashSet<Seat> {
    let mut occ = HashSet::new();
    loop {
        let next_occ = occ_step(nb, &occ, leave_at);
        if next_occ == occ {
            break occ;
        }
        occ = next_occ;
    }
}

fn main() -> Result<()> {
    let lobby = fs::read_to_string("input.txt")?.apply(parse_lobby)?;

    // part1
    let nb1: Neighbors = lobby
        .seats
        .iter()
        .cloned()
        .map(|s| (s, seat_neighbors(&lobby, &s)))
        .collect();
    println!("Part 1: {}", find_stable(&nb1, 4).len());

    // part 2
    let nb2: Neighbors = lobby
        .seats
        .iter()
        .cloned()
        .map(|s| (s, seat_neighbors_los(&lobby, &s)))
        .collect();
    println!("Part 2: {}", find_stable(&nb2, 5).len());

    Ok(())
}
