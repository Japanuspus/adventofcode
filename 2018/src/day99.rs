#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let refdat = [("1122", 3), ("1111", 4), ("1234", 0), ("91212129", 9)];
        for tt in refdat.iter() {
            assert_eq!(part01_01(tt.0), tt.1);
            assert_eq!(part01_02(tt.0), tt.1);
            assert_eq!(part01_03(tt.0), tt.1);
        }
    }
}

pub fn part01_01(d: &str) -> i64 {
    let mut c0: char = d.chars().last().unwrap();
    let mut s: i64 = 0;
    for c in d.chars() {
        if c == c0 {
            s += c.to_digit(10).unwrap() as i64;
        }
        c0 = c;
    }
    s
}

pub fn part01_02(d: &str) -> i64 {
    d.chars()
        .zip(d.chars().cycle().skip(1))
        .map(|(a, b)| {
            if a == b {
                a.to_digit(10).unwrap() as i64
            } else {
                0
            }
        })
        .sum()
}

pub fn part01_03(d: &str) -> i64 {
    let digits = d.chars().map(|a| a.to_digit(10).unwrap() as i64);
    digits
        .clone()
        .zip(digits.cycle().skip(1))
        .filter(|(a, b)| a == b)
        .map(|(a, _b)| a)
        .sum()
    //digits.clone().zip(digits.cycle().skip(1)).filter_map(|(a,b)| if a==b {Some(a)} else {None}).sum()
}

// hmm, this seems a bit heavy. Let's just print for now
//use utils::Response;
//use std::fmt::Display;
//pub fn responses(data: &str, resp: &mut Vec<Box<Display>>) {
//    resp.push(Box::new(Response{ tag: "foo".to_string(), response: 7}));
//}

pub fn run(data: &str) {
    //let data = include_str!("../inputs/day99.txt").trim_right();
    println!("{}", part01_01(&data));
}
