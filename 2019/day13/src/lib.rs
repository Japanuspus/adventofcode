use std::fmt;

#[derive(Debug)]
pub struct Canvas {
    ytop: isize,
    xleft: isize,
    ncol: isize,
    nrow: isize,
    symbols: Vec<char>,
    values: Vec<isize> //size w x h
}

impl Canvas {
    pub fn new(lt: &(isize, isize), rb: &(isize, isize), symbols: &str) -> Canvas {
        let nrow = 1+ rb.1 - lt.1;
        let ncol = 1+ rb.0 - lt.0;
        let symbols: Vec<char> = symbols.chars().collect();
        Canvas {
            ytop: lt.1, xleft: lt.0, nrow, ncol, 
            symbols, 
            values: std::iter::repeat(0).take( (nrow*ncol) as usize).collect()
        }
    }

    pub fn for_points(mut w: impl Iterator<Item=(isize, isize)>, symbols: &str) -> Self {
        let (x,y) = w.next().unwrap();
        let mut x0 = x;
        let mut x1 = x;
        let mut y0 = y;
        let mut y1 = y;
        loop {
            if let Some((x,y)) = w.next() {
                if x<x0 {x0=x};
                if x>x1 {x1=x};
                if y<y0 {y0=y};
                if y>y1 {y1=y};
            } else {
                break;
            }
        }
        Canvas::new(&(x0, y0), &(x1, y1), symbols)
    }
    
    fn index(& self, x: isize, y: isize) -> usize {
        let dy = y-self.ytop;
        let dx = x-self.xleft;
        assert!(dy>=0 && dy<self.nrow && dx>=0 && dx<=self.ncol);
        (dy*self.ncol+dx) as usize
    }

    pub fn set(&mut self, x: isize, y: isize, v: isize) {
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
