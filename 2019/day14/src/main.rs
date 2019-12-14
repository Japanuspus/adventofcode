#![allow(unused)]

use std::collections::{HashMap, HashSet};
// use std::iter;
use itertools::Itertools;

// 1 NMTGB, 4 KCNKC, 5 SBSJ, 4 MCZDZ, 7 DLCP, 2 GRBZF, 1 CLKP, 10 VQHJG => 6 DVCR
#[derive(Debug)]
struct Ingredient<'a> { name: &'a str, amnt: isize }
#[derive(Debug)]
struct Recipe<'a> { dst: Ingredient<'a>, ingr: Vec<Ingredient<'a>> }

fn parse_ingredient(s: &str) -> Ingredient {
    let (a, b) = s.split(" ").next_tuple().unwrap();
    Ingredient {name: b, amnt: a.parse().unwrap()}
}
fn parse_recipe(s: &str) -> Recipe {
    let (a, b) = s.split(" => ").next_tuple().unwrap();
    Recipe{dst: parse_ingredient(b), ingr: a.split(", ").map(parse_ingredient).collect()}
}

#[derive(Debug)]
struct Material<'a> {
    recipe: &'a Recipe<'a>,
    demand: isize,
    needed_for: HashSet<&'a str>
}
impl<'a> Material<'a> {
    fn new(recipe: &'a Recipe) -> Self {
        Material{recipe, demand: 0, needed_for: HashSet::new()}
    }
}

fn ceil_div(n: isize, m:isize) -> isize {
    (n + (m-1))/m
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");
    let recipes: Vec<_> = input.lines().map(parse_recipe).collect();   

    let mut materials: HashMap<_, _> = recipes.iter()
        .map(|r| (r.dst.name, Material::new(r)))
        .collect();
    let ore_recipe = Recipe{dst: Ingredient{name: "ORE", amnt: 1}, ingr: Vec::new()};
    materials.insert(ore_recipe.dst.name, Material::new(&ore_recipe));
    for r in &recipes {
        for i in &r.ingr {
            materials.entry(i.name).and_modify(|m| {m.needed_for.insert(r.dst.name);});
        }
    }

    let mut demands = Vec::new();
    demands.push((Ingredient{name: "FUEL", amnt: 1}, "Part one solution"));
    loop {
        if let Some((d, req)) = demands.pop() {
            let mut m = materials.get_mut(d.name).unwrap();
            m.demand += d.amnt;
            !m.needed_for.remove(req);
            if m.needed_for.len()==0 {
                // all demands registered, push input demands
                let n = ceil_div(m.demand, m.recipe.dst.amnt); 
                println!("Recipe for {} runs {} times", d.name, &n);
                demands.extend(
                    m.recipe.ingr.iter().map(|i| (
                        Ingredient{name: i.name, amnt: i.amnt*n},
                        d.name
                    ))
                );
            }
        } else {
            break;
        }
    }
    //dbg!(materials);
    println!("Part 1: Ore demand: {}", materials.get("ORE").unwrap().demand);
}