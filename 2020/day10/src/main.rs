use anyhow::Result;
use apply::Apply;
use itertools::Itertools;
use std::fs;

fn main() -> Result<()> {
    let input: Vec<_> = fs::read_to_string("input.txt")?
        .lines()
        .filter_map(|s| s.parse::<isize>().ok())
        .sorted()
        .collect();

    println!(
        "Part 1: {}",
        std::iter::once(input[0])  // diff from input to first adapter
            .chain(input.windows(2).map(|v| v[1] - v[0]))
            .fold((0, 0), |(a1, a3), d| match d {
                1 => (a1 + 1, a3),
                3 => (a1, a3 + 1),
                _ => (a1, a3),
            })
            .apply(|(n1, n3)| n1 * (n3 + 1))  // include diff to device
    );

    Ok(())
}
