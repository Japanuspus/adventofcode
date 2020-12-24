use anyhow::Result;
use apply::Apply;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
    num::NonZeroU16,
};
use vecmath::vec3_add;

type XYZ = [i16; 3];

fn parse_steps(s: &str) -> Result<Vec<Vec<XYZ>>> {
    let re = Regex::new(r"(e)|(se)|(sw)|(w)|(nw)|(ne)")?;
    s.lines()
        .map(|ln| {
            re.find_iter(ln)
                .map(|dd| match dd.as_str() {
                    "e" => [1, -1, 0],
                    "ne" => [1, 0, -1],
                    "nw" => [0, 1, -1],
                    "w" => [-1, 1, 0],
                    "sw" => [-1, 0, 1],
                    "se" => [0, -1, 1],
                    _ => panic!("Unknown direction"),
                })
                .collect()
        })
        .collect::<Vec<Vec<XYZ>>>()
        .apply(Ok)
}

const NBS: [XYZ; 6] = [
    [1, -1, 0],
    [1, 0, -1],
    [0, 1, -1],
    [-1, 1, 0],
    [-1, 0, 1],
    [0, -1, 1],
];

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let flips = parse_steps(&input)?;

    let flipped0: HashSet<XYZ> = {
        let mut flipcount: HashMap<XYZ, u16> = HashMap::new();
        for line in flips.iter() {
            let f = line
                .iter()
                .fold([0, 0, 0], |acc, step| vec3_add(acc, step.clone()));
            flipcount.entry(f).and_modify(|e| *e += 1).or_insert(1);
        }
        flipcount
            .into_iter()
            .filter(|(_, ct)| (*ct % 2) == 1)
            .map(|(p, _)| p)
            .collect()
    };
    println!("Part 1: {}", flipped0.len());

    let mut flipped = flipped0.clone();
    for round in 0..100 {
        let field: HashSet<XYZ> = flipped
            .iter()
            .flat_map(|p| NBS.iter().map(move |nb| vec3_add(nb.clone(), p.clone())))
            .collect();
        flipped = field
            .into_iter()
            .filter(|p| {
                let ct = NBS
                    .iter()
                    .map(|nb| vec3_add(nb.clone(), p.clone()))
                    .filter(|np| flipped.contains(np))
                    .count();
                if flipped.contains(p) {
                    // Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
                    !((ct == 0) | (ct > 2))
                } else {
                    //Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
                    ct == 2
                }
            })
            .collect();
        println!("Round {}: {}", round + 1, flipped.len());
    }

    Ok(())
}
