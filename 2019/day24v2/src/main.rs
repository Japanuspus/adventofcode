#![allow(unused)]

use std::iter;
use std::iter::once;
use std::collections::BTreeMap;

type Pos = (isize, isize);
type ConMasks = [[u32; 3]; 24];

/// Build edge lists as bitmap masks for level and neighboring levels
/// con_masks[i]: outer (z-1), current (z), inner (z+1)
struct BoardData {
    cell_coords: Vec<Pos>,
    cell_index_lookup: BTreeMap<Pos, usize>,
    con_masks: ConMasks
}

impl BoardData {
    fn new() -> Self {
        let cell_coords: Vec<Pos> = 
            (-2..=2).flat_map(move |y| (-2..=2).map(move |x| (x,y)))
            .filter(|(x,y)| !(*x==0 && *y==0))
            .collect();
        assert_eq!(cell_coords.len(), 24);
        let cell_index_lookup: BTreeMap<Pos, usize> = cell_coords.iter()
            .enumerate()
            .map(|(i, p)| (p.clone(), i))
            .collect();
        let mut con_masks: ConMasks = [[0; 3]; 24];
        let dirs = [(0isize, 1isize), (1, 0), (0, -1), (-1, 0)];
        for (i, p) in cell_coords.iter().enumerate() {
            con_masks[i][1] = dirs.iter()
                .filter_map(|pp| cell_index_lookup.get(&(pp.0+p.0, pp.1+p.1)).map(|idx| 1u32<<idx))
                .sum();
        }
        // connections from each outer cell to outside layer
        for k in 0..4 {
            let d = &dirs[k];
            let d_orto = &dirs[(k+1)%4];
            let mask_out = 1u32<<cell_index_lookup.get(d).unwrap();
            for xy in -2..=2 {
                let cell = (2*d.0+xy*d_orto.0, 2*d.1+xy*d_orto.1);
                let idx = cell_index_lookup.get(&cell).unwrap();
                con_masks[*idx][0] |= mask_out;
            }
        }
        // connections from inner cells to each outside cell in inner layer
        for k in 0..4 {
            let d = &dirs[k];
            let d_orto = &dirs[(k+1)%4];
            let mask_inner: u32 =(-2..=2)
                .map(|xy| {
                    let cell = (2*d.0+xy*d_orto.0, 2*d.1+xy*d_orto.1);
                    1u32<<cell_index_lookup.get(&cell).unwrap()
                })
                .sum();
            let idx = cell_index_lookup.get(d).unwrap();
            con_masks[*idx][2] |= mask_inner;
        }
        
        BoardData{cell_coords, cell_index_lookup, con_masks}
    }
    
    fn level_from_str(&self, s: &str) -> u32 {
        s
        .lines().enumerate()
        .flat_map(|(y, ln)|
            ln
            .chars().enumerate()
            .filter(|(_, c)| *c=='#')
            .map(move |(x, _)| (x as isize -2, y as isize-2)))
        .map(|p| 1u32<<self.cell_index_lookup.get(&p).unwrap())
        .sum()
    }

    fn print_level(&self, v: u32) {
        for y in -2..=2 {
            for x in -2..=2 {
                print!("{}", 
                    self.cell_index_lookup
                    .get(&(x,y))
                    .map(|idx| if (v>>idx)&1 == 0 {'.'} else {'#'})
                    .unwrap_or('?'));
            }
            println!("");
        }
    }

    fn dump(&self) {
        for (p, cms) in self.cell_coords.iter().zip(self.con_masks.iter()) {
            let idx = self.cell_index_lookup.get(p).unwrap();
            println!("****** Index: {}, Position: \n", idx);
            self.print_level(1u32<<idx);
            println!("Connections, outer/current/inner layers:\n");
            for cm in cms {self.print_level(*cm); println!("");}
        }
    }
}


/// New pattern for level as u32 bitfield
/// cell_and_nb must be a [u32;3] slice of level bitfields
/// outer, current, inner
fn life_step_window(bd: &BoardData, cell_and_nb: &[u32]) -> u32 {
    // Board state at level current
    let mut cur_states = iter::successors(
        Some(cell_and_nb[1]), 
        |bv| Some(bv>>1))
    .map(|bv| (bv&1)!=0);
    let mut nb_counts = bd.con_masks.iter()
    .map(|cell_mask|
        cell_mask.iter().zip(cell_and_nb.iter())
        .map(|(m, v)| (m&v).count_ones())
        .sum());
   
    let next_bits_left_justified = 
    nb_counts.zip(cur_states)
    //.inspect(|cc| {print!("nb count{}, state: {}", cc.0, cc.1);})
    .map(|(nb_count, cur_state): (u32, bool)| 
        if(cur_state) {
            nb_count == 1
        } else {
            nb_count == 1 || nb_count == 2
        })
    //.inspect(|cc| {println!("Res: {}", cc);})
    .fold(0u32, |obit, v| if v {(obit>>1)|0x80000000} else {obit>>1});
    next_bits_left_justified>>(32-24)
}

#[test]
fn test_life_step_window() {
    let board = BoardData::new();
    let a0 = board.level_from_str(".....\n.#...\n.#...\n.....\n......\n");
    let b0 = life_step_window(&board, &[0, a0, 0]);
    println!("Befor"); board.print_level(a0);
    println!("After"); board.print_level(b0);
    assert_eq!(a0, b0);

    let a0 = board.level_from_str("..#..\n.#...\n.....\n.....\n......\n");
    let c0 = board.level_from_str(".#...\n..#..\n.....\n.....\n......\n");
    let b0 = life_step_window(&board, &[0, a0, 0]);
    println!("Befor"); board.print_level(a0);
    println!("After"); board.print_level(b0);
    println!("Expect"); board.print_level(c0);
    assert_eq!(b0, c0);
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");

    let board = BoardData::new();
    //board.dump(); return;
    let b0 = board.level_from_str(&input);
    
    println!("Initial level 0");
    board.print_level(b0);

    let n_rep = 200;
    let n_buf = 500;
    let mut vs: Vec<u32> = iter::repeat(0).take(n_buf).collect();
    vs[n_buf/2] = b0;

    for i in 0..n_rep {
        let v_next: Vec<_> = once(0)
            .chain(
                vs.windows(3)
                .map(|w| life_step_window(&board, w))
            )
            .chain(once(0))
            .collect();
        vs = v_next;
    }

    println!("Part 2: {}", vs.iter().map(|v| v.count_ones()).sum::<u32>());
}
