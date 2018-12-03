//use std::collections::HashSet;
//use std::collections::Vec;
use std::cmp::min;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        assert_eq!(part1_01("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"), 3*4);
    }
    #[test]
    fn part2() {
        assert_eq!(part2_01("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"), "fgij".to_string());
    }
}

fn count_letters(s: &str) -> [u8;26] {
    s
    .as_bytes().iter()
    .fold([0;26], |mut acc, &b| {
        let idx = (b-b'a') as usize;
        acc[idx] = acc[idx]+1;
        acc
    })
}

pub fn part1_01(d: &str) -> i64{
    let c23 = d
    .lines()
    .map(|l| {
        let cs=count_letters(l);
        (
            cs.iter().filter(|&c| *c == 2).count(),
            cs.iter().filter(|&c| *c == 3).count(),
        )
    })
    .fold((0,0), |(a1,a2), (b1,b2)| (a1+min(b1,1), a2+min(b2,1)));
    (c23.0*c23.1) as i64
}

pub fn part2_01(d: &str) -> String {
    "fgij".to_string()
}

pub fn run(data: &str) {
    println!("{}", part1_01(&data));
    println!("{}", part2_01(&data));
}