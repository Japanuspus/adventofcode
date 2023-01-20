#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use itertools::Itertools;
use nom::{self, error::ErrorKind, IResult};
use vecmath::{vec2_add, mat3_id, col_mat3_mul, col_mat3_transform, mat3_transposed};
use std::{fs, time::Instant, collections::{HashMap, HashSet, VecDeque, hash_map}, fmt};

// Part 1

type Pos = [i16;2];
type Tile = [Pos;4]; //>v<^
const DIRECTIONS_P: [Pos;4] = [[1, 0], [0, 1], [-1, 0], [0, -1]]; //>v<^

fn parse_path(s: &str) -> IResult<&str, Vec<(i32, char)>> {
    nom::multi::many1(
        nom::sequence::pair(
            nom::character::complete::i32,
            nom::character::complete::one_of("LRE")
        )
    )(&s)
}

fn parse<T>(input_s: &str, f: impl FnOnce(&str) -> T) -> (T, Vec<(i32, char)>) {
    let mut input= input_s.trim_end().split("\n\n");
    let board = f(input.next().unwrap());
    let path = parse_path(&format!("{}E", input.next().unwrap())).unwrap().1;
    (board, path)
}

fn parse_board_1(s: &str) -> (Pos, HashMap<Pos, Tile>) {
    // min and max values in each column
    let mut col_min: HashMap<i16, i16> = HashMap::new();
    let mut col_max: HashMap<i16, i16> = HashMap::new();
    for (ym, ln) in s.split('\n').enumerate() {
        let y = ym as i16 + 1;
        for (xm, _) in ln.as_bytes().iter().enumerate().filter(|(_, &c)| c!=b' ') {
            let x = xm as i16 + 1;
            col_min.entry(x).or_insert_with(|| y);
            col_max.insert(x, y);
        }
    };

    let mut m = HashMap::new();
    for (ym, ln) in s.split('\n').enumerate() {
        // inner cells
        let y = ym as i16 + 1;
        let x_max = ln.as_bytes().len() as i16;
        let mut x_left = x_max;
        let mut x_min: Option<i16> = None;
        for (xm, &c) in ln.as_bytes().iter().enumerate().filter(|(_, &c)| c!=b' ') {
            let x = xm as i16 +1;
            if x_min.is_none() {x_min = Some(x);}
            if c==b'.' {
                let x_right = if x==x_max {x_min.unwrap()} else {x+1};
                let (y_min, y_max) = (col_min[&x], col_max[&x]);
                let y_up = if y==y_min {y_max} else {y-1};
                let y_down = if y==y_max {y_min} else {y+1};

                m.insert([x, y], [//>v<^
                    [x_right, y],
                    [x, y_down],
                    [x_left, y],
                    [x, y_up],
                ]);
            }
            x_left = x;
        }
    };
    // start pos
    let (xm,_) = s.as_bytes().iter().enumerate().find(|(_, &c)| c!=b' ').unwrap();
    ([xm as i16 +1, 1], m)
}

fn next_facing(facing: u8, rot: char) -> u8 {
    (facing + match rot {'R' => 1, 'L' => 3, 'E' => 0,  _ => panic!()}) % 4
}

fn solution_1(input_s: &str) -> Result<String> {
    // Facing is 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^)
    let ((p0, board), path) = parse(input_s, parse_board_1);

    let mut p = p0;
    let mut t = board[&p];
    let mut facing: u8 = 0;
    for (step, rot) in &path {
        for _ in 0..*step {
            let p2 = t[facing as usize];
            if let Some(t2) = board.get(&p2) {
                p = p2;
                t = *t2;    
            } else {
                break
            };
            //println!("{:?}", p);
        }
        facing = next_facing(facing, *rot);
        // println!("Turn to {}", facing);
    }
    // The final password is the sum of 1000 times the row, 4 times the column, and the facing
    let part1 = 1000*(p[1] as usize)+4*(p[0] as usize)+ (facing as usize);

    Ok(part1.to_string())
}

