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

struct Board {
    nx: isize,
    ny: isize,
}
impl Board {
    fn is_inner(&self, p: &Pos) -> bool {
        let eps: isize = 4;
        p.0 > eps && p.0 < self.nx-eps && p.1 > eps && p.1 < self.ny-eps
    }
}

fn parse_maze(input: &str) -> (Board, HashMap<String, Vec<Pos>>, HashMap<Pos, Vec<Pos>>) {

    let cs: Vec<Vec<char>> = input.lines().map(|ln| ln.chars().collect()).collect();
    fn get_c(cs: &Vec<Vec<char>>, p: &Pos) -> char {cs[p.1 as usize][p.0 as usize]};
    let board = Board{nx: cs[0].len() as isize, ny: cs.len() as isize};

    let mut portals: HashMap<String, Vec<Pos>> = HashMap::new();
    let mut cgraph: HashMap<Pos, Vec<Pos>> = input
    .lines()
    .enumerate()
    .flat_map(|(y, ln)| {
        ln.chars().enumerate().filter_map(move |(x, c)| if c=='.' {Some((x as isize,y as isize))} else {None})
    })
    .map(|p| {
        let mut cons = Vec::new();
        for d in (0..4) {
            let p1 = dp(&p, d);
            match get_c(&cs, &p1) {
                '.' => {cons.push(p1);}
                '#' => {}
                c => {
                    // All portal names appear LR or UD
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
    })
    .collect();
    (board, portals, cgraph)
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");
     
    let (board, portals, mut cgraph) = parse_maze(&input);
    let aa = portals.get("AA").unwrap()[0];
    let zz = portals.get("ZZ").unwrap()[0];
    let cgraph_no_portals = cgraph.clone();

    // Part 1
    for (n,pts) in portals.iter() {
        if pts.len()>1 {
            cgraph.get_mut(&pts[0]).unwrap().push(pts[1].clone());
            cgraph.get_mut(&pts[1]).unwrap().push(pts[0].clone());
        } else {
            println!("single portal: {}: {:?}", &n, &pts)
        }
    }

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
    println!("Part 1: {}", dists.get(&zz).unwrap());

    // Part 2
    // connections: (other: Pos, dz: isize) dz==1 if going to deeper level (using inner portal)
    let mut cgraph: HashMap<Pos, Vec<(Pos, isize)>> = 
        cgraph_no_portals
        .into_iter()
        .map(|(node, cons)| (node, cons.into_iter().map(|c| (c, 0)).collect()))
        .collect();
    for (id, cs) in portals.iter().filter(|(id, p)| p.len()>1) {
        let cio = if board.is_inner(&cs[0]) {
            (&cs[0], &cs[1])
        } else {
            (&cs[1], &cs[0])
        };
        assert!(board.is_inner(cio.0));
        assert!(!board.is_inner(cio.1));
        cgraph.get_mut(cio.0).unwrap().push((cio.1.clone(), 1));
        cgraph.get_mut(cio.1).unwrap().push((cio.0.clone(), -1));
    }
    // BFS from AA to ZZ along implicit graph
    let mut dists = HashMap::new();
    let mut queue = VecDeque::new();
    let zz0 = (zz.clone(), 0);
    let mut d = 0;
    dists.insert((aa.clone(), 0), 0);
    queue.push_back((aa.clone(), 0isize));
    while let Some(p0) = queue.pop_front() {
        d = *dists.get(&p0).unwrap();
        if p0==zz0 {
            break
        }
        for p1 in cgraph
        .get(&p0.0).unwrap().iter()
        .map(|(pt, dz)| (pt.clone(), dz+p0.1))
        .filter(|p| p.1>=0) {
            if dists.get(&p1).is_none() {
                dists.insert(p1.clone(), d+1);
                queue.push_back(p1);
            }
        }
    }
    println!("Part 2: {}", d);
}






















