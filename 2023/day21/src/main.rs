#![allow(unused_imports, dead_code)]

use anyhow::{anyhow, Context, Result};
use vecmath::vec2_add;
use std::{fs, time::Instant, collections::{BTreeSet, HashMap, HashSet}, cell::RefCell, rc::Rc};
use itertools::Itertools;

type V=[i32;2];

struct Board {
    pmax: V,
    p0: V,
    rocks: BTreeSet<V>,
}

fn parse_board(s: &str) -> Board {
    let mut pmax: V = [0;2];
    let mut p0: V = [0;2];
    let mut rocks = BTreeSet::<V>::new();
    for (y,ln) in s.trim_end().split("\n").enumerate() {
        for (x, c) in ln.chars().enumerate() {
            pmax = [x as i32, y as i32];
            match c {
                'S' => {p0=pmax;},
                '#' => {rocks.insert(pmax);},
                _ => {},
            }
        }
    };

    Board{pmax, p0, rocks}
}

const DS:[V;4] = [[0, -1], [-1, 0], [0, 1], [1,0]]; //nwse

fn step_once(board: &Board, p: V) -> Vec<V>{
    DS.iter()
    .map(|d| vec2_add(p, *d))
    .filter(|p2| !board.rocks.contains(&[
        p2[0].rem_euclid(board.pmax[0]+1), 
        p2[1].rem_euclid(board.pmax[1]+1)
    ]))
    .collect()
}

struct TwoStepper<'a> {
    board: &'a Board,
    cache: RefCell<HashMap<V, Rc<Vec<V>>>>,
}

