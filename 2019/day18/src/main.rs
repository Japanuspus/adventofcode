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
struct HeapEntry {
    cost: isize,
    value: KeyNode,
}

// Reverse ordering on cost to get max value out on top
impl cmp::Ord for HeapEntry {
    fn cmp(&self, other: &HeapEntry) -> cmp::Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.value.1.cmp(&other.value.1))
            .then_with(|| self.value.0.cmp(&other.value.0))
    }
}

impl cmp::PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &HeapEntry) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
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
    let starts: Vec<Pos> = annotated_map
        .iter()
        .filter(|(_, &c)| c=='@')
        .map(|(p, _)| p)
        .cloned()
        .collect();

    // Search for shortest paths in a graph with nodes of type KeyNode
    //let mut open: BTreeMap<isize, KeyNode> = BTreeMap::new();
    let mut open: BinaryHeap<HeapEntry> = BinaryHeap::new();
    let mut closed: BTreeMap<KeyNode, isize> = BTreeMap::new();
    open.push(HeapEntry{cost: 0, value: (starts[0].clone(), BTreeSet::new())});
    while let Some(HeapEntry{cost, value}) = open.pop() {
        if closed.contains_key(&value) { continue } //closed at lower cost
        if node_ids.is_subset(&value.1) {
            println!("Part 1: {}", cost);
            break
        }
        for (dist, key_node) in get_adj(&annotated_map, &value) {
            let is_closed = closed.contains_key(&key_node);
            //println!("{:6}: from {:?} -> {:?}", if is_closed {"CLOSED"} else {"OPEN"}, &value, &key_node);
            if !is_closed {
                open.push(HeapEntry{cost: cost+dist, value: key_node})
            }
        }
        closed.insert(value, cost);
    }

}
