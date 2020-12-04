use std::collections::HashSet;

use aoclib;
use parse_display::{FromStr};
use regex::Regex;

#[derive(Debug, FromStr)]
#[display("{key}:{value}")]
struct KeyVal {key: String, value: String}

struct PPChecker {ecl: Regex, pid: Regex, hcl: Regex,}
impl PPChecker {
    fn new() -> Self {Self{
        ecl: Regex::new(r"^(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)$").unwrap(),
        pid: Regex::new(r"^[0-9]{9}$").unwrap(),
        hcl: Regex::new(r"^#[0-9a-f]{6}$").unwrap(),
    }}

    fn check(&self, pp: &Vec<KeyVal>)-> bool {
        fn check_year<T: AsRef<str>>(s: T, ymin: usize, ymax: usize) -> bool {
            s.as_ref().len()==4 && s.as_ref().parse::<usize>().map(|v| v>=ymin && v<=ymax).unwrap_or(false)
        }

        pp.iter().all(|kv| {
            match &(kv.key)[..] {
                "byr" => check_year(&kv.value, 1920, 2002),
                "iyr" => check_year(&kv.value, 2010, 2020),
                "eyr" => check_year(&kv.value, 2020, 2030),
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
                "hcl" => self.hcl.is_match(&kv.value),
                "ecl" => self.ecl.is_match(&kv.value),
                "pid" => self.pid.is_match(&kv.value),
                _ => true,
            }
        })
    }
}

#[test]
fn test_pp_valid() {
    let ppc = PPChecker::new();
    assert_eq!(ppc.check(&vec![KeyVal{key: "byr".to_string(), value: "2002".to_string()}]), true);

    assert_eq!(ppc.check(&vec![KeyVal{key: "ecl".to_string(), value: "blu".to_string()}]), true);
    assert_eq!(ppc.check(&vec![KeyVal{key: "ecl".to_string(), value: "blx".to_string()}]), false);

    assert_eq!(ppc.check(&vec![KeyVal{key: "hcl".to_string(), value: "#123abc".to_string()}]), true);
    assert_eq!(ppc.check(&vec![KeyVal{key: "hcl".to_string(), value: "#123abz".to_string()}]), false);
    assert_eq!(ppc.check(&vec![KeyVal{key: "hcl".to_string(), value: "#123abcz".to_string()}]), false);

    assert_eq!(ppc.check(&vec![KeyVal{key: "hgt".to_string(), value: "149cm".to_string()}]), false);
    assert_eq!(ppc.check(&vec![KeyVal{key: "hgt".to_string(), value: "150cm".to_string()}]), true);
    assert_eq!(ppc.check(&vec![KeyVal{key: "hgt".to_string(), value: "76in".to_string()}]), true);
    assert_eq!(ppc.check(&vec![KeyVal{key: "hgt".to_string(), value: "77in".to_string()}]), false);
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

    let ppc = PPChecker::new();
    println!("Part 2: {}", pps.iter().filter(|pp| {
        let keys: HashSet<&str>=pp.iter().map(|kv| &kv.key[..]).collect();
        rkeys.is_subset(&keys) && ppc.check(pp)
    }).count());

    Ok(())
}
