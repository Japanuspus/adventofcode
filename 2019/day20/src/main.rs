#![allow(unused)]

// use std::collections::HashSet;
use std::collections::{HashMap, VecDeque};
// use std::iter;
// use day11::State; // dep: day11={path="../day11"}

type Pos = (isize, isize);
fn dp(p: &Pos, d: usize) -> Pos {
    match d {
        0 => (p.0+1, p.1), //>
        1 => (p.0, p.1-1), //^
        2 => (p.0-1, p.1), //<
        3 => (p.0, p.1+1), //v
        _ => {panic!("Bad direction");}
    }
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");
     
    let cs: Vec<Vec<char>> = input.lines().map(|ln| ln.chars().collect()).collect();
    fn get_c(cs: &Vec<Vec<char>>, p: &Pos) -> char {cs[p.1 as usize][p.0 as usize]}

    //let mut nodes: HashMap<Pos, usize>;
    let mut portals: HashMap<String, Vec<Pos>> = HashMap::new();
    let mut cgraph: HashMap<Pos, Vec<Pos>> = input
    .lines()
    .enumerate()
    .flat_map(|(y, ln)| {
        ln.chars().enumerate().filter_map(move |(x, c)| if c=='.' {Some((x as isize,y as isize))} else {None})
    }).map(|p| {
        let mut cons = Vec::new();
        for d in (0..4) {
            let p1 = dp(&p, d);
            match get_c(&cs, &p1) {
                '.' => {cons.push(p1);}
                '#' => {}
                c => {
                    let portal_name: String = if {d==1 || d==2} {
                        [get_c(&cs, &dp(&p1, d)), c]
                    } else {
                        [c, get_c(&cs, &dp(&p1, d))]
                    }.iter().collect();
                    portals.entry(portal_name).or_insert_with(Vec::new).push(p.clone());
                } 
            }
        }
        (p, cons)            
    }).collect();
    for (n,pts) in portals.iter() {
        if pts.len()>1 {
            cgraph.get_mut(&pts[0]).unwrap().push(pts[1].clone());
            cgraph.get_mut(&pts[1]).unwrap().push(pts[0].clone());
        } else {
            println!("single portal: {}: {:?}", &n, &pts)
        }
    }
    //dbg!(&portals);
    let aa = portals.get("AA").unwrap()[0];
    let zz = portals.get("ZZ").unwrap()[0];

    //flood fill dists
    let mut dists = HashMap::new();
    let mut queue = VecDeque::new();
    dists.insert(aa.clone(), 0);
    queue.push_back((0isize, aa.clone()));
    while let Some((d, p)) = queue.pop_front() {
        for p1 in cgraph.get(&p).unwrap().iter() {
            if dists.get(p1).is_none() {
                dists.insert(p1.clone(), d+1);
                queue.push_back((d+1, p1.clone()));
            }
        }
    }
    //dbg!(&dists);
    println!("Part 1: {}", dists.get(&zz).unwrap());
}






















