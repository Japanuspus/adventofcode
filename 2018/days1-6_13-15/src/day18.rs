#[cfg(test)]
mod tests {
    use super::*;
    const TT:& str = "";
    #[test]
    fn part1() {
        assert_eq!(part1_01(TT),0);
    }
}


// fn charkey(c: &u8) -> u8 {
//     match b[1] {
//         b'.' => 0,
//         b'|' => 1,
//         b'#' => 2,
//         _ => panic!
//     }
// }

// fn prop(a: & [u8], b: & [u8], c: & [u8]) -> u8 {
//     // Not very efficient, but just fire the big gun
//     let mut counts: [u8;3] = [0,0,0];
//     for v in [a, b, c].iter().flatten() {
//         counts[charkey(v)]+=1;
//     }
    
//     match b[1] {
//         b'.' => if counts[1]>=3 {b'|'} else {b'.'},
//         b'|' => if counts[2]>=3 {b'#'} else {b'|'},
//         b'#' => if (counts[1]>0 && counts[2]>0) {b'#'} else {b'.'}
//         _ => panic!
//     }

// }

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