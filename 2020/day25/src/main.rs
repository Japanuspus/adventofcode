use std::fs;
use anyhow::Result;

// x <-1, repeat: x <- (x*s) % 20201227
fn brute_loop_size(pubkey: usize) -> usize {
    let s = 7;
    let m = 20201227;
    
    let mut x = 1;
    let mut n = 0;
    while !(x==pubkey) {
        n+=1;
        x = (x*s) % m;
    }
    n
}

fn enc_key(s: usize, n: usize) -> usize {
    let m = 20201227;
    (0..n).fold(1, |x, _| (x*s)%m)
}

#[test]
fn test_part1() {
    assert_eq!(brute_loop_size(5764801), 8);
    assert_eq!(enc_key(17807724, 8), 14897079)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let pub_keys: Vec<usize> = input.lines().map(|ln| ln.parse::<usize>()).collect::<Result<_,_>>()?;
    
    let loop_size: Vec<usize> = pub_keys.iter().map(|k| brute_loop_size(*k)).collect();
    let enc1 = enc_key(pub_keys[0], loop_size[1]);

    println!("Part 1: {}", enc1);

    Ok(())
}
