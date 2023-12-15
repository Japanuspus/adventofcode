#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use std::{fs, time::Instant, str::FromStr, collections::HashMap, cell::RefCell};
use itertools::Itertools;
use std::sync::atomic::{AtomicU16, Ordering};

struct Game {
    records: Vec<u8>,
    groups: Vec<usize>,
    cache_feasible: RefCell<HashMap<(usize, usize), usize>>,
    cache_feasible_fixed: RefCell<HashMap<(usize, usize), usize>>,
}

impl Game {
    fn new(records: Vec<u8>, groups: Vec<usize>) -> Self {
        let cache_feasible = RefCell::from(HashMap::new());
        let cache_feasible_fixed = RefCell::from(HashMap::new());
        Self {records, groups, cache_feasible, cache_feasible_fixed}
    }

    fn all_feasible(&self) -> usize {
        self.feasible(self.records.len(), self.groups.len())
    }

    fn feasible(&self, n_rec: usize, n_grp: usize) -> usize {
        let key = (n_rec, n_grp);
        if let Some(v) = self.cache_feasible.borrow().get(&key) {return *v}
        let count = self. feasible_inner(n_rec, n_grp);
        self.cache_feasible.borrow_mut().insert(key, count);
        count
    }

    fn feasible_inner(&self, n_rec: usize, n_grp: usize) -> usize {
        let rec = &self.records[self.records.len()-n_rec..];
        let grp = &self.groups[self.groups.len()-n_grp..];
        let max_shift = {
            let min_pattern_length = if n_grp==0 {0} else {grp.iter().sum::<usize>()+n_grp-1};
            let first_hash = rec.iter().enumerate().filter_map(|(i, v)| if *v==b'#' {Some(i)} else {None}).next();
            if n_grp==0 {
                return if first_hash.is_some() {0} else {1}
            }
            let remaining_space = rec.len().checked_sub(min_pattern_length).unwrap_or_else(|| return 0);
            match first_hash {
                Some(v) => v.min(remaining_space),
                None => remaining_space,
        }};
        let mut count: usize = 0;
        
        for (shift, c) in rec.iter().take(max_shift+1).enumerate() {
            count += self.feasible_fixed(n_rec-shift, n_grp);
            if *c==b'#' {break};
        }
        count
    }

    fn feasible_fixed(&self, n_rec: usize, n_grp: usize) -> usize {
        let key = (n_rec, n_grp);
        if let Some(v) = self.cache_feasible_fixed.borrow().get(&key) {return *v}
        let count = self. feasible_fixed_inner(n_rec, n_grp);
        self.cache_feasible_fixed.borrow_mut().insert(key, count);
        count
    }

    fn feasible_fixed_inner(&self, n_rec: usize, n_grp: usize) -> usize {
        let rec = &self.records[self.records.len()-n_rec..];
        let grp = &self.groups[self.groups.len()-n_grp..];
        let g = grp[0];
        let mut symbols = rec.iter();
        if !(
            symbols.by_ref().take(g).all(|&v| v!=b'.') 
            && symbols.next().and_then(|&v| Some(v!=b'#')).unwrap_or(true)
        ) {
            return 0
        }
        self.feasible((n_rec-g).checked_sub(1).unwrap_or(0), n_grp-1)
    }
}

fn recursive_solution(input_s: &str, n_rep: usize) -> usize {
    let input: Vec<(&[u8], Vec<usize>)> = input_s
        .trim_end()
        .split("\n")
        .filter_map(|ln| {
            ln.split_once(' ')
            .and_then(|(b,r)| Some((
                b.as_bytes(),
                r.split(',').map(|s| s.parse().unwrap()).collect_vec()
            )))
        })
        .collect();

    let input = input.into_iter()
    .map(|(rec, grp)| Game::new(
        vec![rec;n_rep][..].join(&b'?'),
        vec![grp;n_rep][..].concat(),
    ))
    .collect_vec();

    //let foo = AtomicU16::new(0);
    input.iter()
    .map(|game| {
        let res = game.all_feasible();
        //let prog = foo.fetch_add(1, Ordering::SeqCst);
        //println!("{:3}> {:10}: {:?} {:?}", prog, res, std::str::from_utf8(&game.records).unwrap(), game.groups);
        res
    }).sum()
}


fn solution(input_s: &str) -> Result<[String; 2]> {
    let part1 = recursive_solution(input_s, 1);
    let part2 = recursive_solution(input_s, 5);

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test01.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "21");
    assert_eq!(res[1], "525152");
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
