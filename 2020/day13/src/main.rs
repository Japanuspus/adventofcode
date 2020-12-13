use std::collections::VecDeque;
use std::fs;
use anyhow::{Result, Error};
// use itertools::Itertools;
// use apply::Also;

fn extended_euclid(a: isize, b: isize) -> (isize, isize, isize) {
    assert!(a>b);
    struct Row {r: isize, s: isize, t: isize};
    let mut table: VecDeque::<Row> = VecDeque::new();
    table.push_back(Row{r: a, s: 1, t: 0});
    table.push_back(Row{r: b, s: 0, t: 1});
    loop {
        let rim1 = table.pop_front().unwrap();
        let ri = table.front().unwrap();
        let q = rim1.r.div_euclid(ri.r);
        let rip1 = Row{
            r:rim1.r - q*ri.r,
            s:rim1.s - q*ri.s,
            t:rim1.t - q*ri.t,
        };
        if rip1.r == 0 {break;};
        
        table.push_back(rip1);
    };
    let r = table.pop_front().unwrap();
    (r.s, r.t, r.r)
}

#[test]
fn test_extended_euclid() {
    // let mut dq: VecDeque::<isize> = vec![1,2];
    let (x, y, gcd) = extended_euclid(240, 46);
    assert_eq!(x, -9);
    assert_eq!(y, 47);
    assert_eq!(gcd, 2);
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct RSpec{n: isize, a: isize}

fn chinese_remainder(n1: &RSpec, n2: &RSpec) -> RSpec{
    let (m1, m2, gcd) = if n1.n>n2.n {
        extended_euclid(n1.n, n2.n)
    } else {
        let (m2,m1,gcd) = extended_euclid(n2.n, n1.n);
        (m1, m2, gcd)
    };
    assert_eq!(gcd, 1);
    
    RSpec{
        n: n1.n*n2.n, 
        a: n1.a*m2*n2.n+n2.a*m1*n1.n
    }
}

#[test]
fn test_chinese_remainder() {
    let res=chinese_remainder(&RSpec{n: 3, a:2}, &RSpec{n: 5, a:1});
    assert_eq!(res, RSpec{n: 15, a:-4});

    let res=chinese_remainder(&RSpec{n: 5, a:1}, &RSpec{n: 3, a:2});
    assert_eq!(res, RSpec{n: 15, a:-4});
}

fn part2(ln: &str) -> Result<isize> {
    let specs: Vec<RSpec> = ln.split(',').enumerate()
    .filter_map(|(i, cc)| cc.parse::<isize>().ok().map(|n| RSpec{n, a: -(i as isize)}))
    .collect();
    let rtot = specs[1..].iter().fold(
        specs[0].clone(),
        |a,b| chinese_remainder(&a, b));
    println!("RTOT {:?}", rtot);
    Ok(rtot.a)
}


fn main() -> Result<()> {
    let input_file = fs::read_to_string("input.txt")?;
    let mut input = input_file.lines();
    let dep: isize = input.next().map(|ln| ln.parse()).ok_or(Error::msg("No first line"))??;
    let bus_ln = input.next().ok_or(Error::msg("No bus line"))?;
    let busses: Vec<isize> = bus_ln.split(',').filter_map(|cc| cc.parse::<isize>().ok()).collect();

    println!("Part 1: {}",     busses.iter()
    .map(|b| (b-(dep % b), b))
    .min_by_key(|(w,_b)| *w)
    .map(|(w,b)| w*b)
    .unwrap_or(0));

    println!("Part 2: {}", part2(bus_ln)?);
    Ok(())
}
