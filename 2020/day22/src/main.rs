use anyhow::Result;
use std::{
    collections::{HashSet, VecDeque},
    fs,
};

type Deck = VecDeque<u8>;

fn play_normal(mut p1: Deck, mut p2: Deck) -> (bool, Deck, Deck) {
    while (p1.len() > 0) & (p2.len() > 0) {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
    (p1.len() > 0, p1, p2)
}

fn play_recursive(mut p1: Deck, mut p2: Deck) -> (bool, Deck, Deck) {
    let mut old_games: HashSet<[Deck; 2]> = HashSet::new();
    while (p1.len() > 0) & (p2.len() > 0) {
        if !old_games.insert([p1.clone(), p2.clone()]) {
            return (true, p1, p2);
        }

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        if if ((c1 as usize) <= p1.len()) & ((c2 as usize) <= p2.len()) {
            // recursive game
            play_recursive(
                p1.iter().take(c1 as usize).cloned().collect(),
                p2.iter().take(c2 as usize).cloned().collect(),
            )
            .0
        } else {
            c1 > c2
        } {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
    (p1.len() > 0, p1, p2)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let decks: Vec<VecDeque<u8>> = input
        .split("\n\n")
        .map(|pt| {
            pt.lines()
                .skip(1)
                .filter_map(|ln| ln.parse::<u8>().ok())
                .collect()
        })
        .collect();
    for (i, g) in [play_normal, play_recursive].iter().enumerate() {
        let (iwin, p1, p2) = g(decks[0].clone(), decks[1].clone());
        println!(
            "Part {}: {}",
            i + 1,
            (if iwin { &p1 } else { &p2 })
                .iter()
                .rev()
                .enumerate()
                .map(|(i, c)| (i + 1) * (*c as usize))
                .sum::<usize>()
        )
    }
    Ok(())
}
