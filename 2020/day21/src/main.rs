use std::{collections::{HashMap, HashSet}, fs, iter::repeat};
use anyhow::Result;
// use anyhow::Context;
// use itertools::Itertools;
// use parse_display::{FromStr};
use regex::Regex;
// use apply::Also;
// use num::{BigInt, Integer};

//#[derive(Debug, FromStr)]
//#[display("{key}:{value}")]
//struct InputItem {key: String, value: String}

#[derive(Debug)]
struct Recipe<'a>{ingredients: HashSet<&'a str>, allergens: HashSet<&'a str>}

fn main() -> Result<()> {
    let input = fs::read_to_string("input_part1.txt")?;

    // input
    let mut recipes: Vec<Recipe> = Vec::new();
    let re = Regex::new(r"^(?P<ingredients>[\w ]+) \(contains (?P<allergens>[\w ,]+)\)$" )?;
    for ln in input.lines() {
        if let Some(c) = re.captures(ln) {
            recipes.push(Recipe{
                ingredients: c.name("ingredients").unwrap().as_str().split(" ").collect(),
                allergens: c.name("allergens").unwrap().as_str().split(", ").collect(),
            });
        } else {
            println!("unmarked line: {}", ln);
        }
    }
    let recipes = recipes;
    println!("parsed {} recipes", recipes.len());

    // part 1
    let allergens: HashSet<&str> = recipes.iter().flat_map(|r| r.allergens.iter().cloned()).collect();
    let ingredients: HashSet<&str> = recipes.iter().flat_map(|r| r.ingredients.iter().cloned()).collect();
    
    let mut pa: HashMap<&str, HashSet<&str>> = ingredients.iter().cloned().zip(repeat(allergens.clone())).collect();
    for r in recipes.iter() {
        let missing_allergens: Vec<&str> = allergens.difference(&r.allergens).cloned().collect();
        for ingr in r.ingredients.iter() {
            let possible_allergens = pa.get_mut(ingr).unwrap();
            for a in missing_allergens.iter() {
                possible_allergens.remove(a);
            }
        }
    }
    for (k,v) in pa.iter() { println!("{} may contain {:?}", k, v);}

    let safe_ingredients: HashSet<&str> = pa.iter().filter(|(_, a)| a.len()==0).map(|(ingr, _)| ingr).cloned().collect();
    
    println!("Part 1: {}, safe: {:?}", 
        recipes.iter().map(|r| r.ingredients.intersection(&safe_ingredients).count()).sum::<usize>(),
        safe_ingredients);

    Ok(())
}
