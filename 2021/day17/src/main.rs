#![allow(unused_imports, dead_code)]
#![allow(unused_imports, dead_code)]

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

fn ystep(dist: isize) -> f32 {
    let y: f32 = dist as f32;
    (2.0*y+0.25).sqrt()-0.5
}

fn solution(input_s: &str) -> Result<()> {
    let input: Target = input_s
        .trim()
        .parse().with_context(|| format!("Parsing {}", input_s))?;
    let t = input;
    println!("Input: {}", t);
    // y speeds after starting from 0
    //  n = 0   1   2   3
    // vy = 0   1   2   3
    //  y = 0   0   1   3   6  = m*(m+1)/2  m = n-1     (at end of step)
    //  n = sqrt(2y+.25)-.5

    // x speeds
    // n     =  0    1
    // vx    =  3    2    1    0     0
    // xend  =  3    5    6    0     0     
    // x1  =  6 
    for vy in 0..1000 {
        let y_top = vy*(vy+1)/2;
        let n1f = (vy as f32) + 1.0 + ystep(y_top - t.y.b);
        let n2f = (vy as f32) + 1.0 + ystep(y_top - t.y.a);
        let n1 = n1f.ceil();
        let n2 = n2f.floor();
        if n2<n1 {
            //println!("No solution for {}: {}--{}", vy, n1f, n2f);
            continue
        }
        println!("Vy {} (max: {}): {}..={} ({}--{})", vy, vy*(vy+1)/2, n1, n2, n1f, n2f);

    }
    // // assume x stable at end
    // let n1xf = ystep(t.x.a);
    // let n2xf = ystep(t.x.b);
    // let n1x = n1xf.ceil();
    // let n2x = n2xf.floor();
    // println!("Vx {}..={}", n1x, n2x);


    println!("Part 2: {}", 0);
    Ok(())
}

fn main() -> Result<()> {
    println!("** TEST **");
    solution(&fs::read_to_string("test00.txt")?)?;
    println!("\n** INPUT **");
    solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
