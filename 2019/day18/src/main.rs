use std::collections::{HashMap, HashSet, BinaryHeap,  BTreeMap, BTreeSet, VecDeque};
use std::cmp;
type Pos = (isize, isize);

type KeyNode = (Pos, BTreeSet<char>);


// Get adjancies, assuming that route from x to y will always be the shortest 
// such route using any keys required.
fn get_adj(map: &HashMap<Pos, char>, node: &KeyNode) -> Vec<(isize, KeyNode)> {
    let keys = &node.1;
    let o = &node.0;

    let mut ds = Vec::new();
    let mut q = VecDeque::new();
    let mut visited = BTreeSet::new();
    visited.insert(o.clone());
    q.push_back((o.clone(), 0isize));
    while let Some((p, dist)) = q.pop_front() {
        for d in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let p2 = (p.0+d.0, p.1+d.1);
            if visited.contains(&p2) {
                continue
            }
            if let Some(c2) = map.get(&p2) {
                if c2.is_ascii_uppercase() && !keys.contains(&c2.to_ascii_lowercase()) {
                    continue;
                }
                let d2 = dist+1;
                if c2.is_ascii_lowercase() {
                    let mut k2 = keys.clone();
                    k2.insert(*c2);
                    ds.push((d2, (p2.clone(), k2)));
                } else {
                    q.push_back((p2.clone(), d2));
                }
                visited.insert(p2);
            }
        }
    }
    ds
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct HeapEntry<T> {
    cost: isize,
    value: T,
}

// Reverse ordering on cost to get max value out on top
impl<T> cmp::Ord for HeapEntry<T>
where
    T: cmp::Ord
{
    fn cmp(&self, other: &HeapEntry<T>) -> cmp::Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.value.cmp(&other.value))
    }
}

impl<T> cmp::PartialOrd for HeapEntry<T>
where
T: cmp::Ord
{
    fn partial_cmp(&self, other: &HeapEntry<T>) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Breath first search for shortest paths to a node where is_complete returns true
fn bfs<T, U, V>(v0: T, get_adj: U, is_complete: V) -> Option<isize>
where
    T: cmp::Ord,
    U: Fn(&T) -> Vec<(isize, T)>,
    V: Fn(&T)->bool
{
    let mut open: BinaryHeap<HeapEntry<T>> = BinaryHeap::new();
    let mut closed: BTreeMap<T, isize> = BTreeMap::new();
    open.push(HeapEntry{cost: 0, value: v0});
    while let Some(HeapEntry{cost, value}) = open.pop() {
        if closed.contains_key(&value) { continue } //closed at lower cost
        if is_complete(&value) { return Some(cost); }
        for (dist, key_node) in get_adj(&value) {
            let is_closed = closed.contains_key(&key_node);
            if !is_closed {
                open.push(HeapEntry{cost: cost+dist, value: key_node})
            }
        }
        closed.insert(value, cost);
    };
    None
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Error reading input.txt");    

    let annotated_map: HashMap<_,_> = input
        .lines()
        .enumerate()
        .flat_map(|(y,l)| {
            l
            .chars()
            .enumerate()
            .filter(|(_, c)| *c!='#')
            .map(move |(x, c)| ( (x as isize, y as isize), c))
        })
        .collect();

    let nodes: HashMap<char, Pos> = annotated_map
        .iter()
        .filter_map(|(p, c)| if c.is_ascii_lowercase() {Some((*c, p.clone()))} else {None})
        .collect();
    let node_ids:BTreeSet<char> = nodes.keys().cloned().collect();
    
    // Part 1
    let start: Pos = annotated_map
        .iter()
        .filter(|(_, &c)| c=='@')
        .map(|(p, _)| p)
        .next().unwrap().clone();

    let res = bfs(
        (start.clone(), BTreeSet::new()),
        |v| get_adj(&annotated_map, v),
        |v| node_ids.is_subset(&v.1));
    println!("Part 1: {}", res.unwrap());
}
