use anyhow::{Result, Context};
use std::{fs, collections::{HashMap, HashSet}};

#[derive(Debug)]
struct Cave {
    is_small: bool,
    is_end: bool,
    edges: Vec<String>,
}

impl Cave {
    fn from_name(name: &str) -> Self {
        let is_small=name.chars().all(|c| c.is_lowercase());
        let is_end=name=="end";
        Cave{is_small, is_end, edges: Vec::new()}
    }
    fn push(&mut self, dst: &str) {
        self.edges.push(dst.to_owned())
    }
}

#[derive(Debug)]
struct Route <'a> {
    at: &'a str,
    visited: HashSet<&'a str>,
}

impl <'a> Route <'a> {
    fn new(at: &'a str) -> Self {
        Route{at, visited: HashSet::new()}
    }
}

fn solution(input_s: &str) -> Result<()> {
    let input: Vec<Vec<&str>> = input_s
        .trim()
        .split("\n")
        .map(|s| s.split('-').collect())
        .collect();
    let mut caves: HashMap<String, Cave> = HashMap::new();
    for (a, b) in input.iter().flat_map(|v| [(v[0], v[1]), (v[1], v[0])].into_iter()) {
        caves.entry(a.to_string()).or_insert_with(|| Cave::from_name(a)).push(b)
    }
    println!("{:?}", caves);


    let mut p1: usize = 0;
    let mut routes: Vec<Route> = vec![Route::new("start")];
    while let Some(mut r) = routes.pop() {
        //println!("R: {:?}\nRoutes: {:?}", &r, &routes);
        let a = caves.get(r.at).unwrap();
        if a.is_end {
            println!("Finished route: {:?}", r);
            p1+=1;
            continue;
        }
        if a.is_small { r.visited.insert(r.at); }
        routes.extend(
            a.edges.iter().filter_map(|dst| 
                if r.visited.contains(&dst[..]) {
                    None
                } else {
                    Some(Route{at: dst, visited: r.visited.clone()})
                } )
        );
    }
    println!("Part 1: {}", p1);
    println!("Part 2: {}", 0);
    Ok(())
}

fn main() -> Result<()> {
    println!("** TEST **");
    solution(&fs::read_to_string("test00.txt")?)?;
    println!("\n** INPUT **");
    //solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
