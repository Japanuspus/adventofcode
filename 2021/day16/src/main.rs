#![allow(unused_imports, dead_code)]
use anyhow::{Result, Context};
use std::fs;
// use std::collections::BitVec; //unstable
use bitvec::prelude::*;
use nom::IResult; 


#[derive(Debug)]
struct Package {
    version: u8,
    type_id: u8,
    content: Content,
}

#[derive(Debug)]
enum Content {
    Literal {
        value: usize,
    },
    Operator {
        children: Vec<Package>,
    }
}

type BS = BitSlice<Msb0, u8>;

// fn parse_package(bits: &BS) -> IResult<&BS, Package> {
//     let version = bits[..3].to_bitvec().load_be::<u8>();
//     Ok((&bits, Package{version}))
// }

fn parse_literal(bits: &BS) -> Result<(&BS, Content)> {
    let mut rest = &bits[0..];
    let mut vbuf = BitVec::<Msb0>::new();
    loop {
        vbuf.extend_from_bitslice(&rest[1..5]);
        let do_cont = rest[0] as bool;
        rest = &rest[5..];
        if !do_cont {break;}
    };
    let value = vbuf.load_be::<usize>();
    //println!("Value: {} from vbuf: {:?}", &value, &vbuf);
    Ok((&rest, Content::Literal{value}))
}

fn parse_operator_totbit(bits: &BS) -> Result<(&BS, Content)> {
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
    Ok((&rest, Content::Operator {children}))
}

fn parse_operator_npak(bits: &BS) -> Result<(&BS, Content)> {
    let npak = bits[..11].to_bitvec().load_be::<usize>();
    let mut rest = &bits[11..];
    let mut children = Vec::new();
    for _ in 0..npak {
        let (new_rest, p) = parse_package(rest)?;
        rest = new_rest;
        children.push(p);
    }
    Ok((&rest, Content::Operator {children}))
}


fn parse_operator(bits: &BS) -> Result<(&BS, Content)> {
    if bits[0] {parse_operator_npak(&bits[1..])} else {parse_operator_totbit(&bits[1..])} 
}


fn parse_package(bits: &BS) -> Result<(&BS, Package)> {
    let version = bits[..3].to_bitvec().load_be::<u8>();
    let type_id = bits[3..6].to_bitvec().load_be::<u8>();
    let (rest, value) = match type_id  {
        4 => parse_literal(&bits[6..])?,
        _ => parse_operator(&bits[6..])?,
    };
    Ok((rest, Package{version, type_id, content: value}))
}

fn parse(input_s: &str) -> Result<Package> {
    let mut s = input_s.trim().to_owned();
    if s.len() % 2 > 0 {s.push('0')}
    let input: Vec<u8> = hex::decode(&s)?;
    let bits = input.view_bits::<Msb0>();
    let (_rest, p) = parse_package(bits)?;
    //assert!(rest.len() == 0);
    //println!("With {} bits remaining, package is: {:?}", rest.len(), &p);
    Ok(p)
}

#[test]
fn test_literal() {
    let p = parse("D2FE28").unwrap();
    assert!(p.version==6);
    match p.content {
        Content::Literal{value} => {assert!(value == 2021);}
        _ => {panic!();},
    }
}


#[test]
fn test_op01() {
    let p = parse("38006F45291200").unwrap();
    assert!(p.version==1);
    match p.content {
        Content::Operator{children} => {assert!(children.len()==2)}
        _ => {panic!();},
    }
}

#[test]
fn test_op02() {
    let p = parse("EE00D40C823060").unwrap();
    assert!(p.version==7);
    match p.content {
        Content::Operator{children} => {assert!(children.len()==3)}
        _ => {panic!();},
    }
}

fn vsum(p: &Package) -> usize {
    p.version as usize + match &p.content {
        Content::Literal{value: _} => 0,
        Content::Operator{children} => children.iter().map(|p| vsum(p)).sum(),
    }
}

fn compute(p: &Package) -> usize {
    match &p.content {
        Content::Literal{value} => *value,
        Content::Operator{children} => {
            let vs: Vec<usize> = children.iter().map(compute).collect();
            match p.type_id {
                0 => vs.iter().sum(),  // sum packets - their value is the sum of the values of their sub-packets. If they only have a single sub-packet, their value is the value of the sub-packet.
                1 => vs.iter().product(),  // product packets - their value is the result of multiplying together the values of their sub-packets. If they only have a single sub-packet, their value is the value of the sub-packet.
                2 => *vs.iter().min().unwrap(),  // minimum packets - their value is the minimum of the values of their sub-packets.
                3 => *vs.iter().max().unwrap(),  // maximum packets - their value is the maximum of the values of their sub-packets.
                5 => if vs[0]>vs[1] {1} else {0},  // greater than packets - their value is 1 if the value of the first sub-packet is greater than the value of the second sub-packet; otherwise, their value is 0. These packets always have exactly two sub-packets.
                6 => if vs[0]<vs[1] {1} else {0},  // less than packets - their value is 1 if the value of the first sub-packet is less than the value of the second sub-packet; otherwise, their value is 0. These packets always have exactly two sub-packets.
                7 => if vs[0]==vs[1] {1} else {0},  // equal to packets - their value is 1 if the value of the first sub-packet is equal to the value of the second sub-packet; otherwise, their value is 0. These packets always have exactly two sub-packets.
                _ => panic!(),
            }
        }
    }
}

fn solution(input_s: &str) -> Result<()> {
    let p = parse(input_s)?;
    println!("Part 1: {}", vsum(&p));
    println!("Part 2: {}", compute(&p));
    Ok(())
}

fn main() -> Result<()> {
    solution(&fs::read_to_string("input.txt")?)
}
