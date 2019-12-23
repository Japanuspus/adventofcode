use std::collections::{HashMap, HashSet, VecDeque};

type Pt = (isize, isize);

fn flood_dist(m: &HashMap<Pt, u8>, o: &Pt) -> Vec<u8, isize> {
    let ds = Vec::new();
    let mut q = VecDeque::new();
    q.push_back((o, 0isize));
    while let Some((p, d)) = q.pop_front() {
        
    }
    ds
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Error reading input.txt");    
    // let mut points: HashMap<_,_> = HashMap::new();  
    let annotated_map: HashMap<_,_> = input
        .lines()
        .enumerate()
        .flat_map(|(y,l)| {
            l
            .chars()
            .map(|c| c as u8)
            .enumerate()
            .filter(|(_, c)| *c!=b'#')
            .map(move |(x, c)|
                ( (x as isize, y as isize), c)
            )
        })
        .collect();
    let pts: HashMap<_,_> = annotated_map
        .iter()
        .filter_map(|(p, c)| if *c == b'.' {None} else {Some((c, p))})
        .collect();
    let map: HashSet<_> = annotated_map //Does not include doors
        .iter()
        .filter_map(|(p, c)| if *c >= b'A' && *c <= b'Z' {None} else {Some(p)})
        .collect();

    dbg!(&pts);
}
