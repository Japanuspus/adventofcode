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
    let mut y_max = 0;
    for vy in 0..1000 {
        let y_top = vy*(vy+1)/2;
        let n1f = (vy as f32) + 1.0 + ystep(y_top - t.y.b);
        let n2f = (vy as f32) + 1.0 + ystep(y_top - t.y.a);
        let n1 = n1f.ceil();
        let n2 = n2f.floor();
        if n2<n1 {
            continue
        }
        y_max = vy*(vy+1)/2;
        // println!("Vy {} (max: {}): {}..={} ({}--{})", vy, y_max, n1, n2, n1f, n2f);
    }
    println!("Part 1: {}", y_max);


    // limits on vx
    let vxmax = t.x.b+1; 
    let vxmin = ystep(t.x.a).floor() as isize - 1;

    // Include negative initial y...
    let mut p2 = 0;
    for vy in -100..1000 {
        // negative speed, fictive start at y_top
        // -1: we started here one step ago (no motion first step)
        // -2: we started one above two steps ago
        let y_top = vy*(vy+1)/2; // valid for both signs
        let n1f = (vy as f32) + 1.0 + ystep(y_top - t.y.b);
        let n2f = (vy as f32) + 1.0 + ystep(y_top - t.y.a);
        let n1 = n1f.ceil() as isize;
        let n2 = n2f.floor() as isize;
        if n2<n1 {
            continue
        }
        
        for vx0 in vxmin..=vxmax {
            let mut vx = vx0;
            let mut x = 0;
            for i in 1..=n2 {
                if vx>0 {
                    x+=vx;
                    vx-=1;
                };
                if x>t.x.b {break}
                if i<n1 {continue}
                if x>=t.x.a {
                    // solution
                    //println!("Vy {}: {}..={} ({}--{})", vy, n1, n2, n1f, n2f);
                    //println!("Solution: {}, {} at i={}", vx0, vy, i);
                    //print!("({}, {}), ", vx0, vy);
                    p2+=1;
                    break
                }

            }

        }

    }
    println!("\nPart 2: {}", p2);
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
