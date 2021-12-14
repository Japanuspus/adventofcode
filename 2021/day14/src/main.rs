use anyhow::{Result, Context};
use std::{fs, collections::HashMap, iter::once};

use parse_display::{Display, FromStr};

// #[derive(Display, FromStr, PartialEq, Debug)]
// enum Direction {
//     #[display("forward")]
//     Forward,
// }

#[derive(Debug, Display, FromStr)]
 #[display("{a}{b} -> {c}")]
 #[from_str(regex = r#"(?P<a>\w)(?P<b>\w) -> (?P<c>\w)"#)]
struct Rule {
    a: char,
    b: char,
    c: char,
}


struct Input {
    template: Vec<u8>,
    rules: Vec<Rule>,
}

fn parse(input_s: &str) -> Result<Input> {
    let mut input = input_s.trim().split("\n\n");
    let template = input.next().unwrap().as_bytes().to_owned();
    let rules: Vec<Rule> = input.next().unwrap()
        .split("\n")
        .map(|s| s.parse().with_context(|| format!("Parsing {}", s)))
        .collect::<Result<_,_>>()?;
    Ok(Input{template, rules})
}

fn apply_rules(p: &[u8], lookup: &HashMap::<[u8;2], [u8;3]>) -> Vec<u8> {
    once(p[0])
    .chain(
        p.windows(2)
        .flat_map(|w| lookup.get(w).and_then(|rep| Some(rep[1..].iter())).unwrap_or(w[1..].iter()))
        .cloned()  
    )
    .collect()
}

fn part1(input: &Input) {
    let lookup: HashMap::<[u8;2], [u8;3]> = input.rules.iter().map(|r| ([r.a as u8, r.b as u8], [r.a as u8, r.c as u8, r.b as u8])).collect();

    let mut s10 = input.template.iter().cloned().collect::<Vec<_>>();
    for i in 0..10 {
        s10 = apply_rules(&s10, &lookup);
        //println!("Pol {} {}: {}", i+1, s10.len(), std::str::from_utf8(&s10)?);
    }
    println!("S10 length: {}", s10.len());

    let mut cts: HashMap<u8, isize> = HashMap::new();
    for c in s10.iter() {
        *(cts.entry(*c).or_insert(0)) +=1
    }
    println!("Bins: {:?}", &cts);
    let mut cts_sorted: Vec<_> = cts.iter().collect();
    cts_sorted.sort_by_key(|(_c, v)| *v);
    let p1 = cts_sorted[cts.len()-1].1-cts_sorted[0].1;

    println!("Part 1: {}", p1);
}

// work on pair counts. will miss one count at ends
fn part2(input: &Input) {
    let pair_map: HashMap::<[u8;2], [[u8;2];2]> = input.rules.iter()
    .map(|r| ([r.a as u8, r.b as u8], [[r.a as u8, r.c as u8], [r.c as u8, r.b as u8]])).collect();

    let mut pair_count: HashMap::<[u8;2], usize> = HashMap::new();
    for p in input.template.windows(2) {
        *(pair_count.entry([p[0], p[1]]).or_insert(0))+=1;
    } 
    for _ in 0..40 {
        let mut pc = HashMap::new();
        for (p, n) in pair_count.iter() {
            for pair in pair_map.get(p).and_then(|v| Some(v.iter())).unwrap_or([*p].iter()) {
                *pc.entry(*pair).or_insert(0)+=n;
            }
        }
        pair_count = pc;
    }
    // add ends
    *pair_count.entry([input.template[0], input.template[input.template.len()-1]]).or_insert(0)+=1;
    // count character (doubled)
    let mut cts: HashMap<u8, isize> = HashMap::new();
    for (cs, v) in pair_count.iter() {
        for c in cs {
            *(cts.entry(*c).or_insert(0)) +=*v as isize;
        }
    }

    let mut cts_sorted: Vec<_> = cts.iter().collect();
    cts_sorted.sort_by_key(|(_c, v)| *v);
    let p2 = (cts_sorted[cts.len()-1].1-cts_sorted[0].1)/2;

    println!("Part 2: {}", p2);
}



fn main() -> Result<()> {
    println!("** TEST **");
    part1(&parse(&fs::read_to_string("test00.txt")?)?);
    part2(&parse(&fs::read_to_string("test00.txt")?)?);
    println!("\n** INPUT **");
    part1(&parse(&fs::read_to_string("input.txt")?)?);
    part2(&parse(&fs::read_to_string("input.txt")?)?);
    Ok(())
}
