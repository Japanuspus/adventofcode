use std::collections::BTreeMap;

#[cfg(test)]
mod tests {
    use super::*;
    const TT:& str = r#"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   "#;
    const TT2: & str = r#"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#;
    #[test]
    fn part1() {
        assert_eq!(part1_01(TT), (3, 7));
        assert_eq!(part2_01(TT2), (4, 6));
    }

    #[test]
    fn tt() {
        let c = Cart {heading: 0, turn: 0};
        assert_eq!(c.nextpos(&(7, 8)), (7, 9));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cart {
    heading: u8, //>v<^
    turn: u8, // left, straight, right  -- mod 3
}

impl Cart {
    pub fn nextpos(& self, ij: &(usize, usize)) -> (usize, usize) {
        match self.heading {
            0 => (ij.0, ij.1+1), //>
            1 => (ij.0+1, ij.1), //v
            2 => (ij.0, ij.1-1), //<
            3 => (ij.0-1, ij.1), //^
            _ => panic!()
        }
    }

    // new state for cart if it lands on c
    //  ESWN
    // | S N  
    // -E W   
    // \SENW  h <- -h + 1 mod 4
    // /NWSE  h <- -h + 3 mod 4
    // +      h <- h+(t-1) mod 4, t <- t+1 mod 3
    pub fn nextcart(&self, c: u8) -> Cart {
        match c {
            b'|' | b'-' => self.clone(),
            b'\\' => Cart {heading: ((-(self.heading as i8) + 1 + 4) % 4) as u8, turn: self.turn},
            b'/'  => Cart {heading: ((-(self.heading as i8) + 3 + 4) % 4) as u8, turn: self.turn},
            b'+'  => Cart {heading: (self.heading + self.turn + 3) % 4, turn: (self.turn + 1) % 3},  
            _ => panic!()
        }
    }
}

type Board = Vec<Vec<u8>>;
type Carts = BTreeMap<(usize, usize), Cart>;

pub fn parse_board(d: &str) -> (Board, Carts) {
    let mut carts: Carts = BTreeMap::new();
    let board: Board = d.lines().enumerate().map(|(i, r)| {
        r.as_bytes().iter().enumerate().map(|(j, c)| 
            match c {
                b'>' => {carts.insert((i, j), Cart {heading: 0,  turn: 0}); &b'-'},
                b'v' => {carts.insert((i, j), Cart {heading: 1,  turn: 0}); &b'|'},
                b'<' => {carts.insert((i, j), Cart {heading: 2,  turn: 0}); &b'-'},
                b'^' => {carts.insert((i, j), Cart {heading: 3,  turn: 0}); &b'|'},
                cc => cc,
            }
        ).cloned().collect()
    }).collect();
    (board, carts)
}

pub fn part1_01(d: &str) -> (usize, usize) {
    let (board, mut carts) = parse_board(d);

    'outer: loop {
        let cart_positions: Vec<(usize, usize)> = carts.keys().cloned().collect();
        for pos in cart_positions {
            let cart = carts.remove(&pos).unwrap();
            let p2 = cart.nextpos(&pos);
            if carts.contains_key(&p2) {
                break 'outer p2;
            }
            carts.insert(p2, cart.nextcart(board[p2.0][p2.1]));
        }
    }
}

pub fn part2_01(d: &str) -> (usize, usize) {
    let (board, mut carts) = parse_board(d);

    'outer: loop {
        let cart_positions: Vec<(usize, usize)> = carts.keys().cloned().collect();
        for pos in cart_positions {
            if let Some(cart) = carts.remove(&pos) {
                let p2 = cart.nextpos(&pos);
                if carts.contains_key(&p2) {
                    carts.remove(&p2);
                } else {
                    carts.insert(p2, cart.nextcart(board[p2.0][p2.1]));
                }
            }
        }
        if carts.len()<2 {
            break 'outer carts.keys().next().unwrap().clone()
        }
    }
}

pub fn run(data: &str) {
    let a = part1_01(&data);
    println!("Part 1: i,j: {:?} -- flip order: {},{}", a, a.1, a.0);
    let a = part2_01(&data);
    println!("Part 2: i,j: {:?} -- flip order: {},{}", a, a.1, a.0);
}