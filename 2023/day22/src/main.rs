#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use vecmath::vec3_sub;
use std::{fs, time::Instant, collections::{BTreeMap, HashMap, HashSet, BTreeSet}};
use itertools::Itertools;

mod nm {
    pub use nom::multi::*;
    pub use nom::sequence::*;
    pub use nom::character::complete::*;
    pub use nom::bytes::complete::*;
    pub use nom::error::*;
    pub use nom::combinator::*;
    pub use nom::IResult;
}

type V = [i16;3];

fn parse(s: &str) -> nm::IResult<&str, Vec<(V,V)>> {
    let pt = || nm::separated_list1(nm::char(','), nm::i16);
    let v = || nm::map_res(pt(), |v| v.try_into());
    let block = nm::separated_pair(v(), nm::char('~'), v());
    nm::separated_list1(nm::newline, block)(s)
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
struct Block {
    a: V,
    b: V,
    d: V,
    n: i16,
}
type XY = [i16;2];

fn solution(input_s: &str) -> Result<[String; 2]> {
    let (_, bps) = parse(input_s).map_err(|e| e.to_owned())?;

    let blocks: Vec<Block> = bps.iter().map(|(p1,p2)| {
        let (a,b) = if p1[2]<p2[2] {(*p1,*p2)} else {(*p2,*p1)};
        let ba = vec3_sub(b, a);
        let n = ba.iter().map(|v| v.abs()).max().unwrap();
        let d = if n==0 {[0,0,0]} else {ba.map(|v| v/n)};
        Block{n, d, a, b}
    }).collect();

    // populate each x,y-stack to determine dependency graph
    let mut columns: BTreeMap<XY, BTreeMap<(i16,i16), usize>> = BTreeMap::new();
    for (i,b) in blocks.iter().enumerate() {
        let zrange = (b.a[2], b.b[2]);
        for ii in 0..=b.n{
            let xy=[b.a[0]+b.d[0]*ii, b.a[1]+b.d[1]*ii];
            columns.entry(xy).or_default().insert(zrange, i);
        } 
    }

    let mut nb_below: Vec<Vec<(usize, i16)>> = (0..blocks.len()).map(|_| Vec::new()).collect(); //(identifier, distance plus 1)
    for c in columns.values() {
        for (((_,zb),idb), ((zt,_), idt)) in c.iter().tuple_windows() {
            assert!(zt>zb);
            nb_below[*idt].push((*idb, zt-zb));
        }
    }

    let n = blocks.len();
    // process nb_below graph depth first to find downshift
    let mut downshift: Vec<i16> = vec![-1;n]; //-1: not processed
    let mut is_only_support: HashSet<usize> = HashSet::new();
    let mut work: Vec<usize> = Vec::new();
    let mut rests_on: Vec<Vec<usize>> = (0..n).map(|_| Vec::new()).collect();
    let mut supports: Vec<Vec<usize>> = (0..n).map(|_| Vec::new()).collect();
    while let Some(id) = work.pop().or((0..n).filter(|id| downshift[*id]<0).next()) {
        if downshift[id]>=0 {continue}

        // check if all nbs below have settled
        let nbs = &nb_below[id];
        let nb_downshifts: Vec<i16> = nbs.iter().map(|(nb_id,_)| downshift[*nb_id]).collect();
        if nb_downshifts.iter().min().unwrap_or(&0) >=&0 {
            // all below are settled, find how far to shift down and what limits us
            let avail_downshifts: BTreeSet<(i16, usize)> = nbs.iter()
            .zip(nb_downshifts.iter())
            .map(|((nb_id, nb_dist_p1), nb_downshift)| (nb_dist_p1-1+nb_downshift, *nb_id))
            .collect();
            downshift[id] = if let Some((min_downshift, nb_id)) = avail_downshifts.iter().next() {
                rests_on[id].extend(
                    avail_downshifts.iter()
                    .take_while(|(ds, _)| ds==min_downshift)
                    .map(|(_, nb_id)| *nb_id)
                );
                for nb_id in &rests_on[id] {supports[*nb_id].push(id);}
                if rests_on[id].len()==1 {
                    is_only_support.insert(*nb_id);
                }
                *min_downshift
            } else {
                // on floor
                blocks[id].a[2]-1
            };
            // println!("Downshifted {} by {} ({:?})", id, downshift[id], blocks[id]);
        } else {
            // handle this item when all below have been handled:
            work.push(id);
            work.extend(
                nbs.iter().zip(nb_downshifts.iter()).filter_map(|((nb_id, _), od)| {
                    if *od<0 {Some(nb_id)} else {None}
                })
            );
        }
    }

    let part1 = n-is_only_support.len();

    let mut part2: usize = 0;
    for id in 0..n {
        let mut work: BTreeSet<(i16, usize)> = supports[id].iter().map(|ro| (blocks[*ro].a[2], *ro)).collect();
        let mut affected: HashSet<usize> = HashSet::new();
        affected.insert(id);
        while let Some((_, lowest)) = work.pop_first() {
            if affected.contains(&lowest) {continue};
            if !rests_on[lowest].iter().all(|sup| affected.contains(sup)) {continue};
            affected.insert(lowest);
            work.extend(supports[lowest].iter().map(|ro| (blocks[*ro].a[2], *ro)));
        }
        part2+=affected.len()-1;
    }

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "5");
    assert_eq!(res[1], "7");
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