impl <'a> TwoStepper<'a> {
    fn new(board: &'a Board) -> Self {Self{board, cache: RefCell::new(HashMap::new())}}

    fn step(&self, p: V) -> Rc<Vec<V>> {
        if let Some(res) = self.cache.borrow().get(&p) {
            return res.clone()
        }
        let s1 = step_once(self.board, p);
        let s2: Vec<V> = s1.iter().flat_map(|p1| step_once(self.board, *p1).into_iter()).unique().collect();
        self.cache.borrow_mut().insert(p, Rc::new(s2));
        self.cache.borrow().get(&p).unwrap().clone()
    }
}

fn part1(input_s: &str) -> Result<String> {
    let board = parse_board(input_s);
    let mut v = BTreeSet::<V>::from_iter([board.p0,]);
    for _ in 0..64 {
        let mut v2 = BTreeSet::new();
        for p2 in v.iter().flat_map(|p| DS.iter().map(|d| vec2_add(*p, *d))) {
            if p2.iter().any(|v| *v<0) || p2.iter().zip(board.pmax.iter()).any(|(v, vmax)| v>vmax) {continue};
            if !board.rocks.contains(&p2) {
                v2.insert(p2);
            };
        };
        v=v2;
    }
    Ok(v.len().to_string())
}   

fn part2_0(input_s: &str, n: usize) -> usize {
    let board = parse_board(input_s);
    let two_stepper = TwoStepper::new(&board);

    let mut s0: HashSet<V> = HashSet::new();
    let mut s1: HashSet<V> = if n%2==1 {
        step_once(&board, board.p0).into_iter().collect()
    } else {
        [board.p0].into_iter().collect()
    };
    let mut count: usize = s1.len();

    let n_half = n/2;

    for _i in 0..n_half {
        let mut s: HashSet<V> = HashSet::new();
        for p in &s1 {
            s.extend(two_stepper.step(*p).iter().cloned());
        }
        // ignore known points:
        let s = &(&s-&s1)-&s0;
        
        count += s.len();
        s0 = s1;
        s1 = s;
    }

    count
}

type Lookup = HashMap<V, HashSet<V>>;

/// All points reachable by twice the number of steps in the input lookup
/// lookups map from board field to reachable positions, which may be outside board
fn double_steps(board: &Board, lookup: &Lookup) -> Lookup {
    let wrap = {
        let mods = board.pmax.map(|v| v+1);
        move |q: V| [q[0].rem_euclid(mods[0]), q[1].rem_euclid(mods[1])]
    };
    let unwrap = {
        let mods = board.pmax.map(|v| v+1);
        move |q: V| [mods[0]*q[0].div_euclid(mods[0]), mods[1]*q[1].div_euclid(mods[1])]
    };
    
    lookup.iter().map(|(p0, dsts)| (
        *p0,
        dsts.iter()
        .flat_map(|p1| 
            lookup[&wrap(*p1)].iter().map(|p2w| vec2_add(*p2w, unwrap(*p1)))
        ).collect()
    )).collect()
}

fn make_one_step_lookup(board: &Board) -> Lookup {
    let wrap = {
        let mods = board.pmax.map(|v| v+1);
        move |q: V| [q[0].rem_euclid(mods[0]), q[1].rem_euclid(mods[1])]
    };

    (0..=board.pmax[0]).flat_map(move |x| (0..=board.pmax[1]).map(move |y| [x,y]))
        .filter(|p| !board.rocks.contains(p))
        .map(|p| (
            p,
            DS.iter().map(|d| vec2_add(p, *d))
            .filter(|pd| !board.rocks.contains(&wrap(*pd)))
            .collect()
        )).collect()
}

fn part2_1(input_s: &str, n: usize) -> Result<String> {
    let board = parse_board(input_s);

    // lookups for 1,2,4,... 2**i steps
    let mut lookups: Vec<Lookup> = vec![make_one_step_lookup(&board)]; 
    for i in 1..=13 {
        let last_lookup = lookups.last().unwrap();
        lookups.push(double_steps(&board, last_lookup));
        println!("Completed lookups up to 2**{} steps. Largest entry: {}", i, 
            lookups.last().unwrap().values().map(|v| v.len()).max().unwrap());
    }

    // let mut s0: HashSet<V> = HashSet::new();
    // let mut s1: HashSet<V> = if n%2==1 {
    //     step_once(&board, board.p0).into_iter().collect()
    // } else {
    //     [board.p0].into_iter().collect()
    // };
    let count: usize = 0;
    
    // let n_half = n/2;

    // for _i in 0..n_half {
    //     let mut s: HashSet<V> = HashSet::new();
    //     for p in &s1 {
    //         s.extend(two_stepper.step(*p).iter().cloned());
    //     }
    //     // ignore known points:
    //     let s = &(&s-&s1)-&s0;
        
    //     count += s.len();
    //     s0 = s1;
    //     s1 = s;
    // }

    Ok(count.to_string())
}


// input: 131 x 131, S at center. 
// empty centers, skew diagonals and edges.
// 26_501_365, //i32::MAX; // 2_147_483_647
#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    assert_eq!(part2_0(&input, 6), 16);
    assert_eq!(part2_0(&input, 100), 6536);
    //assert_eq!(part2_0(&input, 5000), 16733044);
    Ok(())
}

fn part2(s: &str, n: usize) -> Result<String> {
    assert_eq!(n, 131*202_300+65);
    let board = parse_board(s);
    assert_eq!(board.pmax, [130, 130]);

    let n0 = part2_0(s, 0*131+65);
    let n2 = part2_0(s, 2*131+65);
    let n4 = part2_0(s, 4*131+65);

    println!("n0,2,4 {:?}", [n0, n2, n4]);
    let b8 = 4*n2-n4-3*n0;
    println!("b8%8: {}", b8 % 8);

    // [n0, n2, n4] = [3944, 97230, 314556]
    // b = (4*n2-n4-3*n0)/8
    // a = n0-b
    // c = (n4-a-25*b)/16
    // [a+b*(k+1)**2+c*k**2 for k in [0,2,4, 202300]]

    Ok(0.to_string())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    let start = Instant::now();
    let (res, time) = loop { // run warmup for 100ms
        let lap = Instant::now();
        let res = [part1(&input)?, part2(&input, 26_501_365)?];
        if start.elapsed().as_millis()>100 {break (res, lap.elapsed())};
    };
    println!( "({} us)\nPart 1: {}\nPart 2: {}", time.as_micros(), res[0], res[1]);
    Ok(())
}
