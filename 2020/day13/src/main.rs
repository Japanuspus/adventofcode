use num::BigInt;
use std::fs;
use anyhow::{Result, Error};
use num::Integer;

#[test]
fn test_extended_euclid() {
    let e = isize::extended_gcd(&240, &46);
    assert_eq!(e.x, -9);
    assert_eq!(e.y, 47);
    assert_eq!(e.gcd, 2);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RSpec<T: Clone >{n: T, a: T}

fn chinese_remainder<T: Integer+Clone>(n1: RSpec<T>, n2: RSpec<T>) -> RSpec<T> {  
    let ee = T::extended_gcd(&n1.n, &n2.n);
    assert!(ee.gcd==T::one());
    let n = n1.n.clone()*n2.n.clone();
    let a = (n1.a*ee.y*n2.n+n2.a*ee.x*n1.n).mod_floor(&n); // rem_euclid(n);
    RSpec{n, a}
}

#[test]
fn test_chinese_remainder() {
    let res=chinese_remainder(RSpec::<isize>{n: 3, a:2}, RSpec::<isize>{n: 5, a:1});
    assert_eq!(res, RSpec::<isize>{n: 15, a:11});
}

fn part2<T: Integer+Clone+std::str::FromStr+num::FromPrimitive>(ln: &str) -> Result<T> {
    let specs: Vec<RSpec<T>> = ln.split(',').enumerate()
    .filter_map(|(i, cc)| cc.parse::<T>().ok().map(|n| RSpec::<T>{n, a: T::from_isize(-(i as isize)).unwrap()}))
    .collect();
    let mut spec_iter = specs.into_iter();
    let spec_0 = spec_iter.next().ok_or(Error::msg("No specs"))?;
    Ok(spec_iter.fold(spec_0, move |a,b| chinese_remainder(a, b)).a)
}

#[test]
fn test_part_2() {
    assert_eq!(part2::<isize>("17,x,13,19").ok(), Some(3417));
}

fn main() -> Result<()> {
    let input_file = fs::read_to_string("input.txt")?;
    let mut input = input_file.lines();
    let dep: isize = input.next().map(|ln| ln.parse()).ok_or(Error::msg("No first line"))??;
    let bus_ln = input.next().ok_or(Error::msg("No bus line"))?;
    let busses: Vec<isize> = bus_ln.split(',').filter_map(|cc| cc.parse::<isize>().ok()).collect();

    println!("Part 1: {}", busses.iter()
    .map(|b| (b-(dep % b), b))
    .min_by_key(|(w,_b)| *w)
    .map(|(w,b)| w*b)
    .unwrap_or(0));

    println!("Part 2: {}", part2::<BigInt>(bus_ln)?);
    Ok(())
}
