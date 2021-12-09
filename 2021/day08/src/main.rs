use anyhow::{Result, Context};
use itertools::Itertools;
use std::fs;
use std::collections::{BTreeSet, HashMap};


type CSet = BTreeSet<char>;  // hashable, unlike HashSet
fn str2cset(s: &str) -> CSet {
    s.chars().collect()
}

fn solve_line(v_in: &[&str], v_out: &[&str], digit_to_dec: &HashMap<CSet, u8>) -> usize {
    //println!("Inputs: {:?} -> {:?}", v_in, v_out);
    // map from group length to intersection of occurrences
    let mut map: HashMap<usize, CSet> = HashMap::new();
    for v in v_in.iter().cloned() {
        let cc = map.entry(v.len()).or_insert(str2cset("abcdefg"));
        *cc = cc.intersection(&str2cset(v)).cloned().collect();
    }
    //println!("Intersections by length of group: {:?}", map);

    // "cf",     //lengt 2
    let cf: &CSet = &map[&2usize];
    // "acf",    //lengt 3
    let acf: &CSet = &map[&3usize];
    // "bcdf",   //lengt 4
    let bcdf: &CSet = &map[&4usize];
    // "acdeg",  //lengt 5
    // "acdfg",  //lengt 5
    // "abdfg",  //lengt 5
    let adg: &CSet = &map[&5usize];
    // "abcefg", //lengt 6
    // "abdefg", //lengt 6
    // "abcdfg", //lengt 6
    let abfg: &CSet = &map[&6usize];
    // "abcdefg",//lengt 7
    let abcdefg: &CSet = &map[&7usize];
    
    let a: CSet = acf.difference(cf).cloned().collect();
    let bd: CSet = bcdf.difference(cf).cloned().collect();
    let dg: CSet = adg.difference(&a).cloned().collect();
    let d: CSet = bd.intersection(&dg).cloned().collect();
    let b: CSet = bd.difference(&d).cloned().collect();
    let g: CSet = dg.difference(&d).cloned().collect();
    let f: CSet = abfg.intersection(cf).cloned().collect();
    let c: CSet = cf.difference(&f).cloned().collect();
    let cde: CSet = abcdefg.difference(abfg).cloned().collect();
    let de: CSet = cde.difference(&c).cloned().collect();
    let e: CSet = de.difference(&d).cloned().collect();
    
    let mut links: HashMap<char, char> = HashMap::new();
    for (i, k_set) in [a, b, c, d, e, f, g].into_iter().enumerate() {
        assert!(k_set.len() == 1);
        let k = k_set.into_iter().next().unwrap();
        let v = (b'a'+(i as u8)) as char;
        if let Some(_old_v) = links.insert(k, v) {
            panic!("Duplicate entry")
        }
    };
    //println!("Resolved map: {:?}", &links);

    v_out.iter()
    .map(|v| v.chars().map(|c| links.get(&c).unwrap()).cloned().collect::<String>())
    .map(|s| digit_to_dec.get(&str2cset(&s)).unwrap())
    .fold(0usize, |a, d| a*10+(*d as usize))
}

fn parse(input_s: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    input_s
    .trim()
    .split("\n")
    .map(|s| {
        let mut p=s.split(" | ");
        (p.next().unwrap().split_whitespace().collect_vec(), 
            p.next().unwrap().split_whitespace().collect_vec())
    })
    .collect()
}

fn make_digit_to_dec() -> HashMap<CSet, u8> {
    [
        "abcefg", //0 - 6
        "cf",     //1 - 2
        "acdeg",  //2 - 5
        "acdfg",  //3 - 5
        "bcdf",   //4 - 4
        "abdfg",  //5 - 5
        "abdefg", //6 - 6
        "acf",    //7 - 3
        "abcdefg",//8 - 7
        "abcdfg", //9 - 6
    ].iter()
    .enumerate()
    .map(|(i, s)| (str2cset(s), i as u8))
    .collect()
}

fn solution(input:  &[(Vec<&str>, Vec<&str>)]) -> Result<(usize, usize)> {
    // unique lengths: 2, 4, 3, 7
    let p1=input.iter().map(|(_a, b)| b.iter().map(|d| d.len()).filter(|n| (*n==2) || (*n==3) || (*n==4) || (*n==7 )).count()).sum();
    println!("Part 1: {}", p1);

    // part 2
    let digit_to_dec = make_digit_to_dec();
    let p2 = input.iter().map(|vs| solve_line(&vs.0, &vs.1, &digit_to_dec)).sum();
    println!("Part 2: {}", &p2);
    Ok((p1, p2))
}

#[test]
fn test_solution() -> Result<()> {
    let (a, b) = solution(&parse(&fs::read_to_string("test01.txt")?))?;
    assert!(b==5353);
    Ok(())
}

fn main() -> Result<()> {
    println!("*** Test ****");
    solution(&parse(&fs::read_to_string("test02.txt")?))?;
    println!("\n*** Input ****");
    solution(&parse(&fs::read_to_string("input.txt")?))?;
    Ok(())
}
