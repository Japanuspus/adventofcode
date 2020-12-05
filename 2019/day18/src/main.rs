use std::collections::{HashMap, BinaryHeap,  BTreeMap, BTreeSet, VecDeque};
use std::cmp;

type Pos = (isize, isize);
type AnnotatedMap = HashMap<Pos,char>;

fn parse_map(input: &str) -> AnnotatedMap
{   input
    .lines()
    .enumerate()
    .flat_map(|(y,l)| {
        l
        .chars()
        .enumerate()
        .filter(|(_, c)| *c!='#')
        .map(move |(x, c)| ( (x as isize, y as isize), c))
    })
    .collect()
}

type KeyNode = (Pos, BTreeSet<char>);

// Get adjancies, assuming that route from x to y will always be the shortest 
// such route using any keys required.
fn get_adj(map: &AnnotatedMap, node: &KeyNode) -> Vec<(isize, KeyNode)> {
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

type KeyNode4 = ([Pos; 4], BTreeSet<char>);

fn get_adj4(map: &AnnotatedMap, node: &KeyNode4) -> Vec<(isize, KeyNode4)> {
    let os = &node.0;
    let keys = &node.1;
    let mut ds4 = Vec::new();
    for (io, o) in os.iter().enumerate() {
        let kn0 = (o.clone(), keys.clone());
        for (d2, (p2, keys2)) in get_adj(map, &kn0) {
            let mut p2_4 = os.clone();
            p2_4[io] = p2;
            ds4.push((d2, (p2_4, keys2)));
        }
    };
    ds4
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
    T: cmp::Ord + std::fmt::Debug,
    U: Fn(&T) -> Vec<(isize, T)>,
    V: Fn(&T)->bool
{
    let mut open: BinaryHeap<HeapEntry<T>> = BinaryHeap::new();
    let mut closed: BTreeMap<T, isize> = BTreeMap::new();
    open.push(HeapEntry{cost: 0, value: v0});
    while let Some(HeapEntry{cost, value}) = open.pop() {
        if closed.contains_key(&value) { 
            //println!("Skipping closed node: {:?}", &value);
            continue //closed at lower cost
        } 
        //println!("Processing {:?}", &value);
        if is_complete(&value) { return Some(cost); }
        for (dist, key_node) in get_adj(&value) {
            let is_closed = closed.contains_key(&key_node);
            if !is_closed {
                //println!("  Registering descendant: {:?}", key_node);
                open.push(HeapEntry{cost: cost+dist, value: key_node})
            }
        }
        closed.insert(value, cost);
    };
    None
}


fn get_starts(map: &AnnotatedMap) -> Vec<Pos> {
    map.iter()
        .filter(|(_, &c)| c=='@').map(|(p, _)| p)
        .cloned().collect()
}


fn get_starts4(map: &AnnotatedMap) -> [Pos;4] {
    let starts_vec = get_starts(map);
    let mut starts: [Pos;4] = [(0,0);4];
    starts.copy_from_slice(&starts_vec[0..4]);
    starts
}

fn split_map(map: &AnnotatedMap) -> AnnotatedMap {
    let mut map2 = map.clone();
    let start = get_starts(map).iter().next().unwrap().clone();
    let new_center = "@#@\n###\n@#@";
    let (x0, y0) = (start.0-1, start.1-1);
    for (dy,ln) in new_center.lines().enumerate() {
        for (dx, c) in ln.chars().enumerate() {
            let p = (x0+dx as isize, y0+dy as isize);
            if c=='#' {
                map2.remove(&p);
            } else {
                map2.insert(p, c);
            }
        }
    };
    map2
}

#[test]
fn test_get_adj4() {
    let map0 = parse_map(&"#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
#####.@.#####
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############");

    let start = get_starts(&map0).into_iter().next().unwrap();
    assert_eq!(map0.get(&start), Some(&'@'));

    let map = split_map(&map0);
    assert_eq!(map.get(&start), None);

}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Error reading input.txt");    

    let map1 = parse_map(&input);

    let node_ids: BTreeSet<char> = map1.iter()
        .filter_map(|(_, c)| if c.is_ascii_lowercase() {Some(*c)} else {None})
        .collect();
    
    // Part 1
    let start: Pos = get_starts(&map1).into_iter().next().unwrap();
    let res = bfs(
        (start.clone(), BTreeSet::new()),
        |v| get_adj(&map1, v),
        |v| node_ids.is_subset(&v.1));
    println!("Part 1: {}", res.unwrap());

    // Part 2
    let map2 = split_map(&map1);
    let starts = get_starts4(&map2);
    let res = bfs(
        (starts, BTreeSet::new()),
        |v| get_adj4(&map2, v),
        |v| node_ids.is_subset(&v.1));
    println!("Part 2: {}", res.unwrap());
}
