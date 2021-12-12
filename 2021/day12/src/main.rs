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

#[derive(Debug)]
struct RouteT <'a> {
    path: Vec<&'a str>,
    at: &'a str,
    visited: HashSet<&'a str>,
    twice: Option<&'a str>,
}

impl <'a> RouteT <'a> {
    fn new(at: &'a str) -> Self {
        RouteT{at, visited: HashSet::new(), twice: None, path: vec![at]}
    }
    fn goto(&self, dst: &'a str) -> Self {
        let mut path = self.path.clone();
        path.push(dst);
        Self{
            at: dst, visited: self.visited.clone(), twice: self.twice,
            path
        }
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
    println!("Part 1: {}\n", p1);

    let mut p2: usize = 0;
    let mut routes: Vec<RouteT> = vec![RouteT::new("start")];
    while let Some(mut r) = routes.pop() {
        //println!("R: {:?}\nRoutes: {:?}", &r, &routes);
        let a = caves.get(r.at).unwrap();
        if a.is_end {
            println!("Finished route: {:?}", r);
            p2+=1;
            continue;
        }
        if a.is_small { 
            let first_time = r.visited.insert(r.at);
            if !first_time {
                r.twice = Some(r.at);
            }
        }
        routes.extend(
            a.edges.iter().filter_map(|dst| 
                if (r.visited.contains(&dst[..]) && r.twice.is_some()) || dst == "start" {
                    None
                } else {
                    Some(r.goto(dst))
                } )
        );
    }
    println!("Part 2: {}", p2);
    Ok(())
}

fn main() -> Result<()> {
    println!("** TEST **");
    solution(&fs::read_to_string("test00.txt")?)?;
    println!("\n** INPUT **");
    solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
