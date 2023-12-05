#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use intervaltree::IntervalTree;
use parse_display::{Display, FromStr};
use std::{collections::BTreeSet, fs, time::Instant};

#[derive(Debug, Clone, Display, FromStr, PartialEq, Eq, PartialOrd, Ord)]
#[display("{dst} {src} {length}")]
struct MapRange {
    src: isize,
    dst: isize,
    length: isize,
}

#[derive(Debug, Display, FromStr)]
#[display("{from}-to-{to} map:")]
struct MapHead {
    from: String,
    to: String,
}

// intervals input as (start, end), end not included
fn map_intervals(
    maps: BTreeSet<MapRange>,
    intervals: BTreeSet<(isize, isize)>,
) -> BTreeSet<(isize, isize)> {
    let mut intervals = intervals.iter().cloned().peekable();
    // pad ranges for safety
    let mut map_ranges = maps
        .iter()
        .map(|m| (m.src, m.src + m.length, m.dst - m.src))
        .chain(std::iter::once((isize::MAX, isize::MAX, 0)));
    let mut res = BTreeSet::new();
    let mut r = intervals.next().unwrap();
    let mut m = map_ranges.next().unwrap();
    loop {
        if m.1 <= r.0 {
            m = map_ranges.next().unwrap();
        } else if m.0 < r.0 {
            m.0 = r.0
        } else {
            let dd = if m.0 == r.0 { m.2 } else { 0 };
            let nr0 = if m.0 == r.0 {
                // region and map starts agree
                // map overlap
                if m.1 >= r.1 {
                    // all of region is mapped -> new region
                    r.1
                } else {
                    // m.1 < r.1 // only part of region is covered by map
                    m.1
                }
            } else {
                // m.0>r.0 -- map starts after region
                if m.0 >= r.1 {
                    // all of region not mapped -> new region
                    r.1
                } else {
                    // m.0 < r.1 not full region mapped
                    m.0
                }
            };
            res.insert((r.0 + dd, nr0 + dd));
            if nr0 == r.1 {
                if let Some(new_r) = intervals.next() {
                    r = new_r;
                } else {
                    break;
                }
            } else {
                r.0 = nr0;
            }
        };
    }
    res
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let mut blocks = input_s.trim_end().split("\n\n");

    let seeds: Vec<isize> = blocks
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_, _>>()?;

    let maps: Vec<(MapHead, Vec<MapRange>)> = blocks
        .map(|bs| {
            let mut lns = bs.split('\n');
            let head = lns.next().unwrap().parse().unwrap();
            let ranges = lns
                .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
                .collect::<Result<_, _>>()
                .unwrap();
            (head, ranges)
        })
        .collect();

    let maptrees: Vec<IntervalTree<isize, isize>> = maps
        .iter()
        .map(|(_, ranges)| {
            IntervalTree::from_iter(ranges.iter().map(|r| intervaltree::Element {
                range: (r.src)..(r.src + r.length + 1),
                value: r.dst - r.src,
            }))
        })
        .collect();

    let part1: isize = seeds
        .iter()
        .map(|seed0| {
            maptrees.iter().fold(*seed0, |seed, map| {
                seed + map
                    .query_point(seed)
                    .next()
                    .and_then(|el| Some(&el.value))
                    .unwrap_or(&0)
            })
        })
        .min()
        .unwrap();

    let mapped_ranges = maps.iter().fold(
        seeds
            .chunks_exact(2)
            .map(|ab| (ab[0], (ab[0] + ab[1])))
            .collect::<BTreeSet<_>>(),
        |ranges, map| map_intervals(map.1.iter().cloned().collect(), ranges),
    );
    let (part2, _) = mapped_ranges.iter().next().unwrap();

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "35");
    assert_eq!(res[1], "46");
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
