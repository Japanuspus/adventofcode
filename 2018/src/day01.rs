use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        assert_eq!(part1_01("+1\n-2\n+3\n+1"), 3);
    }
    #[test]
    fn part2() {
        assert_eq!(part2_01("+1\n-2\n+3\n+1"), 2);
    }
}

pub fn part1_01(d: &str) -> i64 {
    d
    .lines()
    .map(|a| a.parse::<i64>().unwrap())
    .sum()
}

pub fn part2_01(d: &str) -> i64 {
//    let shifts = d
//        .lines()
//        .map(|a| a.parse::<i64>().unwrap())

    let mut fs = HashSet::new();
    let mut s: i64 = 0;

    let xx: i64 = d
        .lines()
        .map(|a| a.parse::<i64>().unwrap())
        .cycle()
        .map(|a| {s=s+a; s})
        .take_while(|s| fs.insert(*s))
        .last().unwrap();
//        .last()
//        .unwrap()
    0
}


pub fn run(data: &str) {
    println!("{}", part1_01(&data));
}