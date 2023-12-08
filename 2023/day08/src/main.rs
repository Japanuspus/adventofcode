#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use core::fmt;
use itertools::Itertools;
use num::BigInt;
use num::Integer;
use std::{collections::HashMap, fs, time::Instant};

#[test]
fn test_extended_euclid() {
    let e = isize::extended_gcd(&240, &46);
    assert_eq!(e.x, -9);
    assert_eq!(e.y, 47);
    assert_eq!(e.gcd, 2);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RSpec<T: Clone> {
    n: T,
    a: T,
}

fn chinese_remainder<T: Integer + Clone + fmt::Debug>(n1: RSpec<T>, n2: RSpec<T>) -> RSpec<T> {
    let ee = T::extended_gcd(&n1.n, &n2.n);
    if ee.gcd != T::one() {
        assert_eq!(n1.a.mod_floor(&ee.gcd), n2.a.mod_floor(&ee.gcd));
    }
    let n = n1.n.clone() * n2.n.clone() / ee.gcd.clone();
    let a = ((n1.a * ee.y * n2.n + n2.a * ee.x * n1.n) / ee.gcd.clone()).mod_floor(&n); // rem_euclid(n);
    RSpec { n, a }
}

#[test]
fn test_chinese_remainder() {
    let res = chinese_remainder(RSpec::<isize> { n: 3, a: 2 }, RSpec::<isize> { n: 5, a: 1 });
    assert_eq!(res, RSpec::<isize> { n: 15, a: 11 });
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let mut lines = input_s.trim_end().split("\n");
    let lrs: Vec<u8> = lines.next().unwrap().as_bytes().to_owned();
    lines.next();
    // QKX = (SQD, XTJ)
    let forks: HashMap<String, (String, String)> = lines
        .map(|ln| {
            (
                ln[0..3].to_owned(),
                (ln[7..10].to_owned(), ln[12..15].to_owned()),
            )
        })
        .collect();

    let part1 = lrs
        .iter()
        .cycle()
        .scan("AAA", |state, step| {
            let inst = forks.get(*state).unwrap();
            *state = match step {
                b'L' => &inst.0,
                b'R' => &inst.1,
                _ => panic!("not L or R"),
            };
            Some(*state)
        })
        .take_while_inclusive(|state| *state != "ZZZ")
        .count();

    let periods: Vec<(usize, usize)> = forks
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|start| {
            let zs: Vec<_> = lrs
                .iter()
                .cycle()
                .scan(start, |state, step| {
                    let inst = forks.get(*state).unwrap();
                    *state = match step {
                        b'L' => &inst.0,
                        b'R' => &inst.1,
                        _ => panic!("not L or R"),
                    };
                    Some(*state)
                })
                .enumerate()
                .filter(|(_, state)| state.ends_with('Z'))
                .take(2)
                .collect();
            assert_eq!(zs[0].1, zs[1].1); //Check that this is a cycle
            (zs[0].0 + 1, zs[1].0 + 1)
        })
        .collect();
    // for p in &periods {
    //     println!("Period: {:?} {}", p, p.1 - p.0)
    // }

    let common = periods
        .iter()
        .map(|(a, n2)| RSpec {
            n: BigInt::from(n2 - a),
            a: BigInt::from(*a),
        })
        .reduce(|acc, p2| chinese_remainder(acc, p2))
        .unwrap();

    let part2 = common.a + common.n;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test01.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "6");
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
