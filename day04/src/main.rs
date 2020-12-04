use std::collections::HashSet;

use aoclib;
use parse_display::{FromStr};
use regex::Regex;

#[derive(Debug, FromStr)]
#[display("{key}:{value}")]
struct KeyVal {key: String, value: String}

fn pp_valid(pp: &Vec<KeyVal>)-> bool {
    let ecl = Regex::new(r"(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)").unwrap();
    let pid = Regex::new(r"[0-9]{9}").unwrap();
    let hcl = Regex::new(r"#[0-9a-f]{6}").unwrap();

    pp.iter().all(|kv| {
        match &(kv.key)[..] {
            "byr" => kv.value.parse::<usize>().map(|v| v>=1920 && v<=2002).unwrap_or(false),
            "iyr" => kv.value.parse::<usize>().map(|v| v>=2010 && v<=2020).unwrap_or(false),
            "eyr" => kv.value.parse::<usize>().map(|v| v>=2020 && v<=2030).unwrap_or(false),
            "hgt" => {
                let n = kv.value.len();
                n>2 && kv.value[..n-2].parse::<usize>().map(|v| {
                    match &kv.value[n-2..] {
                        "cm" => v>=150 && v<=193,
                        "in" => v>=59 && v<=76,
                        _ => false,
                    }
                }).unwrap_or(false)
            },
            "hcl" => hcl.is_match(&kv.value),
            "ecl" => ecl.is_match(&kv.value),
            "pid" => pid.is_match(&kv.value),
            _ => true,
        }
    })
}

#[test]
fn test_pp_valid() {
    assert_eq!(pp_valid(&vec![KeyVal{key: "byr".to_string(), value: "2002".to_string()}]), true);

    assert_eq!(pp_valid(&vec![KeyVal{key: "ecl".to_string(), value: "blu".to_string()}]), true);
    assert_eq!(pp_valid(&vec![KeyVal{key: "ecl".to_string(), value: "blx".to_string()}]), false);

    assert_eq!(pp_valid(&vec![KeyVal{key: "hgt".to_string(), value: "150cm".to_string()}]), true);
    assert_eq!(pp_valid(&vec![KeyVal{key: "hgt".to_string(), value: "76in".to_string()}]), true);
}

fn main() -> aoclib::Result<()> {
    let input = aoclib::get_inputs_pwd()?;
    let pps: Vec<Vec<KeyVal>> = input
    .split("\n\n").map(|ln| 
        ln.split(|c| c==' ' || c=='\n').filter_map(|s| s.parse().ok()).collect()
    ).collect();

    let rkeys: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].into_iter().collect();
    
    println!("Part 1: {}", pps.iter().filter(|pp| {
        let keys: HashSet<&str>=pp.iter().map(|kv| &kv.key[..]).collect();
        rkeys.is_subset(&keys)
    }).count());

    println!("Part 2: {}", pps.iter().filter(|pp| {
        let keys: HashSet<&str>=pp.iter().map(|kv| &kv.key[..]).collect();
        rkeys.is_subset(&keys) && pp_valid(pp)
    }).count());

    Ok(())
}
