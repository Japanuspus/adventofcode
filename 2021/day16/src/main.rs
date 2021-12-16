#![allow(unused_imports, dead_code)]
use anyhow::{Result, Context};
use std::fs;
// use std::collections::BitVec; //unstable
use bitvec::prelude::*;
use nom::IResult; 


#[derive(Debug)]
struct Package {
    version: u8,
    value: Value,
}

#[derive(Debug)]
enum Value {
    Literal,
    Operator {
        children: Vec<Package>,
    }
}

type BS = BitSlice<Msb0, u8>;

// fn parse_package(bits: &BS) -> IResult<&BS, Package> {
//     let version = bits[..3].to_bitvec().load_be::<u8>();
//     Ok((&bits, Package{version}))
// }

fn parse_literal(bits: &BS) -> Result<(&BS, Value)> {
    let mut idx=0;
    while bits[idx] {idx+=5}
    Ok((&bits[(idx+5)..], Value::Literal))
}

fn parse_operator_totbit(bits: &BS) -> Result<(&BS, Value)> {
    let totbit = bits[..15].to_bitvec().load_be::<usize>();
    // The default `load` depends on arch!
    //println!("Totbit: {} of {}, from {:?}", &totbit, bits.len()-15, &bits[..15]);
    let mut payload = &bits[15..(15+totbit)];
    let mut children = Vec::new();
    while payload.len()>0 {
        let (new_payload, p) = parse_package(payload)?;
        payload = new_payload;
        children.push(p);
    }
    let rest = &bits[(15+totbit)..];
    Ok((&rest, Value::Operator {children}))
}

fn parse_operator_npak(bits: &BS) -> Result<(&BS, Value)> {
    let npak = bits[..11].to_bitvec().load_be::<usize>();
    let mut rest = &bits[11..];
    let mut children = Vec::new();
    for _ in 0..npak {
        let (new_rest, p) = parse_package(rest)?;
        rest = new_rest;
        children.push(p);
    }
    Ok((&rest, Value::Operator {children}))
}


fn parse_operator(type_id: u8, bits: &BS) -> Result<(&BS, Value)> {
    if bits[0] {parse_operator_npak(&bits[1..])} else {parse_operator_totbit(&bits[1..])} 
}


fn parse_package(bits: &BS) -> Result<(&BS, Package)> {
    let version = bits[..3].to_bitvec().load_be::<u8>();
    let type_id = bits[3..6].to_bitvec().load_be::<u8>();
    let (rest, value) = match type_id  {
        4 => parse_literal(&bits[6..])?,
        _ => parse_operator(type_id, &bits[6..])?,
    };
    Ok((rest, Package{version, value}))
}

fn parse(input_s: &str) -> Result<Package> {
    let mut s = input_s.trim().to_owned();
    if s.len() % 2 > 0 {s.push('0')}
    let input: Vec<u8> = hex::decode(&s)?;
    let bits = input.view_bits::<Msb0>();
    let (rest, p) = parse_package(bits)?;
    //assert!(rest.len() == 0);
    //println!("With {} bits remaining, package is: {:?}", rest.len(), &p);
    Ok(p)
}

#[test]
fn test_literal() {
    let p = parse("D2FE28").unwrap();
    assert!(p.version==6);
    // assert p.value.value == 2021
}


#[test]
fn test_op01() {
    let p = parse("38006F45291200").unwrap();
    assert!(p.version==1);
    match p.value {
        Value::Operator{children} => {assert!(children.len()==2)}
        _ => {panic!();},
    }
}

#[test]
fn test_op02() {
    let p = parse("EE00D40C823060").unwrap();
    assert!(p.version==7);
    match p.value {
        Value::Operator{children} => {assert!(children.len()==3)}
        _ => {panic!();},
    }
}

fn vsum(p: &Package) -> usize {
    p.version as usize + match &p.value {
        Value::Literal => 0,
        Value::Operator{children} => children.iter().map(|p| vsum(p)).sum(),
    }
}

fn solution(input_s: &str) -> Result<()> {
    let p = parse(input_s)?;
    println!("Part 1: {}", vsum(&p));
    println!("Part 2: {}", 0);
    Ok(())
}

fn main() -> Result<()> {
    solution(&fs::read_to_string("input.txt")?)
}
