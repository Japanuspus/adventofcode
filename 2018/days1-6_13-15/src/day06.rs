#[cfg(test)]
mod tests {
    use super::*;
    const TT:& str = "";
    #[test]
    fn part1() {
        assert_eq!(part1_01(TT),0);
    }
}

pub fn part1_01(_d: &str) -> i64{
    0
}

pub fn part2_01(_d: &str) -> i64 {
    0
}

pub fn run(data: &str) {
    println!("Part 1: {}", part1_01(&data));
    println!("Part 2: {}", part2_01(&data));
}