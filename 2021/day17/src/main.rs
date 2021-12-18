use anyhow::{Result, Context};
use std::fs;

use parse_display::{Display, FromStr};
#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{a}..{b}")]
struct Range {
    a: isize,
    b: isize,
}
#[derive(Debug, Display, FromStr)]
#[display("target area: x={x}, y={y}")]
struct Target {
    x: Range,
    y: Range,
}

// y speeds after starting from 0
//  n = 0   1   2   3
// vy = 0   1   2   3
//  y = 0   0   1   3   6  = m*(m+1)/2  m = n-1     (at end of step)
//  n = sqrt(2y+.25)-.5
fn y_step(dist: isize) -> f32 {
    let y: f32 = dist as f32;
    (2.0*y+0.25).sqrt()-0.5
}

/// Possible step counts after starting with vy
fn y_range(vy: isize, r: &Range) -> Option<(usize, usize)> {
    // negative speed, fictive start at y_top
    // -1: we started here one step ago (no motion first step)
    // -2: we started one above two steps ago
    let y_top = vy*(vy+1)/2;
    let n1 = ((vy as f32) + 1.0 + y_step(y_top - r.b)).ceil() as usize;
    let n2 = ((vy as f32) + 1.0 + y_step(y_top - r.a)).floor() as usize;
    if n2<n1 {None} else {Some((n1, n2))}
}

fn solution(input_s: &str) -> Result<()> {
    let input: Target = input_s
        .trim()
        .parse().with_context(|| format!("Parsing {}", input_s))?;
    let t = input;
    println!("Input: {}", t);
    let vy_max = 1 + t.y.a.abs() as isize; 

    let vy_1 = (0..=vy_max).filter(|&vy| y_range(vy, &t.y).is_some()).max().unwrap();
    println!("Part 1: {}", vy_1*(vy_1+1)/2);


    let vx_max = t.x.b+1; 
    let vx_min = y_step(t.x.a).floor() as isize - 1;
    let vy_min = -vy_max;
    let p2:usize = (vy_min..=vy_max)
    .filter_map(|vy| y_range(vy, &t.y))
    .map(|(n1, n2)| 
        (vx_min..vx_max).filter(|vx0| 
            (1..=n2)
            .scan((*vx0, 0), |(vx, x), i| {if *vx>0 {*x+=*vx; *vx-=1;}; Some((i, *x))})
            .any(|(i, x)| i>=n1 && x>=t.x.a && x<=t.x.b)
        ).count()
    )
    .sum();
    println!("Part 2: {}", p2);
    Ok(())
}

fn main() -> Result<()> {
    println!("** TEST **");
    solution(&fs::read_to_string("test00.txt")?)?;
    println!("\n** INPUT **");
    // 1142 low
    solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}

//      1    2     3      4    5    6
//      1    0    -1     -2   -3   -4
//  0   1    1     0     -2   -5   -9
#[test]
fn test_y_range() {
    assert_eq!(y_range(1, &Range{a:-4, b:-3}), None);
    assert_eq!(y_range(1, &Range{a:-4, b:-2}), Some((4, 4)));
    assert_eq!(y_range(1, &Range{a:-5, b:-2}), Some((4, 5)));
    assert_eq!(y_range(1, &Range{a:-5, b:-3}), Some((5, 5)));
}

//       1      2      3  
//      -1    -2     -3
//  0   -1    -3     -6
#[test]
fn test_y_range_negv() {
    assert_eq!(y_range(-1, &Range{a:-5, b:-4}), None);
    assert_eq!(y_range(-1, &Range{a:-6, b:-4}), Some((3, 3)));
    assert_eq!(y_range(-1, &Range{a:-6, b:-3}), Some((2, 3)));
}


//      1    2     3      4   
//      0    -1     -2   -3   
//  0   0    -1     -3   -6   
#[test]
fn test_y_range_zerov() {
    assert_eq!(y_range(0, &Range{a:-5, b:-4}), None);
    assert_eq!(y_range(0, &Range{a:-6, b:-4}), Some((4, 4)));
    assert_eq!(y_range(0, &Range{a:-5, b:-3}), Some((3, 3)));
    assert_eq!(y_range(0, &Range{a:-6, b:-3}), Some((3, 4)));
}
