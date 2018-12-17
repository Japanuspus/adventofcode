#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        assert_eq!(part1_01("5"),"0124515891");
        assert_eq!(part2_01("51589"), 9);
    }
}

pub fn part1_01(data: &str) -> String {
    let mut a: usize = 0;
    let mut b: usize = 1;
    let mut d: Vec<u8> = vec![3, 7];
    let n_train: usize = data.trim().to_string().parse().unwrap();
    while d.len()<n_train+10 {
        let ra = d[a];
        let rb = d[b];
        d.extend((ra+rb).to_string().as_bytes().iter().map(|c| c-b'0'));
        a = (a + ra as usize + 1) % d.len();
        b = (b + rb as usize + 1) % d.len();
    };
    
    String::from_utf8(d[n_train .. (n_train+10)].iter().map(|i| i+b'0').collect()).expect("??")
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

pub fn part2_01(data: &str) -> usize {
    let mut a: usize = 0;
    let mut b: usize = 1;
    let mut d: Vec<u8> = vec![3, 7];
    let tgt: Vec<u8> = data.trim().as_bytes().iter().map(|c| c-b'0').collect();
    let ntgt = tgt.len();

    loop {
        let ra = d[a];
        let rb = d[b];
        d.extend((ra+rb).to_string().as_bytes().iter().map(|c| c-b'0'));
        a = (a + ra as usize + 1) % d.len();
        b = (b + rb as usize + 1) % d.len();
        //At most two new characters of which at least 1 must be part of solution
        let sa = if d.len() > 2+ntgt {d.len() - (2+ntgt)} else {0};
        if let Some(k) = find_subsequence(&d[sa .. d.len()], &tgt) {
            break sa+k
        }
    }
}

pub fn run(data: &str) {
    println!("Part 1: {}", part1_01(&data));
    println!("Part 2: {}", part2_01(&data));
}