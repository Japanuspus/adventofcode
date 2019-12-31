use num_bigint::ToBigInt;
use num_bigint::{BigInt};
use num_traits::cast::{ToPrimitive};
use modinverse;
// use num_integer::{Integer};

#[derive(Debug)]
enum Shuffle {
    DealInc(isize),
    Cut(isize),
    DealNew
}

// deal with increment 64
// deal into new stack
// cut 1004
fn parse_shuffle(ln: &str) -> Shuffle {
    let p: Vec<_> = ln.split(' ').collect();
    let n = p.len();
    match p[n-2] {
        "increment" => Shuffle::DealInc(p[n-1].parse().unwrap()),
        "new" => Shuffle::DealNew,
        "cut" => Shuffle::Cut(p[n-1].parse().unwrap()),
        _ => {panic!();}
    }
}

fn do_shuffle(deck: &[u16], cmd: &Shuffle) -> Vec<u16> {
    match cmd {
        Shuffle::DealInc(n) => {
            let n = *n as usize;
            let ndeck = deck.len();
            let mut d2: Vec<u16> = (0..(ndeck as u16)).collect();
            let mut p = 0;
            for v in deck {
                d2[p] = *v;
                p = (p+n) % ndeck;
            }
            d2
        }
        Shuffle::Cut(v) => {
            let k = if *v<0 {deck.len()-v.abs() as usize} else {*v as usize};
            deck[k..].iter().chain(deck[..k].iter()).cloned().collect()
        }
        Shuffle::DealNew => 
            deck.iter().rev().cloned().collect()
    }
}

// All shuffle operations are linear maps (mod decksize)
fn coefficients(cmd: &Shuffle) -> (isize, isize) {
    match cmd {
        Shuffle::DealInc(k) => (*k, 0),
        Shuffle::Cut(v) => (1, -v),
        Shuffle::DealNew => (-1, -1)
    }
}


/// Return coefficients corresponding to applying first m0 and then m1
/// 
/// Corresponds to matrix product
/// (a1 b1)  (a0 b0)
/// (0   1)  (    1)
/// 
/// (a1*a0 a1*b0+b1)
/// (    0        1)


fn solve_naive(cmds: &Vec<Shuffle>, n_deck: usize, index_initial: usize) -> usize {
    let deck0: Vec<u16> = (0..n_deck as u16).collect();
    let deck = cmds.iter().fold(deck0, |d, c| do_shuffle(&d, c));
    let (idx,_) = deck.iter().enumerate().filter(|(_, v)| **v == index_initial as u16).next().unwrap();
    idx
}

type Map = (BigInt, BigInt);

fn combined_map(cmds: &Vec<Shuffle>) -> Map {
    cmds.iter()
    .map(coefficients)
    .fold(
        (1.to_bigint().unwrap(), 0.to_bigint().unwrap()),
        // a1, b1 is applied last
        |(a0, b0), (a1,b1)| (a0*a1, a1*b0+b1))
}

fn map_repeated(m: &Map, n_deck: usize, n_repeat: usize) -> Map {
    let (a_comb, b_comb) = m;
    let mut a_tot = 1.to_bigint().unwrap();
    let mut b_tot = 0.to_bigint().unwrap();

    let mut a=a_comb % n_deck;
    let mut b=b_comb % n_deck;
    let mut bits=n_repeat;
    loop {
        if bits & 1 != 0 {
            let b_tot2 = b_tot.clone()*&a+&b;
            let a_tot2 = a_tot.clone()*&a;
            b_tot = b_tot2 % n_deck;
            a_tot = a_tot2 % n_deck;
        }
        bits>>=1;
        if bits==0 {break}
        let b2 = b.clone()*&a+&b;
        let a2 = a.clone()*&a;
        a=a2 % n_deck;
        b=b2 % n_deck;
    }
    (a_tot, b_tot)
}

fn apply_map(m: &Map, n_deck: usize, index_initial: usize) -> usize {
    let (a_comb, b_comb) = m;
    let index = 
        (index_initial.to_bigint().unwrap() * a_comb + b_comb) % n_deck;
    ((index+n_deck)%n_deck).to_usize().unwrap()
}

fn solve_fast(cmds: &Vec<Shuffle>, n_deck: usize, index_initial: usize) -> usize {
    apply_map(&combined_map(cmds), n_deck, index_initial)
}

fn solve_fast_repeated(cmds: &Vec<Shuffle>, n_deck: usize, index_initial: usize, n_repeat: usize) -> usize {
    let m = combined_map(cmds);
    apply_map(&map_repeated(&m, n_deck, n_repeat), n_deck, index_initial)
}

fn solve_repeated(cmds: &Vec<Shuffle>, n_deck: usize, index_initial: usize, n_repeat: usize) -> usize {
    let m = combined_map(cmds);
    (0..n_repeat).fold(index_initial, |idx, _| apply_map(&m, n_deck, idx))
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input file");

    let cmds: Vec<Shuffle> = input
    .lines()
    .map(|ln| parse_shuffle(ln))
    .collect();

    
    // Part 1
    println!("Part 1: {}", solve_naive(&cmds, 10007, 2019));
    println!("Part 1: {} - fast", solve_fast(&cmds, 10007, 2019));

    println!("Part 1 repeated 7 times: {}", solve_repeated(&cmds, 10007, 2019, 7));
    println!("Part 1 repeated 7 times: {} - fast", solve_fast_repeated(&cmds, 10007, 2019, 7));

    // Part 2 -- read incorrectly
    let n_deck:usize = 119315717514047; 
    let index_final = 2020;
    let n_repeat:usize = 101741582076661;

    println!("Part 2 - read incorrectly: {}", solve_fast_repeated(&cmds, n_deck, index_final, n_repeat));

    // Part 2
    let m0 = combined_map(&cmds);
    let (abig, bbig) = map_repeated(&m0, n_deck, n_repeat);
    let a = ((abig.clone() + n_deck)%n_deck).to_usize().unwrap();
    let b = ((bbig.clone() + n_deck)%n_deck).to_usize().unwrap();
    let n = n_deck;
    // Solve a*x + b = d mod n
    //      u= a*x = d-b = a1 mod n
    //      u= a*x = 0 mod a
    let a1 = index_final + (n_deck - b);
    println!("Want solution for {} * x  = {} mod {}", a, a1, n);
    // x congruent 41685581334351 (mod 119315717514047) via wolfram alpha

    // modinverse does not work for BigInt and konks out on usize...
    let (g, _m1, m2) = modinverse::egcd(n, a);
    assert_eq!(g, 1); // gcd a,n == 1

    let res = abig * &m2 * &a1 % &n_deck;
    let res2 = (res + &n_deck) % &n_deck;
    println!("Part 2: {}", res2);
}