// Part 2

type V3 = [i16;3];
type M3 = [V3;3]; // (column-major), x, y, z (plane normal)
const EYE: M3 = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

/// Rotating over the edge in direction ax+by
/// See pdf notes for background
fn compute_ardat(a: i16, b: i16) ->  M3 {
    [[b*b, -a*b, -a], [-a*b, a*a, -b], [a, b, 0]]
}

/// Compute map from face normal to region and basis s.t. basis matches connections on flat map
fn parse_board_regions(s: &str, n: usize) -> (Pos, HashMap<V3, (Pos, M3)>) {
    // Region map: [col, row] (chunked)
    let regions: HashSet<Pos> = s.split('\n').step_by(n).enumerate()
    .flat_map(|(i, r)| 
        r.as_bytes().iter().step_by(n).enumerate()
        .filter_map(move |(j, v)| if *v==b' ' {None} else {Some([j as i16, i as i16])})
    ).collect();

    // Initial region
    let r0 = [(0..(regions.len() as i16)).find(|j| regions.contains(&[*j, 0])).unwrap(), 0];

    let mut face2base: HashMap<V3, (Pos, M3)> = HashMap::new(); 
    let mut work: VecDeque<(Pos, M3)> = VecDeque::new();
    work.push_back((r0, EYE));
    while let Some((r, base)) = work.pop_front() {
        match face2base.entry(base[2]) {
            hash_map::Entry::Occupied(entry) => {
                // This must be where we came from, so basis should match
                // Doesn't hurt to check for consistency...  
                assert_eq!(entry.get(), &(r, base));
            },
            hash_map::Entry::Vacant(entry) => {
                entry.insert((r, base));
                work.extend(
                    DIRECTIONS_P.iter()
                    .map(|dp| (dp, vec2_add(r, *dp)))
                    .filter(|(_, r2)| regions.contains(r2))
                    .map(|(dp, r2)| (r2, col_mat3_mul(base, compute_ardat(dp[0], -dp[1]))))   
                );
            },
        }
    }
    (r0, face2base)
}

#[test]
fn test_parse_regions() {
    let input = &fs::read_to_string("test00.txt").unwrap();
    let ((_, face2base), _) = parse(input, |s| parse_board_regions(s, 4));
    assert_eq!(face2base.len(), 6);
    assert!(face2base.contains_key(&[0,0,1]));
}


fn parse_board_dots(s: &str) -> HashSet<Pos> {
    s
    .split('\n').enumerate()
    .flat_map(|(y, ln)| 
        ln.as_bytes().iter().enumerate()
        .filter(|(_, &c)| c==b'.').map(move |(x, _)| [x as i16, y as i16]))
    .collect()
}

struct Board {
    //face2base: HashMap<V3, (Pos, M3)>,
    region0: Pos,
    /// Given region and direction index, what is next region, direction index and what is the B2.T B1 map
    edge_transitions: HashMap<Pos, [(Pos, u8, M3);4]>,
    dots: HashSet<Pos>,
    nm: i16,
}

const DIRECTIONS: [Pos;4] = [[1, 0], [0, -1], [-1, 0], [0, 1]]; //>v<^

/// Helper to compute Board::next_region
/// Used by `parse_board`. See pdf notes.
fn next_region(face2base: &HashMap<V3, (Pos, M3)>, base: &M3, d_index: u8) -> (Pos, u8, M3) {
    let dp = DIRECTIONS[d_index as usize];
    let (r2, base2) = face2base[&col_mat3_transform(*base, [dp[0], dp[1], 0])];
    let b2tb1 = col_mat3_mul(mat3_transposed(base2), *base);
    let neg_d2 = b2tb1[2];
    let d2xy = [-neg_d2[0], -neg_d2[1]];
    let d2_index = DIRECTIONS.iter().enumerate().find(|(_, dxy)| **dxy==d2xy).unwrap().0;
    (r2, d2_index as u8, b2tb1)
}

