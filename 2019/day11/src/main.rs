#![allow(unused)]

use std::collections::HashMap;
use std::fmt;
use day11::State;

fn run_robot(input: &str, initial: Option<isize>) -> HashMap<(isize, isize), isize> {
    let mut s = State::from(input);
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut h: isize = 0; // 0^, 1>, 2v, 3< mod 4
    let mut board = HashMap::new();

    if let Some(v) = initial {
        board.insert((0,0), v);
    }

    loop {
        let color_at_xy: isize = *board.get(&(x, y)).unwrap_or(&0);
        // let mut ii = inputs.iter().cloned();
        if let Some(res) = s.next_numbers(2, || Some(color_at_xy)).unwrap() {
            let color = res[0];
            let turn = res[1];

            board.insert((x,y), res[0]);
            h = (h + if res[1]==0 {-1} else {1}).rem_euclid(4);
            x += match h { 1 => 1, 3=> -1, _ => 0};
            y += match h { 0 => -1, 2=> 1, _ => 0};
        } else {
            break
        }
    }

    board
} 


#[derive(Debug)]
struct Canvas {
    ytop: isize,
    xleft: isize,
    ncol: isize,
    nrow: isize,
    symbols: Vec<char>,
    values: Vec<isize> //size w x h
}

impl Canvas {
    fn new(lt: &(isize, isize), rb: &(isize, isize), symbols: &str) -> Canvas {
        let nrow = (1+ rb.1 - lt.1);
        let ncol = (1+ rb.0 - lt.0);
        let symbols: Vec<char> = symbols.chars().collect();
        Canvas {
            ytop: lt.1, xleft: lt.0, nrow, ncol, 
            symbols, 
            values: std::iter::repeat(0).take( (nrow*ncol) as usize).collect()
        }
    }

    fn for_points<'a>(mut w: impl Iterator<Item=&'a (isize, isize)>, symbols: &str) -> Self {
        let (x,y) = w.next().unwrap();
        let mut x0 = *x;
        let mut x1 = *x;
        let mut y0 = *y;
        let mut y1 = *y;
        loop {
            if let Some((x,y)) = w.next() {
                if *x<x0 {x0=*x};
                if *x>x1 {x1=*x};
                if *y<y0 {y0=*y};
                if *y>y1 {y1=*y};
            } else {
                break;
            }
        }
        Canvas::new(&(x0, y0), &(x1, y1), symbols)
    }
    
    fn index(& self, x: isize, y: isize) -> usize {
        let dy = y-self.ytop;
        let dx = x-self.xleft;
        (dy*self.ncol+dx) as usize
    }

    fn set(&mut self, x: isize, y: isize, v: isize) {
        let idx = self.index(x,y);
        self.values[idx]=v;
    }
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.values.chunks(self.ncol as usize) {
            writeln!(f,"{}", row
                .iter()
                .map(|v| self.symbols.get(*v as usize).unwrap_or(&'?'))
                .collect::<String>())?;
        }
        Ok(())
    }
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
    .expect("Error reading input file");

    let board = run_robot(&input, None);
    println!("Part 1: {}", board.len());
    
    let board = run_robot(&input, Some(1));
    let mut canvas = Canvas::for_points(board.keys(), &" #");
    for (k, v) in board.iter() { canvas.set(k.0, k.1, *v)};
    println!("Part 2:\n{}", &canvas);
}