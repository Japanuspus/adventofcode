use anyhow::{Result, Context};
use std::fs;
use parse_display::{Display, FromStr};
use std::collections::HashMap;


#[derive(Display, Debug, FromStr)]
#[display("{x},{y}")]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Display, FromStr)]
#[display("{a} -> {b}")]
struct Line {
    a: Point,
    b: Point,
}

fn between(a: i32, b: i32) -> std::ops::RangeInclusive<i32> {
    if b>a {a..=b} else {b..=a}
}

fn main() -> Result<()> {
    let input_s = fs::read_to_string("input.txt")?;
    let input: Vec<Line> = input_s
        .trim()
        .split("\n")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_,_>>()?;

    let mut map: HashMap<(i32, i32), usize> = HashMap::new();
    for ln in input.iter() {
        if ln.a.x == ln.b.x {
            for pt in between(ln.a.y, ln.b.y).map(|y| (ln.a.x, y))
            { *map.entry(pt).or_insert(0)+=1; }
        } else if ln.a.y == ln.b.y {
            for pt in between(ln.a.x, ln.b.x).map(|x| (x, ln.a.y))
            { *map.entry(pt).or_insert(0)+=1; }         
        }
    }
    println!("Part 1: {}", map.values().filter(|&v| *v>=2).count());

    // Keep map, add diagonals
    for ln in input.iter() {
        if (ln.b.x-ln.a.x).abs() == (ln.b.y-ln.a.y).abs() {
            let dx=(ln.b.x-ln.a.x).signum();
            let dy=(ln.b.y-ln.a.y).signum();
            let nx=1+(ln.b.x-ln.a.x).abs() as usize;
            let (mut x, mut y) = (ln.a.x, ln.a.y);
            for _ in 0..nx {
                *map.entry((x,y)).or_insert(0)+=1; 
                x+=dx;
                y+=dy;
            }
        } 
    }
    println!("Part 2: {}", map.values().filter(|&v| *v>=2).count());
    Ok(())
}    