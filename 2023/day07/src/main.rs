#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use std::{collections::BTreeSet, fs, time::Instant};

fn card_val(c: char) -> Result<u8> {
    match c {
        'A' => Some(14),
        'K' => Some(13),
        'Q' => Some(12),
        'J' => Some(11),
        'T' => Some(10),
        c => c
            .to_digit(10)
            .and_then(|v| if v > 1 { Some(v as u8) } else { None }),
    }
    .ok_or_else(|| anyhow!("Invalid card symbol: {}", c))
}

fn parse_cards(s: &str) -> Result<[u8; 5]> {
    s.chars()
        .map(|c| card_val(c))
        .collect::<Result<Vec<_>>>()
        .and_then(|v| {
            v.try_into()
                .or_else(|_| Err(anyhow!("Invalid hand string {}", s)))
        })
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    signature: [u8; 5], //multiplicity, largest first, 0-padded. signature.sum==5
    cards: [u8; 5],
}

fn score_hand(cards: [u8; 5]) -> Hand {
    // Use 0 in input to indicate Joker
    let mut card_values: Vec<_> = cards.into_iter().collect();
    card_values.sort();
    let counts: BTreeSet<(u8, u8)> = card_values
        .into_iter()
        .filter(|v| *v > 0) // do not include jokers in grouping here
        .group_by(|v| *v)
        .into_iter()
        .map(|(val, group)| (group.count() as u8, val))
        .collect();

    let mut signature = [0; 5];
    for (i, (count, _val)) in counts.into_iter().rev().enumerate() {
        signature[i] = count;
    }
    //Add jokers to largest group in signature
    signature[0] += cards.iter().filter(|v| **v == 0).count() as u8;
    Hand { signature, cards }
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let input: Vec<([u8; 5], u32)> = input_s
        .trim_end()
        .split("\n")
        .map(|s| {
            parse_cards(&s[..5]).and_then(|cards| {
                s[6..]
                    .parse::<u32>()
                    .context("No bet")
                    .and_then(|bet| Ok((cards, bet)))
            })
        })
        .collect::<Result<Vec<_>>>()?;

    // Map 11 (jacks) to 0 (joker) for part 2:
    Ok([|v| v, |v| if v == 11 { 0 } else { v }].map(|f| {
        let hands: BTreeSet<_> = input
            .iter()
            .map(|(cards, bet)| (score_hand(cards.map(f)), *bet))
            .collect();
        hands
            .iter()
            .enumerate()
            .map(|(i, (_hand, bet))| (i + 1) * (*bet as usize))
            .sum::<usize>()
            .to_string()
    }))
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "6440");
    assert_eq!(res[1], "5905");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    for _ in 0..20 {
        solution(&input)?;
    } //warmup
    let start = Instant::now();
    let res = solution(&input)?;
    println!(
        "({} us)\nPart 1: {}\nPart 2: {}",
        start.elapsed().as_micros(),
        res[0],
        res[1],
    );
    Ok(())
}