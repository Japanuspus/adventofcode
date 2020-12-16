use anyhow::Error;
use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fs,
};
use parse_display::FromStr;

#[derive(Debug, FromStr)]
#[display("{name}: {v11}-{v12} or {v21}-{v22}")]
struct FieldRule {
    name: String,
    v11: isize,
    v12: isize,
    v21: isize,
    v22: isize,
}

fn check_rule(r: &FieldRule, v: isize) -> bool {
    ((v >= r.v11) & (v <= r.v12)) | ((v >= r.v21) & (v <= r.v22))
}

#[test]
fn test_check_rule() {
    let rule = FieldRule {
        name: "".to_string(),
        v11: 11,
        v12: 12,
        v21: 21,
        v22: 22,
    };
    assert!(check_rule(&rule, 11));
    assert!(check_rule(&rule, 12));
    assert!(check_rule(&rule, 21));
    assert!(check_rule(&rule, 22));

    assert!(!check_rule(&rule, 15));
}

struct Input {
    rules: Vec<FieldRule>,
    my_ticket: Vec<isize>,
    tickets: Vec<Vec<isize>>,
}

fn parse_input(input: &str) -> Result<Input> {
    let mut parts = input.split("\n\n");
    let rules = parts
        .next()
        .ok_or(Error::msg("No rules"))?
        .lines()
        .map(|ln| ln.parse::<FieldRule>())
        .collect::<Result<_, _>>()?;

    fn parse_ticket(ln: &str) -> Vec<isize> {
        ln.split(',')
            .filter_map(|s| s.parse::<isize>().ok())
            .collect()
    }

    let my_ticket = parts
        .next()
        .ok_or(Error::msg("No ticket"))?
        .lines()
        .skip(1)
        .map(parse_ticket)
        .next()
        .ok_or(Error::msg("no ticket line"))?;

    let tickets = parts
        .next()
        .ok_or(Error::msg("No other"))?
        .lines()
        .skip(1)
        .map(parse_ticket)
        .collect();

    Ok(Input {
        rules,
        tickets,
        my_ticket,
    })
}

fn part1(inputs: &str) -> Result<isize> {
    let input = parse_input(&inputs)?;

    let v = input
        .tickets
        .iter()
        .flat_map(|ticket| ticket.iter())
        .filter(|&field| !input.rules.iter().any(|rule| check_rule(rule, *field)))
        .sum::<isize>();
    Ok(v)
}

#[test]
fn test_part1() {
    let t_inputs = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    assert_eq!(part1(t_inputs).ok(), Some(71));
}

fn part2(inputs: &str) -> Result<isize> {
    let input = parse_input(&inputs)?;
    // all fields on valid tickets much mach at least one rule
    let tickets: Vec<_> = input
        .tickets
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|&field| input.rules.iter().any(|rule| check_rule(rule, field)))
        })
        .collect();

    // for each index find possible field names
    let field_names: Vec<&str> = input.rules.iter().map(|r| &r.name[..]).collect();
    let mut possible: Vec<HashSet<&str>> = (0..tickets[0].len())
        .map(|_| field_names.iter().cloned().collect())
        .collect();
    for ticket in tickets.iter() {
        for (field, pos) in ticket.iter().zip(possible.iter_mut()) {
            for name in input
                .rules
                .iter()
                .filter(|rule| !check_rule(rule, *field))
                .map(|r| &r.name[..])
            {
                pos.remove(name);
            }
        }
    }
    for (i, v) in possible.iter().enumerate() {
        println!("Possible at [{}]: {:?}", i, v);
    }

    // problem is "maximum cover" on bipartite graph, but unfortunately none of the 
    // graph tools seem to not quite cover this out of the box.
    // Looking at the input suggests a simple solution by recursively eliminating indices where only
    // one field name is possible:
    let mut solution: HashMap<&str, usize> = HashMap::new();
    while let Some((i, ps)) = possible
        .iter()
        .enumerate()
        .filter(|(_, ps)| ps.len() == 1)
        .next()
    {
        let unique = ps.iter().cloned().next().unwrap(); // field which is only valid for a single index
        solution.insert(unique, i);
        for ps in possible.iter_mut() {
            ps.remove(unique);
        }
    }
    println!("Solution: {:?}", solution);

    if possible.iter().any(|ps| ps.len() > 0) {
        Err(Error::msg(
            "Multiple possible fields for at least one after naive algorithm",
        ))
    } else {
        Ok(solution
            .iter()
            .filter(|(field, _)| field.starts_with("departure "))
            .map(|(_, &i)| input.my_ticket[i])
            .product())
    }
}

fn main() -> Result<()> {
    let inputs = fs::read_to_string("input.txt")?;
    println!("{}", part1(&inputs)?);
    println!("{}", part2(&inputs)?);

    Ok(())
}
