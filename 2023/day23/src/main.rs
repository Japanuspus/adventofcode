#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use vecmath::{vec2_add, vec2_scale};
use std::{fs, time::Instant, collections::{HashMap, HashSet, BTreeSet}};
use itertools::Itertools;

type V = [i16;2];
struct Map {
    paths: HashMap<V, char>,
    pmax: V,
}
fn parse(s: &str) -> Map  {
    let mut paths = HashMap::new();
    let mut pmax = [0,0];
    for (y, ln) in s.trim_end().split_whitespace().enumerate() {
        for (x, c) in ln.chars().enumerate() {
            pmax = [x as i16, y as i16];
            if c=='#' {continue}
            paths.insert(pmax, c);
        }
    }
    Map{paths, pmax}
}

fn edge_norm(a: V, b: V) -> (V, V) {
    if a<b {(a,b)} else {(b,a)}
}

// Contracted graph
// (length, direction: 1 ->, -1 <-, 0: <>, endpoint)
type Edges = HashMap<V, Vec<(i16, i16, V)>>;
fn make_edges(map: &Map) -> Edges {
    let mut edges: Edges = HashMap::new();
    let mut work: Vec<(V,V)> = vec![([1,0],[0,1])];
    let mut complete: HashSet<(V, V)> = HashSet::new();
    while let Some(w_pd) = work.pop() {
        if !complete.insert(w_pd) {
            continue;
        }
        let (s, d2, pd2) = {
            let mut s = 0;
            let mut pd: (V,V) = (vec2_add(w_pd.0, w_pd.1), w_pd.1);
            let mut c: char = map.paths[&pd.0];

            let mut uphill = false;
            let mut downhill = false;
            loop {
                s+=1;
                if c=='.' {
                    // discover all ways out from point
                    let mut next_pdcs = 
                    [[0,-1], [-1,0], [0,1], [1,0]].into_iter()
                    .filter(|d2| vec2_add(pd.1, *d2)!=[0,0])
                    .map(|d2| (vec2_add(pd.0, d2), d2))
                    .filter_map(|pd2| map.paths.get(&pd2.0).and_then(|c| Some((pd2, *c))))
                    .collect_vec();

                    if next_pdcs.len()!=1 {
                        // edge ends here
                        work.extend(
                                next_pdcs.iter().map(|((_, d2), _c)| (pd.0, *d2))
                        );
                        break
                    } else {
                        (pd, c) = next_pdcs.pop().unwrap();
                    }
                } else {
                    match (c, pd.1) {
                        ('>', [ 1,  0]) | ('<', [-1,  0]) => {downhill=true},
                        ('>', [-1,  0]) | ('<', [ 1,  0]) => {uphill=true},
                        ('v', [ 0,  1]) | ('^', [ 0, -1]) => {downhill=true},
                        ('v', [ 0, -1]) | ('^', [ 0,  1]) => {uphill=true},
                        _ => panic!("skew hill")
                    };
                    pd.0 = vec2_add(pd.0, pd.1);
                    c = *map.paths.get(&pd.0).unwrap(); 
                };
            }
            let o = match (uphill, downhill) {
                (true, false) => -1,
                (false, true) => 1,
                (false, false) => 0,
                _ => panic!("Impassable path")
            };
            (s, o, pd)
        };
        // register both directions for this path
        edges.entry(w_pd.0).or_default().push((s, d2, pd2.0));
        edges.entry(pd2.0).or_default().push((s, -d2, w_pd.0));
        complete.insert((pd2.0, vec2_scale(pd2.1, -1)));
    }
    edges
}

fn max_distance(
    edges: &Edges,
    visited: &mut HashSet<(V, V)>,
    p0: V, p1: V) 
-> Option<usize> {
    if p0==p1 {
        return Some(0)
    }
    let options = edges.get(&p0).iter().flat_map(|v| v.iter())
        .filter(|(_s, o, _nb)| *o>=0)    
        .filter(|(_s, _o, nb)| !visited.contains(&edge_norm(p0, *nb)))
        .cloned().collect_vec();
    let mut res = 0;
    for (s, _, nb) in options {
        let en = edge_norm(p0, nb);
        visited.insert(en);
        if let Some(from_here) = max_distance(edges, visited, nb, p1) {
            res = res.max(s as usize + from_here);
        }
        visited.remove(&en);
    }
    if res==0 {None} else {Some(res)}
}

fn max_dry_distance(
    edges: &Edges,
    visited: &mut HashSet<V>,
    p0: V, p1: V) 
-> Option<usize> {
    if p0==p1 {
        return Some(0)
    }
    let options = edges.get(&p0).iter().flat_map(|v| v.iter())
        .filter(|(_s, _o, nb)| !visited.contains(nb))
        .cloned().collect_vec();
    let mut res = 0;
    visited.insert(p0);
    for (s, _, nb) in options {
        if let Some(from_here) = max_dry_distance(edges, visited, nb, p1) {
            res = res.max(s as usize + from_here);
        }
    }
    visited.remove(&p0);
    if res==0 {None} else {Some(res)}
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let map = parse(input_s);
    let edges = make_edges(&map);
    
    for (v, cs) in &edges {
        println!("{:?}", v);
        for c in cs {
            println!(" -> {:?}", c);
        }
    }

    let p1 = [1,0];
    let p2 = [map.pmax[0]-1, map.pmax[1]];
    let part1 = max_distance(&edges, &mut HashSet::new(),p1, p2).unwrap_or(0);
    let part2 = max_dry_distance(&edges, &mut HashSet::new(), p1, p2).unwrap_or(0);

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "94");
    assert_eq!(res[1], "154");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    let start = Instant::now();
    let (res, time) = loop { // run warmup for 100ms
        let lap = Instant::now();
        let res = solution(&input)?;
        if start.elapsed().as_millis()>100 {break (res, lap.elapsed())};
    };
    println!( "({} us)\nPart 1: {}\nPart 2: {}", time.as_micros(), res[0], res[1]);
    Ok(())
}
