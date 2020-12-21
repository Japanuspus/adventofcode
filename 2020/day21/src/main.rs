use std::{collections::{HashMap, HashSet}, fs};
use anyhow::Result;
use itertools::{Itertools};
use regex::Regex;

#[derive(Debug)]
struct Recipe<'a>{ingredients: HashSet<&'a str>, allergens: HashSet<&'a str>}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

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
    
    // each allergen can only appear in one ingredient, an ingredient can at most have one allergen
    // map allergen to possible sources: intersection of ingr from recipes where allergen is listed
    let possible_sources: HashMap<&str, HashSet<&str>> = allergens.iter().map(|a| (a.clone(), {
        recipes.iter()
        .filter(|r| r.allergens.contains(a))
        .map(|r| &r.ingredients)
        .fold(ingredients.clone(), |acc, ingr| acc.intersection(ingr).cloned().collect())
    })).collect();

    let mut ps = possible_sources.clone();
    let mut sources: HashMap<&str, &str> = HashMap::new();
    while let Some((&a, is)) = ps.iter().filter(|(_, v)| v.len()==1).next() {
        let ingr: &str = is.iter().next().unwrap().clone();
        sources.insert(a, ingr);
        ps.remove(&a);
        for (_, s) in ps.iter_mut() {s.remove(&ingr);}
    }
    assert_eq!(ps.len(), 0); // non-resolved
    println!("Source map: {:?}", sources);

    let unsafe_ingredients: HashSet<&str> = sources.iter().map(|(_, v)| v).cloned().collect();
    
    println!("Part 1: {}", 
        recipes.iter().map(|r| r.ingredients.difference(&unsafe_ingredients).count()).sum::<usize>());

    // part 2 was accidentally solved above...
    println!("Part 2: {}", 
        allergens.iter().sorted().map(|a| sources.get(a).unwrap()).join(","));

    Ok(())
}
