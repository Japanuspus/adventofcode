#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use std::{fs, collections::{HashSet, HashMap}, str, time::Instant};

/// 800us
fn find_marker(s: &str, n: usize) -> Option<usize> {
    s.as_bytes().windows(n).enumerate().find_map(|(i, grp)|{
        if grp.iter().collect::<HashSet<_>>().len()==n {Some(i+n)} else {None}
    })
}

/// 300us
fn find_marker_b(s: &str, n: usize) -> Option<usize> {
    let bs = s.as_bytes();
    let mut m: HashMap<u8, usize> = HashMap::new();
    for b in bs[..n].iter() {
        *(m.entry(*b).or_default())+=1;
    }
    bs.windows(n+1).enumerate().scan(m, |m, (i, w)| {
        *(m.get_mut(&w[0]).unwrap())-=1;
        *(m.entry(w[n]).or_default())+=1;
        Some((i, m.values().filter(|&v| *v==1).count()))
    }).find_map(|(i, n_distinct)| 
        if n_distinct==n {Some(i+n+1)} else {None}
    )
}

/// 250us
fn find_marker_c(s: &str, n: usize) -> Option<usize> {
    let bs = s.as_bytes();
    let mut m = [0u8;u8::MAX as usize];
    for b in bs[..n].iter() { m[*b as usize]+=1; }
    bs.windows(n+1).enumerate().scan(m, |m, (i, w)| {
        m[w[0] as usize]-=1;
        m[w[n] as usize]+=1;
        Some((i, m.iter().filter(|&v| *v==1).count()))
    }).find_map(|(i, n_distinct)| 
        if n_distinct==n {Some(i+n+1)} else {None}
    )
}

fn solution(input_s: &str, f: &dyn Fn(&str, usize) -> Option<usize>) -> Result<(String, String)> {
    Ok((
        f(input_s, 4).unwrap().to_string(),
        f(input_s, 14).unwrap().to_string()
    ))
}

#[test]
fn test_solution() -> Result<()> {
    let input = fs::read_to_string("test00.txt")?;
    for f in [find_marker, find_marker_b, find_marker_c] {
        let res=solution(&input, &f)?;
        println!("Part 1: {}\nPart 2: {}", res.0, res.1);
        assert!(res.0=="7");
        assert!(res.1=="19");
    }
    Ok(())
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    for f in [find_marker, find_marker_b, find_marker_c] {
        let tick = Instant::now();
        let res=solution(&input, &f)?;
        let elapsed = tick.elapsed();
        println!("Part 1: {}\nPart 2: {}\n Duration: {} us", res.0, res.1, elapsed.as_micros());
    }
    Ok(())
}
