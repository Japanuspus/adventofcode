#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use std::{fs, collections::HashMap};

// use parse_display::{Display, FromStr};

// #[derive(Display, FromStr, PartialEq, Debug)]
// enum Direction {
//     #[display("forward")]
//     Forward,
// }

// #[derive(Debug, Display, FromStr)]
// #[display("{direction} {distance}")]
// struct Step {
//     direction: Direction,
//     distance: i32,
// }

// inp w            w = d
// 00   mul x 0     x = 0
// 01   add x z     x = z
// 02   mod x 26    x = z%26
// 03   div z <a1>  z = z/a1
// 04   add x <a2>  x = z%26+a2
// 05   eql x w     x = z%26+a2 == d
// 06   eql x 0     x = !(z%26+a2 == d)
// 07   mul y 0     y = 0
// 08   add y <a3>  y = a3
// 09   mul y x     y = a3*!(z%26+a2 == d)
// 10   add y 1     y = 1+a3*!(z%26+a2 == d)
// 11   mul z y     z = (z/a1)*(1+a3*!(z%26+a2 == d))
// 12   mul y 0     y = 0
// 13   add y w     y = d
// 14   add y <a4>  y = d+a4
// 15   mul y x     y = !(z%26+a2 == d)*(d+a4)
// 16   add z y     z = (z/a1)*(1+a3*!(z%26+a2 == d))+ !(z%26+a2 == d)*(d+a4)
//                    = if (z%26+a2 == d) {z/a1} else {(z/a1)*(1+a3)+(d+a4)}

type Arg = [i8; 4];

fn compute_next_z(z: usize, d: i8, [a1, a2, a3, a4]: Arg) -> usize {
    if (z % 26) as i8 + a2 == d {
        z / a1 as usize
    } else {
        (z / a1 as usize) * (1 + a3 as usize) + (d + a4) as usize
    }
}

fn maximize(z_in: usize, args: &[Arg], cache: &mut HashMap<(usize, usize), Option<Vec<i8>>>) -> Option<Vec<i8>> {
    if args.len() == 0 {
        return if z_in == 0 { Some(Vec::new()) } else { None };
    }
    (1..=9)
        .rev()
        .filter_map(|d| {
            let z = compute_next_z(z_in, d, args[0]);
            let key = (args.len(), z);
            //let v = cache.entry(key).or_insert_with(|| maximize(z, &args[1..], cache)).clone();
            let v;
            if let Some(value) = cache.get(&key) {
                v = value.clone();
            } else {
                let value = maximize(z, &args[1..], cache);
                v = value.clone();
                cache.insert(key, value);
            }
            v.and_then(|mut vv| {
                vv.push(d);
                Some(vv)
            })
        })
        .next()
}

fn minimize(z_in: usize, args: &[Arg], cache: &mut HashMap<(usize, usize), Option<Vec<i8>>>) -> Option<Vec<i8>> {
    if args.len() == 0 {
        return if z_in == 0 { Some(Vec::new()) } else { None };
    }
    (1..=9)
        .filter_map(|d| {
            let z = compute_next_z(z_in, d, args[0]);
            let key = (args.len(), z);
            //let v = cache.entry(key).or_insert_with(|| maximize(z, &args[1..], cache)).clone();
            let v;
            if let Some(value) = cache.get(&key) {
                v = value.clone();
            } else {
                let value = minimize(z, &args[1..], cache);
                v = value.clone();
                cache.insert(key, value);
            }
            v.and_then(|mut vv| {
                vv.push(d);
                Some(vv)
            })
        })
        .next()
}


fn solution(input_s: &str) -> Result<()> {
    let input: Vec<Vec<&str>> = input_s
        .trim()
        .split("inp w\n")
        .skip(1)
        .map(|s| s.split("\n").collect()) //has trailing empty
        .collect();

    let expected = [
        (00, "mul x 0"),
        (01, "add x z"),
        (02, "mod x 26"),
        // (03, "div z <a1>"),
        // (04, "add x <a2>"),
        (05, "eql x w"),
        (06, "eql x 0"),
        (07, "mul y 0"),
        // (08, "add y <a3>"),
        (09, "mul y x"),
        (10, "add y 1"),
        (11, "mul z y"),
        (12, "mul y 0"),
        (13, "add y w"),
        // (14, "add y <a4>"),
        (15, "mul y x"),
        (16, "add z y"),
    ];

    for (i_sec, section) in input.iter().enumerate() {
        for (i, v) in &expected {
            let s_val = &section[*i as usize];
            if s_val == v {
                continue;
            }
            panic!(
                "Variation in section {} line {}: {} <> {}",
                i_sec, i, &v, s_val
            );
        }
    }

    let args: Vec<_> = input
        .iter()
        .map(|s| {
            [s[03], s[04], s[08], s[14]]
                .iter()
                .map(|ln| {
                    ln.split(" ")
                        .skip(2)
                        .next()
                        .unwrap()
                        .parse()
                        .with_context(|| format!("Parsing {}", ln))
                })
                .collect::<Result<Vec<i8>>>()
                .and_then(|v| Ok([v[0], v[1], v[2], v[3]]))
        })
        .collect::<Result<_>>()?;

    let mut cache = HashMap::new();
    let res = maximize(0, &args, &mut cache).unwrap();

    print!("Part 1: ");
    for x in res.iter().rev() {
        print!("{}", x);
    }
    println!("");

    let mut cache = HashMap::new();
    let res = minimize(0, &args, &mut cache).unwrap();

    print!("Part 2: ");
    for x in res.iter().rev() {
        print!("{}", x);
    }
    println!("");
    Ok(())
}

fn main() -> Result<()> {
    println!("** TEST **");
    //solution(&fs::read_to_string("test00.txt")?)?;
    println!("\n** INPUT **");
    solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
