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

fn get_ore_requirement<'a>(fuel_requirement: isize, recipes: &Vec<Recipe<'a>>) -> isize {
    let mut materials: HashMap<_, _> = recipes.iter()
        .map(|r| (r.dst.name, Material::new(r)))
        .collect();
    for r in recipes {
        for i in &r.ingr {
            materials.entry(i.name).and_modify(|m| {m.needed_for.insert(r.dst.name);});
        }
    }

    let mut demands = Vec::new(); // work stack
    demands.push((Ingredient{name: "FUEL", amnt: fuel_requirement}, "Part one solution"));
    loop {
        if let Some((d, req)) = demands.pop() {
            let mut m = materials.get_mut(d.name).unwrap();
            m.demand += d.amnt;
            m.needed_for.remove(req);
            if m.needed_for.len()==0 {
                // all demands registered, push input demands
                let n = ceil_div(m.demand, m.recipe.dst.amnt); 
                // println!("Recipe for {} runs {} times", d.name, &n);
                demands.extend(
                    m.recipe.ingr.iter().map(|i| (
                        Ingredient{name: i.name, amnt: i.amnt*n},
                        d.name
                    ))
                );
            }
        } else { break;}
    }
    materials.get("ORE").unwrap().demand
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");
    let recipes: Vec<_> = input.lines().map(parse_recipe).chain(
        std::iter::once(Recipe{dst: Ingredient{name: "ORE", amnt: 1}, ingr: Vec::new()})
    ).collect();   

    // part 1
    let o1 = get_ore_requirement(1, &recipes);
    println!("Part 1: Ore demand: {}", o1);

    // part 2
    let otot: isize = 1000000000000;
    let mut nmin = 1;
    let mut omin = o1;
    while otot-omin > o1 {
        nmin += (otot-omin)/o1;
        omin = get_ore_requirement(nmin, &recipes); 
    }
    println!("After newton: F {} -> O: {}", nmin, omin);
    // maybe babystep a bit further
    while get_ore_requirement(nmin+1, &recipes) <= otot { nmin+=1;}
    println!("Part 2: {}", nmin);
}