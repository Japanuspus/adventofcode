#![allow(unused_imports, dead_code)]
#![allow(unused_imports, dead_code)]

use anyhow::{Result, Context};
use itertools::Itertools;
use regex::internal::Input;
use std::{fs, rc::Rc};
use std::str::FromStr;

use parse_display::{Display, FromStr};

#[derive(PartialEq, Debug, Display)] //#, FromStr, Display, 
enum Node {
    #[display("{a}")]
    Number {a: isize},
    #[display("[{a},{b}]")]
    Pair {a: Box<Node>, b: Box<Node>},
}

fn parse_node(i: &[u8]) -> Result<(&[u8], Node)> {
    let c = i[0];
    if c==b'[' {
        let (i, a) = parse_node(&i[1..])?;
        let (i, b) = parse_node(&i[1..])?; //skipping ,
        Ok((&i[1..], Node::Pair{a: Box::new(a), b: Box::new(b)}))
    } else {
        Ok((&i[1..], Node::Number{a: (c-b'0') as isize}))
    } 
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_node(s.as_bytes())
        .and_then(|(_, v)| Ok(v)).with_context(|| format!("Parsing {}", s))
    }
}

// trait NodeVisitor {
//     fn visit(&mut self, n: usize, node: &mut Node);
// }

// fn walk_tree(t: &mut Node, n: usize, v: &mut dyn NodeVisitor) {
//     match &t {
//         Node::Number{a:_} => {},
//         Node::Pair{mut a, mut b} => {
//             walk_tree(&mut a, n+1, v);
//             v.visit(n, t);
//             walk_tree(&mut b, n+1, v);
//         }
//     }
// }

fn add_first(t: &mut Node, vo: Option<isize>) {
    if let Some(v) = vo {
        match t {
            Node::Number{a} => {*a+=v;},
            Node::Pair{a, b:_} => {add_first(&mut *a, vo)},
        }
    }
}

fn add_last(t: &mut Node, vo: Option<isize>) {
    if let Some(v) = vo {
        match t {
            Node::Number{a} => {*a+=v;},
            Node::Pair{a:_, b} => {add_last(&mut *b, vo)},
        }
    }
}

fn explode(t: &mut Node, n: usize) -> (bool, Option<isize>, Option<isize>) {
    match t {
        Node::Number{a:_} => (false, None, None),
        Node::Pair{a, b} if n>3 => 
            match (&**a, &**b) {
                (Node::Number{a: n1}, Node::Number{a: n2}) => {
                    let res = (true, Some(*n1), Some(*n2));
                    *t = Node::Number{a: 0}; 
                    res
                },
                _ => panic!()
            },
        Node::Pair{a, b} => {
            let (t, a1, a2) = explode(&mut *a, n+1);
            if t {add_first(&mut *b, a2); return (true, a1, None)}
            let (t, a1, a2) = explode(&mut *b, n+1);
            if t {add_last(&mut *a, a1); return (true, None, a2)}
            (false, None, None)
        }
    }
}

fn do_explode(t: &mut Node) -> bool {
    explode(t, 0).0
}

#[test]
fn test_explode() {
    let mut n = "[[[[[9,8],1],2],3],4]".parse::<Node>().unwrap();
    assert!(do_explode(&mut n));
    assert_eq!(format!("{}", &n), "[[[[0,9],2],3],4]");
}

fn split(t: &mut Node) -> bool {
    match t {
        Node::Number{a} => if *a>=10 {
            *t = Node::Pair{
                a: Box::new(Node::Number{a: *a/2}), 
                b: Box::new(Node::Number{a: (*a+1)/2})
            }; 
            true
        } else {
            false
        },
        Node::Pair{a, b} => {split(&mut *a) || split(&mut *b)}
    }
}

// #[test]
// fn test_split() {
//     let mut n = "[11,13]".parse::<Node>().unwrap();
//     assert!(split(&mut n));
//     assert_eq!(format!("{}", &n), "[[5,6],13]");
// }

fn reduce(i: &mut Node) {
    loop {
        while do_explode(i) {}
        if !split(i) {break}
    }
}

fn magnitude(t: &Node) -> isize {
    match t {
        &Node::Number{a} => a,
        Node::Pair{a, b} => 3*magnitude(&*a)+2*magnitude(&*b),
    }
}

#[test]
fn test_reduce() {
    let mut n = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".parse::<Node>().unwrap();
    reduce(&mut n);
    assert_eq!(format!("{}", &n), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
}

fn solution(input_s: &str) -> Result<()> {
    let input: Vec<Node> = input_s
        .trim()
        .split("\n")
        .map(|ln| ln.parse())
        .collect::<Result<_,_>>()?;

    //println!("Part 1: {}", input.len());
    let s = input.into_iter().reduce(|a, b| {let mut n = Node::Pair{a: Box::new(a), b: Box::new(b)}; reduce(&mut n); n}).unwrap();
    println!("Sum: {}", s);
    println!("Part 1: {}", magnitude(&s));
    Ok(())
}

fn main() -> Result<()> {
    println!("** TEST **");
    solution(&fs::read_to_string("test07.txt")?)?;
    println!("\n** INPUT **");
    solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
