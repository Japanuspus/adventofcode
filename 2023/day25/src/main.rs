#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use keyed_priority_queue::KeyedPriorityQueue;
use std::{fs, time::Instant, collections::{HashMap, HashSet}, sync::Arc};
use itertools::Itertools;

type Node=[u8;3];
type Edge=[Node;2];

#[derive(Debug, Default)]
struct AdjacencyEntry {
    nbs: HashSet<Node>,
    overweight: u32, // number of nodes merged into this
}

struct Graph {
    al: HashMap<Node, AdjacencyEntry>,
    edge_weights: HashMap<Edge, u32>,
}

fn edge(a: Node, b: Node) -> Edge {
    if a<b {[a,b]} else {[b,a]}
}

impl Graph {
    fn new() -> Self {
        Self{ al: HashMap::new(), edge_weights: HashMap::new()}
    }

    fn add_edge_weight(&mut self, a: Node, b: Node, w_add: u32) {
        self.al.entry(a).or_default().nbs.insert(b);
        self.al.entry(b).or_default().nbs.insert(a);
        *self.edge_weights.entry(edge(a,b)).or_default() += w_add;
    }

    /// merge node b into a
    fn merge(&mut self, a: Node, b: Node) {
        let bn = self.al.remove(&b).unwrap();
        self.al.get_mut(&a).unwrap().overweight += 1+bn.overweight; 
        for nb in bn.nbs {
            self.al.get_mut(&nb).unwrap().nbs.remove(&b);
            let w = self.edge_weights.remove(&edge(b, nb)).unwrap();
            if nb==a {continue}
            self.add_edge_weight(a, nb, w);
        }
    }

    fn len(&self) -> usize {
        self.al.len()
    }
}

#[test]
fn test_graph() {
    let mut g = Graph::new();
    g.add_edge_weight([0,0,1], [0,0,2], 1);
    g.add_edge_weight([0,0,1], [0,0,3], 1);
    g.add_edge_weight([0,0,2], [0,0,3], 1);
    assert_eq!(g.len(), 3);

    g.merge([0,0,2], [0,0,3]);
    assert_eq!(g.len(), 2);
    assert_eq!(g.edge_weights.len(), 1);

    assert_eq!(g.edge_weights[&edge([0,0,1],[0,0,2])], 2);
    assert_eq!(g.al[&[0,0,2]].overweight, 1);
    assert_eq!(g.al[&[0,0,1]].overweight, 0);
}

fn solution(input_s: &str) -> Result<[String; 2]> {
    let mut g: Graph = Graph::new();
    for s in input_s.trim_end().split("\n") {
        let a = s.as_bytes()[..3].try_into().unwrap();
        for b_s in s.as_bytes()[5..].split(|b| *b==b' ') {
            g.add_edge_weight(a, b_s.try_into().with_context(|| format!("parsing: {:?} in {}", &b_s, s)).unwrap(), 1);
        }
    };
    let n0 = g.len();

    // Stoer–Wagner following networkx impl 
    // https://github.com/networkx/networkx/blob/main/networkx/algorithms/connectivity/stoerwagner.py
    let mut part1 = 0;
    for _ in 0..g.len()-1 {
        let u = *g.al.keys().next().unwrap();
        let mut a: HashSet<Node> = [u,].into_iter().collect();

        let mut h = KeyedPriorityQueue::new();
        for v in &g.al[&u].nbs {
            h.push(*v, g.edge_weights[&edge(u, *v)]);
        }

        let u = loop {
            let (u, _) = h.pop().unwrap();
            a.insert(u);
            for v in &g.al[&u].nbs {
                if a.contains(v) {continue}
                let dw = g.edge_weights[&edge(u, *v)];
                if let Some(w0) = h.get_priority(v).cloned() {
                    h.set_priority(v, w0+dw).unwrap();
                } else {
                    h.push(*v, dw);
                }
            }
            if a.len()+1==g.len() {break u}
        };

        assert_eq!(h.len(), 1);
        let (v, w) = h.pop().unwrap();
        if w==3 {
            let nv = g.al[&v].overweight as usize+1;
            part1 = (n0-nv)*nv;
            break;
        }

        g.merge(v, u);
    }
    let part2 = 0;

    Ok([part1.to_string(), part2.to_string()])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test01.txt")?;
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}", res[0], res[1]);
    assert_eq!(res[0], "54");
    assert_eq!(res[1], "0");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    let start = Instant::now();
    let (res, time) = loop { // run warmup for 100ms
        let lap = Instant::now();
        let res = solution(&input)?;
        if start.elapsed().as_millis()>100 {break (res, lap.elapsed())};
    };
    println!( "({} us)\nPart 1: {}\nPart 2: {}", time.as_micros(), res[0], res[1]);
    Ok(())
}