fn parse_board(input_s: &str, n: usize) -> Board {
    let (region0, face2base) = parse_board_regions(input_s, n);
    let dots = parse_board_dots(input_s);
    let nm = n as i16 -1;

    // for each region and each direction, compute what region comes next
    let edge_transitions: HashMap<Pos, [(Pos, u8, M3); 4]> =face2base.iter()
    .map(|(_face, (r, base))| (*r, [0,1,2,3].map(|id| next_region(&face2base, base, id))))
    .collect();

    Board{region0, edge_transitions, dots, nm}
}

#[derive(Clone, Debug)]
struct State {
    region: Pos,
    xy: Pos,
    index_d: u8,
}

impl Board {
    fn first(&self) -> State {
        State{region: self.region0, xy: [-self.nm, self.nm], index_d: 0}
    }

    fn state_to_map(&self, state: &State) -> Pos {
        [
            state.region[0] * (self.nm+1) + (self.nm+state.xy[0])/2,
            state.region[1] * (self.nm+1) + (self.nm-state.xy[1])/2,
        ]
    }

    fn contains(&self, state: &State) -> bool {
        self.dots.contains(&self.state_to_map(state))
    }

    fn next(&self, state: &State) -> State {
        let mut xy: Pos = [0;2];
        for (i, v) in state.xy.iter().zip(DIRECTIONS[state.index_d as usize].iter()).map(|(u, d)| u+2*d).enumerate() {
            xy[i] = v;
        };
        if xy.iter().all(|u| -self.nm <= *u && *u <= self.nm) {
            State{xy, ..*state}
        } else {
            let (region, index_d, b2tb1) = self.edge_transitions[&state.region][state.index_d as usize];
            let xyz = col_mat3_transform(b2tb1, [state.xy[0], state.xy[1], self.nm]);
            State{region, index_d, xy: [xyz[0], xyz[1]]}
        }
    }

    fn turn(&self, state: &State, rot: char) -> State {
        let index_d = next_facing(state.index_d, rot);
        State{index_d, ..*state}
    }
}

#[test]
fn test_board() {
    let input_s = &fs::read_to_string("test00.txt").unwrap();
    let (board, _path) = parse(input_s, |s| parse_board(s, 4));

    let mut s0 = board.first();
    assert_eq!(s0.region, [2, 0]);
    assert_eq!(s0.xy, [-3, 3]);
    s0.index_d = 2;
    let s1 = board.next(&s0);
    assert_eq!(s1.region, [1, 1]);
    assert_eq!(s1.xy, [-3, 3]);
    assert_eq!(s1.index_d, 1);
}

fn solution_2(input_s: &str, n: usize) -> Result<String> {
    let (board, path) = parse(input_s, |s| parse_board(s, n));
    let last_state = path.iter().fold(board.first(), |state, (steps, rot)| {
        let mut dst = state;
        for _ in 0..*steps {
            let candidate = board.next(&dst);
            if board.contains(&candidate) {dst=candidate} else {break;};
        };
        board.turn(&dst, *rot)
    });
    let p = board.state_to_map(&last_state);
    let part2 = 1000*(p[1] as usize + 1)+4*(p[0] as usize + 1)+ (last_state.index_d as usize);
    Ok(part2.to_string())
}

fn solution(input_s: &str, n: usize) -> Result<[String;2]> {
    let p1 = solution_1(input_s)?;
    let p2 = solution_2(input_s, n)?;
    Ok([p1, p2])
}

#[test]
fn test_solution() -> Result<()> {
    let input = &fs::read_to_string("test00.txt")?;
    let res = solution(&input, 4)?;
    assert_eq!(res[0], "6032");
    assert_eq!(res[1], "5031");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    for _ in 0..20 {solution(&input, 50)?;} //warmup
    let start = Instant::now();
    let res = solution(&input, 50)?;
    println!(
        "({} us)\nPart 1: {}\nPart 2: {}",
        start.elapsed().as_micros(), res[0], res[1],
    );
    Ok(())
}
