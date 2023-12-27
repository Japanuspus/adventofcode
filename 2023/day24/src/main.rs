#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};use vecmath::{vec2_sub, vec2_add, vec2_scale};
use std::{fs, time::Instant};
use itertools::Itertools;

type V = [f64;2];

fn det(a: V, b: V) -> f64 {
    a[0]*b[1]-a[1]*b[0]
}

fn check_intersect(a1: V, a2: V, v1: V, v2: V, bounds: V) -> bool {
    let det_v = det(v1, v2);
    if det_v==0.0 {return false};
    let a21 = vec2_sub(a2, a1);
    let t1 = det(a21, v2)/det_v;
    if t1<0.0 {return false;}
    let t2 = det(a21, v1)/det_v;
    if t2<0.0 {return false;}
    
    let x = vec2_add(a1, vec2_scale(v1, t1));
    x.into_iter().all(|xi| bounds[0]<=xi && xi<=bounds[1])
}

fn solution(input_s: &str, bounds: V) -> Result<[String; 2]> {
    let input: Vec<f64> = input_s
        .trim_end()
        .split(['@',',', '\n'])
        .map(|s| s.trim().parse())
        .collect::<Result<_, _>>()?;
    
    
    // let m = Array2::from_shape_vec([input.len()/6, 6], input);
    let part1 = input.chunks_exact(6).tuple_combinations().filter(|(l1, l2)| 
        check_intersect([l1[0], l1[1]], [l2[0], l2[1]], [l1[3], l1[4]], [l2[3], l2[4]], bounds)
    ).count();
    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input, [7., 27.])?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "2");
    assert_eq!(res[1], "0");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    let start = Instant::now();
    let (res, time) = loop { // run warmup for 100ms
        let lap = Instant::now();
        let res = solution(&input, [200000000000000., 400000000000000.])?;
        if start.elapsed().as_millis()>100 {break (res, lap.elapsed())};
    };
    println!( "({} us)\nPart 1: {}\nPart 2: {}", time.as_micros(), res[0], res[1]);
    Ok(())
}
