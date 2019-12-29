
use std::collections::{HashMap, HashSet, BinaryHeap,  BTreeMap, BTreeSet, VecDeque};
use std::cmp;
type Pos = (isize, isize);

/// Flood fill to get dists from o to all named positions together with 
/// key requirements (as downcase chars)
fn flood_dist(m: &HashMap<Pos, char>, o: &Pos) -> HashMap<char, (isize, BTreeSet<char>, Pos)> {
    let mut ds = HashMap::new();
    let mut q = VecDeque::new();
    let mut visited = BTreeSet::new();
    visited.insert(o.clone());
    q.push_back((o.clone(), 0isize, BTreeSet::new()));
    while let Some((p, dist, r)) = q.pop_front() {
        for d in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let p2 = (p.0+d.0, p.1+d.1);
            if visited.contains(&p2) {
                continue
            }
            if let Some(c2) = m.get(&p2) {
                let d2 = dist+1;
                let mut r2 = r.clone();
                if c2.is_ascii_lowercase() {
                    ds.insert(*c2, (d2, r.clone(), p2.clone()));
                } else {
                    if c2.is_ascii_uppercase() {
                        r2.insert(c2.to_ascii_lowercase());
                    }
                }
                visited.insert(p2.clone());
                // TODO: should be ok to not search further from lowercase
                q.push_back((p2, d2, r2));
            }
        }
    }
    ds
}

type KeyNode = (Pos, BTreeSet<char>);

/// Check that all routes to given point has same key requirements
/// This is required by the get_adj implementation below
fn check_map(m: &HashMap<Pos, char>, o: &Pos) -> bool {
    let mut q = VecDeque::new();
    let mut visited = BTreeMap::new();
    let kn = (o.clone(), BTreeSet::new());
    visited.insert(kn.0.clone(), kn.1.clone());
    q.push_back(kn);
    while let Some((p, r)) = q.pop_front() {
        for d in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let p2 = (p.0+d.0, p.1+d.1);
            if let Some(c2) = m.get(&p2) {
                let mut r2 = r.clone();
                if c2.is_ascii_uppercase() {
                        r2.insert(c2.to_ascii_lowercase());
                }
                if let Some(old_req) = visited.get(&p2) {
                    if old_req != &r2 {
                        println!("Mismatching requirements to {} at {:?}: {:?} vs {:?}", &c2, &p2, old_req, r2);
                        return false
                    }
                } else {
                    visited.insert(p2.clone(), r2.clone());
                    q.push_back((p2, r2));
                }
            }
        }
    };
    true
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

// Get adjancies, assuming that route from x to y will always be the shortest 
// such route using any keys required.
fn get_adj(map: &HashMap<Pos, char>, node: &KeyNode) -> Vec<(isize, KeyNode)> {
    let ds = flood_dist(map, &node.0);
    let keys = &node.1;
    ds
    .into_iter()
    .filter_map(|(c, (dist, req, p))|{
        if keys.is_subset(&req) {
            let mut next_keys = keys.clone();
            next_keys.insert(c);
            Some((dist, (p, next_keys)))
        } else {
            None
        }
    })
    .collect()
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

    // let fd = flood_dist(&annotated_map, &starts[0]);
    // println!("Reachable from start: {:?}", fd.keys().cloned().collect::<Vec<_>>());
    // let pta = fd.get(&'n').unwrap().2;
    // let fd = flood_dist(&annotated_map, &pta);
    // println!("Reachable from a: {:?}", fd.keys().cloned().collect::<Vec<_>>());
    // dbg!(&node_ids);
    assert!(check_map(&annotated_map, &starts[0]));
 
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
        let old_heap_size = open.len();
        for (dist, key_node) in get_adj(&annotated_map, &value) {
            let is_closed = closed.contains_key(&key_node);
            println!("{:6}: from {:?} -> {:?}", if is_closed {"CLOSED"} else {"OPEN"}, &value, &key_node);
            if !is_closed {
                open.push(HeapEntry{cost: cost+dist, value: key_node})
            }
        }
        let new_open = open.len() - old_heap_size;
        println!("After visiting {:?}: nodes added: {}", &value, new_open);
        //if new_open==0 {panic!()};
        closed.insert(value, cost);
    }

}
