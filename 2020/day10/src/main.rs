use anyhow::Result;
use apply::Apply;
use itertools::Itertools;
use std::{collections::HashMap, fs, iter::once};

fn main() -> Result<()> {
    let input: Vec<_> = fs::read_to_string("input.txt")?
        .lines()
        .filter_map(|s| s.parse::<isize>().ok())
        .sorted()
        .collect();

    println!(
        "Part 1: {}",
        std::iter::once(input[0]) // diff from input to first adapter
            .chain(input.windows(2).map(|v| v[1] - v[0]))
            .fold((0, 0), |(a1, a3), d| match d {
                1 => (a1 + 1, a3),
                3 => (a1, a3 + 1),
                _ => (a1, a3),
            })
            .apply(|(n1, n3)| n1 * (n3 + 1)) // include diff to device
    );

    // fill possibility count starting from rear=1
    let mut n_map: HashMap<isize, usize> = input.last().iter().map(|&v| (v + 3, 1)).collect();
    for v in input.iter().rev().cloned().chain(once(0)) {
        n_map.insert(v, (1..4).filter_map(|dv| n_map.get(&(v + dv))).sum());
    }
    println!("Part 2: {}", n_map[&0]);

    // Sometimes oldskool mutable code is just easier to read :)
    println!(
        "Part 2: {} as expression",
        input
            .iter()
            .rev()
            .cloned()
            .chain(once(0))
            .scan(
                input
                    .last()
                    .iter()
                    .map(|&v| (v + 3, 1))
                    .collect::<HashMap<isize, usize>>(),
                |m, v| {
                    ((1..4).filter_map(|dv| m.get(&(v + dv))).sum::<usize>())
                        .apply(|count| m.entry(v).or_insert(count).apply(|v| Some(*v)))
                }
            )
            .last()
            .unwrap_or(0)
    );
    Ok(())
}
