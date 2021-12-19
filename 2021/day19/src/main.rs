#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use itertools::Itertools;
use ndarray::{s, ArrayView2, Array2, Array, ArrayView, ArrayView1, ArrayBase, Data, Ix1};
use std::{fs, collections::{HashMap, HashSet}, ops::Sub};
// use ndarray_linalg::solve::Determinant; // compile errors galore 

fn compute_det2(m: &ArrayView2<i32>) -> i32 {
    m[[0,0]]*m[[1,1]] - m[[0,1]]*m[[1,0]]
}

fn make_rot(a: Vec<&[i32;3]>) -> Option<Array2<i32>> {
    let m = Array::from_iter(a.iter().flat_map(|r| r.iter()).cloned());
    // alternative: from_shape_vec
    let m = m.into_shape((3,3)).unwrap();
    // massive compile issues for ndarray-linalg
    // also ndarray-linalg only supports floats...
    let det = 
    m[[0,0]]*compute_det2(&m.slice(s![1.., 1..]))
    -m[[0,1]]*compute_det2(&m.slice(s![1.., 0..;2]))
    +m[[0,2]]*compute_det2(&m.slice(s![1.., ..2]));
    if det==1 {Some(m)} else {None}
}

fn make_array<S>(r: &ArrayBase<S, Ix1>) -> [i32;3] 
where
    S: Data<Elem = i32>
{
    [r[0], r[1], r[2]]
}

fn array_to_point_set(m: &Vec<i32>) -> HashSet<&[i32]> {
    m.chunks(3).collect()
}

fn align(s0: &Vec<i32>, s1: &Vec<i32>, rots: &Vec<Array2<i32>>) -> Result<Option<Vec<i32>>> {
    //Build lookup of relative vectors within s0
    let pts_set = array_to_point_set(s0);
    let pts = ArrayView2::from_shape((s0.len()/3, 3), s0)?.into_owned();

    let mut dp_lookup: HashMap<[i32;3], HashSet<usize>> = HashMap::new();
    for (i, ri) in pts.rows().into_iter().enumerate() {
        for rj in pts.rows().into_iter() {
            // https://docs.rs/ndarray/0.15.4/ndarray/struct.ArrayBase.html#binary-operators-with-two-arrays 
            let dij = &rj-&ri;
            if dij.iter().all(|v| *v==0) { continue }
            dp_lookup.entry(make_array(&dij)).or_default().insert(i);
        }
    }

    //Loop over rotations and pairs in s1. If matching relative vector is found, check if it is a match
    let new_pts = ArrayView2::from_shape((s1.len()/3, 3), s1)?;
    for r in rots {
        let new_pts_rot = new_pts.dot(r);
        for (k, rk) in new_pts_rot.rows().into_iter().enumerate() {
            // possible matches:
            let i_vals: HashSet<usize> = new_pts_rot.rows().into_iter()
            .filter_map(|rj| dp_lookup.get(&make_array(&(&rj-&rk))))
            .flat_map(|rj_is| rj_is.iter())
            .cloned()
            .collect();
            //println!("Found {} possible matches for entry {}: {}", i_vals.len(), k, rk);
            // check each possible match
            for i in i_vals.iter() {
                let new_pts_shift = &new_pts_rot + (&pts.row(*i)-&rk);
                assert_eq!(new_pts_shift.row(k), pts.row(*i));
                let new_pts_shift_vec = new_pts_shift.into_raw_vec();
                let new_pts_shift_set = array_to_point_set(&new_pts_shift_vec);
                let ct = pts_set.intersection(&new_pts_shift_set).count();
                assert!(ct>=2);
                if ct>=12 {
                    return Ok(Some(new_pts_shift_vec))
                }
            }
        }
    }
    Ok(None)
}

fn solution(input_s: &str) -> Result<()> {
    let input: Vec<Vec<i32>> = input_s
        .trim()
        .split("\n\n")
        .map(|s| {
            s.split(&['\n', ','][..])
            .skip(1)
            .map(|v| v.parse())
            .collect::<Result<_,_>>()
            .with_context(|| format!("Parsing {}", s))
        })
        .collect::<Result<_,_>>()?;

    //rotations
    let units = [[0,0,1],[0,1,0],[1,0,0],[0,0,-1],[0,-1,0],[-1,0,0]];
    let rots: Vec<_>  = (0..3).map(|_| units.iter()).multi_cartesian_product().filter_map(make_rot).collect();
    assert_eq!(rots.len(), 24);

    //part 1
    let mut complete: Vec<Vec<i32>> = Vec::new();
    let mut ready: Vec<Vec<i32>> = vec![input[0].clone()];
    let mut remaining: Vec<&Vec<i32>> = input[1..].iter().collect();
    while let Some(s0) = ready.pop() {
        remaining = remaining.into_iter().filter_map(|s1| {
            if let Some(s) = align(&s0, s1, &rots).unwrap() {
                ready.push(s);
                None
            } else {
                Some(s1)
            }
        }).collect();
        complete.push(s0);
    }

    let beacons = complete.iter()
    .map(|s| array_to_point_set(s))
    .reduce(|mut a, b| {a.extend(b.into_iter()); a})
    .unwrap();

    println!("Part 1: {}", beacons.len());
    println!("Part 2: {}", 0);
    Ok(())
}

fn main() -> Result<()> {
    println!("** TEST **");
    solution(&fs::read_to_string("test05.txt")?)?;
    println!("\n** INPUT **");
    solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
