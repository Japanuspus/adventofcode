//use std::collections::HashSet;
use std::collections::BTreeSet;
//use std::collections::Vec;
use std::cmp::min;

#[cfg(test)]
mod tests {
    const TT: &str = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
    const TT2: &str = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";

    use super::*;
    #[test]
    fn part1() {
        assert_eq!(part1_01(TT), 3 * 4);
    }
    #[test]
    fn part2() {
        assert_eq!(part2_01(TT2), "fgij".to_string());
    }
    #[test]
    fn part2_helper() {
        assert_eq!(match_at_k(TT2, 0), None);
        assert_eq!(match_at_k(TT2, 2), Some("fgij".to_string()));
    }
}

fn count_letters(s: &str) -> [u8; 26] {
    s.as_bytes().iter().fold([0; 26], |mut acc, &b| {
        let idx = (b - b'a') as usize;
        acc[idx] = acc[idx] + 1;
        acc
    })
}

pub fn part1_01(d: &str) -> i64 {
    let c23 = d
        .lines()
        .map(|l| {
            let cs = count_letters(l);
            (
                cs.iter().filter(|&c| *c == 2).count(),
                cs.iter().filter(|&c| *c == 3).count(),
            )
        })
        .fold((0, 0), |(a1, a2), (b1, b2)| {
            (a1 + min(b1, 1), a2 + min(b2, 1))
        });
    (c23.0 * c23.1) as i64
}

fn match_at_k(d: &str, k: usize) -> Option<String> {
    let mut bt = BTreeSet::new();
    d.lines()
        .map(|a| String::from(&a[0..k]) + &a[k + 1..])
        .find(|s: &String| !bt.insert(s.clone()))
}

pub fn part2_01(d: &str) -> String {
    (0..).find_map(|k| match_at_k(d, k)).unwrap()
}

pub fn run(data: &str) {
    println!("{}", part1_01(&data));
    println!("{}", part2_01(&data));
}
