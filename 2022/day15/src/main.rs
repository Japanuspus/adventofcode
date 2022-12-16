#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use std::{fs, time::Instant, collections::{BTreeMap, BTreeSet}};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug)]
#[display("Sensor at {sensor}: closest beacon is at {beacon}")]
struct Row {
    sensor: Position,
    beacon: Position,
}

#[derive(Debug, Display, FromStr)]
#[display("x={x}, y={y}")]
struct Position {
    x: i32,
    y: i32,
}

fn make_edges(sensors: &Vec<([i32;2], u32)>, y0: i32, m: i32) -> BTreeMap::<i32, i32> {
    let mut es = BTreeMap::new();
    for (&x, r) in sensors.iter().filter_map(
        |([x,y], range)| range.checked_sub(y.abs_diff(y0)).and_then(|r| Some((x, r as i32)))
    ) {
        *(es.entry(x-r).or_default())+=1;
        *(es.entry(x+r+1).or_default())-=1;
    };
    es.entry(0).or_default();
    es.entry(m).or_default();
    es
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum AR {
    Add,
    Remove,
}

fn solution(input_s: &str, y0: i32) -> Result<[String; 2]> {
    let input: Vec<Row> = input_s.trim_end()
        .split("\n")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_, _>>()?;

    let sensors: Vec<([i32;2], u32)> = input.iter()
        .map(|r| (
            [r.sensor.x, r.sensor.y],
            r.beacon.x.abs_diff(r.sensor.x) + r.beacon.y.abs_diff(r.sensor.y)
        )).collect();

    let edges: BTreeSet::<[i32;2]> = sensors.iter().filter_map(
        |([x,y], range)| range.checked_sub(y.abs_diff(y0)).and_then(|r| Some((x, r as i32)))
    ).flat_map(|(&x, r)| [[x-r,-1],[x+r,1]].into_iter()).collect();

    let mut active = 0;
    let mut x0 = 0;
    let mut cover = 0;
    for [x,v] in edges.iter() {
        if active>0 {cover+=x-x0;}
        active-=v;
        x0=*x;
    }
    let part1 = cover;


    let m = 2*y0;
    let mut part2 = -1;
    'outer: for y in 0..(m+1) {
        let mut active = 0;
        let es = make_edges(&sensors, y, m); // always has entries at 0 and m
        //println!("{}: {:?}", y, &es);
        for (x,v) in es.iter() {
            active+=v;
            if active==0 && *x>=0 && *x<=m {
                let pp2 = *x as isize * 4000000isize+y as isize;
                println!("\n xy {},{} -> {}", x, y, pp2);
                let mut ok=true;
                for s in sensors.iter() {
                    let sd = s.0[0].abs_diff(*x) + s.0[1].abs_diff(y);
                    if sd<=s.1 {
                        ok=false;
                    }
                    println!("dist-range {:06} ok: {} > range: {}, dist: {} @ {:?}", 
                        sd-s.1, !(sd<=s.1), s.1, sd, s.0);
                }
                if ok {
                    part2 = pp2;
                }
                // println!("{}: {:?}", y, &es);
                // break 'outer;
            }
        }    
    } 

    // // Part 2: use u,v -coordinates
    // let m = 2*y0;
    // // u = x+y
    // // v = x-y
    // // store regions as [[v1, v2],[u1, u2]]
    // // outer bound is abs(v) < m - abs(m-u)
    // let regions: Vec::<[[i32;2];2]> = sensors.iter().map(|([x,y],r)| {
    //     let u=x+y;
    //     let v=x-y;
    //     [[v-*r as i32, v+*r as i32], [u-*r as i32, u+*r as i32]]
    // }).collect();
    // let mut u_edges: BTreeSet::<(i32, AR, &[[i32;2];2])> = regions.iter().flat_map(|r| [(r[1][0], AR::Add, r), (r[1][1], AR::Remove, r)].into_iter()).collect();
    // let mut active: BTreeSet::<[i32;2]> = BTreeSet::new();
    // let u = w.iter().next().unwrap().0;
    // loop {
    //     let mut active_modified = false;
    //     //let mut next_u = 2*m;
    //     while let Some(w1) = w.iter().next() {
    //         if w1.0 == u {
    //             let r = w.pop_first().unwrap();
    //             match r.1 {
    //                 AR::Add =>{active.remove(r.2.0.clone())},
    //                 AR::Remove => {active.insert(r.2.0.clone())},
    //             }
    //         } else {
    //             //next_u = w1.0;
    //             break;
    //         }
    //     };
    //     u +=1;
    // }
    // if active_modified {
    //     // update observation bounds to be the region that spans 0;
    // }
    // let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}



#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input, 10)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert!(res[0] == "26");
    assert!(res[1] == "56000011");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    for _ in 0..20 {solution(&input, 2000000)?;} //warmup
    let start = Instant::now();
    let res = solution(&input, 2000000)?;
    println!(
        "({} us)\nPart 1: {}\nPart 2: {}",
        start.elapsed().as_micros(), res[0], res[1],
    );
    Ok(())
}


// // Make it simple to compare timing for multiple solutions
// type Solution = dyn Fn(&str) -> Result<[String; 2]>;
// const SOLUTIONS: [(&str, &Solution); 1] = [("Original", &solution)];

// #[test]
// fn test_solution() -> Result<()> {
//     let input = &fs::read_to_string("test00.txt")?;
//     for (name, solution) in SOLUTIONS {
//         let res = solution(&input).with_context(|| format!("Running solution {}", name))?;
//         println!("---\n{}\nPart 1: {}\nPart 2: {}", name, res[0], res[1]);
//         assert!(res[0] == "0");
//         assert!(res[1] == "0");
//     }
//     Ok(())
// }

// fn main() -> Result<()> {
//     let input = &fs::read_to_string("input.txt")?;
//     for (_, solution) in SOLUTIONS.iter().cycle().take(10) {
//         solution(&input)?;
//     } //warmup
//     for (name, solution) in SOLUTIONS {
//         let start = Instant::now();
//         let res = solution(&input)?;
//         println!(
//             "---\n{} ({} us)\nPart 1: {}\nPart 2: {}",
//             name, start.elapsed().as_micros(), res[0], res[1],
//         );
//     }
//     Ok(())
// }
