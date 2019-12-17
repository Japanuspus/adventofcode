#![allow(unused)]

// use std::collections::HashSet;
use std::collections::HashMap;
// use std::iter;
use day11::State; // dep: day11={path="../day11"}

type Pos = (isize, isize);

fn dp(p: &Pos, d: isize) -> Pos {
    match d {
        1 => (p.0, p.1-1),
        2 => (p.0, p.1+1),
        3 => (p.0-1, p.1),
        4 => (p.0+1, p.1),
        _ => {panic!("bad direction")}
    }
}

fn opp(d: isize) -> isize {
    match d {
        1 => 2,
        2 => 1, 
        3 => 4, 
        4 => 3, 
        _ => {panic!("bad direction")}
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");
    let mut s = State::from(input);
    
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut res = Vec::new();
    let mut mapstr = Vec::new();
    while let Some(v) = s.next_number_callback(|| None).unwrap() {
        mapstr.push(v as u8);
        if v<32 {
            if v==10 {
                x=0;
                y+=1;
            } else {
                panic!();
            }
        } else {
            res.push(((x, y), v));
            x+=1;
        }
    }
    let res = res; //unmut
    println!("Map: \n{}", std::str::from_utf8(&mapstr).unwrap());

    let map: HashMap<_,_> = res.iter().map(|(p,v)| (p.clone(), *v as u8)).collect();
    let mut coord_sum = 0;
    for (p, v) in &map {
        if *v != b'.' {
            // possible intersection
            let nb: usize = (1..=4)
                .filter_map(|d| 
                    map
                    .get(&dp(&p, d))
                    .and_then(|v| if *v!=b'.' {Some(1)} else {None})
                )
                .count();
            
            //dbg!(nb);
            if nb>2 {
                dbg!((&p,&v, &nb));
                let coord_num = p.0*p.1;
                coord_sum += coord_num;
            }
        }
    }
    println!("Part 1: {}", coord_sum);
}