#[cfg(test)]
mod tests {
    use super::*;
    const TT:& str = "dabAcCaCBAcCcaDA";
    #[test]
    fn part1() {
        assert_eq!(part1_01(TT),10);
        assert_eq!(part1_02(TT),10);
        assert_eq!(part2_01(TT),4);
    }
}

pub fn opposite(a: u8, b: u8) -> bool {
    a^b==32
}

#[test]
fn test_opo() {
    assert_eq!('a' as u8 ^ 'A' as u8, 32);
    assert!(opposite('a' as u8,'A' as u8));
}

pub fn part1_01(d: &str) -> i64{
    let mut stack: Vec<u8> = Vec::new();
    //let mut top: Option<u8>;
    for c in d.trim().as_bytes().iter() {
        let top = stack.pop();
        match top {
            Some(sc) if opposite(*c, sc) => {},
            Some(sc) => {stack.push(sc); stack.push(*c);},        
            None => {stack.push(*c);},
        }
        /*
        // This fails because stack is both imutably and mutably borrowed
        // should work when new scoping is activated
        match stack.last() {
            Some(sc) if opposite(*c, *sc) => {stack.pop();},
            _ => {stack.push(*c);},
        }
        */
    };
    stack.len() as i64
}

fn collapse_polymer<'_a, I>(vals: I) -> usize
where I: IntoIterator<Item = &'_a u8> {
    let mut stack: Vec<u8> = Vec::new();
    for c in vals.into_iter() {
        let top = stack.pop();
        match top {
            Some(sc) if opposite(*c, sc) => {},
            Some(sc) => {stack.push(sc); stack.push(*c);},        
            None => {stack.push(*c);},
        }
    };
    stack.len()
}

pub fn part1_02(d: &str) -> i64{
    collapse_polymer(d.trim().as_bytes()) as i64
}

fn collapse_filtered<'_a, I>(vals: I, c: u8) -> usize 
where I: IntoIterator<Item = &'_a u8> {
    collapse_polymer(vals.into_iter().filter(|&cc| !(c==*cc || opposite(c, *cc))))
}

pub fn part2_01(d: &str) -> i64 {
    ('a' as u8 ..'z' as u8)
    .map(|c| collapse_filtered(d.trim().as_bytes(), c))
    .min().unwrap() as i64
}

pub fn run(data: &str) {
    println!("Part 1: {}", part1_01(&data));
    println!("Part 2: {}", part2_01(&data));
}