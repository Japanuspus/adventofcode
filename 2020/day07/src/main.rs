use anyhow::{Context, Error, Result};
use std::{
    collections::{HashMap, HashSet},
    fs,
};
// use itertools::Itertools;
use parse_display::FromStr;

#[derive(Debug, FromStr)]
#[from_str(regex = r"(?P<count>[0-9]+) (?P<bag_type>\w+ \w+) (bag|bags).?")]
struct HoldsSpec {
    bag_type: String,
    count: usize,
}
#[derive(Debug)]
struct RuleSpec {
    bag_type: String,
    holds: Vec<HoldsSpec>,
}

fn parse_line(ln: &str) -> Result<RuleSpec> {
    let mut parts = ln.split(" bags contain ");
    let bag_type = parts
        .next()
        .ok_or("no bag_type")
        .map_err(Error::msg)?
        .to_string();
    let holds = parts
        .next()
        .ok_or("no holds")
        .map_err(Error::msg)?
        .split(", ")
        .filter_map(|s| {
            if s == "no other bags." {
                None
            } else {
                Some(s.parse::<HoldsSpec>())
            }
        })
        .collect::<Result<Vec<_>, _>>()
        .with_context(|| format!("While parsing holds of {}", ln))?;
    Ok(RuleSpec { bag_type, holds })
}

fn main() -> Result<()> {
    let rule_specs: Vec<_> = fs::read_to_string("input.txt")?
        .lines()
        .map(parse_line)
        .collect::<Result<_, _>>()?;

    let mut holdable_by: HashMap<&str, Vec<&str>> = HashMap::new();
    for rule in rule_specs.iter() {
        for h in rule.holds.iter() {
            holdable_by
                .entry(&h.bag_type)
                .or_default()
                .push(&rule.bag_type);
        }
    }
    let holdable_by = holdable_by;

    let mut holders: HashSet<&str> = HashSet::new();
    let mut stack: Vec<&str> = vec!["shiny gold"];
    loop {
        if let Some(v) = stack.pop() {
            if let Some(v_holdable_by) = holdable_by.get(v) {
                for h in v_holdable_by.iter() {
                    if holders.insert(h) {
                        stack.push(h)
                    }
                }
            } else {
                continue;
            }
        } else {
            break;
        }
    }
    println!("Part 1: {}", holders.len());

    fn price(bag_type: &str, child_map: &HashMap<&str, &Vec<HoldsSpec>>) -> usize {
        child_map
            .get(bag_type)
            .unwrap()
            .iter()
            .map(|c| c.count * (1 + price(&c.bag_type, child_map)))
            .sum()
    }
    println!(
        "Part 2: {}",
        price(
            "shiny gold",
            &rule_specs
                .iter()
                .map(|rs| (&rs.bag_type[..], &rs.holds))
                .collect()
        )
    );

    Ok(())
}
