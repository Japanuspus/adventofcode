use std::{collections::HashMap, fs};
use anyhow::Result;
use itertools::iterate;
use std::iter::Iterator;

fn part1(input: &[isize], turns: isize) -> isize {
    let mut n: isize = input.len() as isize;
    let mut round_lookup: HashMap::<isize, isize> = input
    .iter().enumerate().take((n-1) as usize).map(|(i, &n)| (n, (i+1) as isize)).collect();
    let mut spoken = input[(n-1) as usize];
    while n<turns {
        spoken = round_lookup.insert(spoken, n).map(|old_n| n-old_n).unwrap_or(0);
        n+=1;
        //println!("n: {}, spoken: {}, lookup: {:?}", n, spoken, round_lookup);
    };
    spoken
}

fn iter_solution(input: &[isize], turns: usize) -> usize {
    struct State {spoken: usize, lookup: HashMap::<usize, usize>};
    (input.len()..turns).
     fold(
        State{
            spoken: input[input.len()-1] as usize,
            lookup: input.iter().enumerate().take(input.len()-1).map(|(i, &n)| (n as usize, i+1)).collect(),
        },
        |mut s, n| {
            s.spoken = s.lookup.insert(s.spoken, n).map(|old_n| n-old_n).unwrap_or(0);
            s
        },
    ).spoken
}

#[test]
fn test_part1() {
    assert_eq!(part1(&[0,3,6], 9), 4); 
    assert_eq!(iter_solution(&[0,3,6], 9), 4); 
}


fn main() -> Result<()> {
    let input: Vec<isize> = fs::read_to_string("input.txt")?
    .lines()
    .flat_map(|ln| ln.split(',').filter_map(|s| s.parse::<isize>().ok()))
    .collect();

    //part1(&[0,3,6], 9);
    println!("Part 1: {}", part1(&input, 2020));
    println!("Part 2: {}", part1(&input, 30000000));
    Ok(())
}
