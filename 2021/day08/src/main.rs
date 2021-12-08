use anyhow::{Result, Context};
use itertools::Itertools;
use std::fs;
use std::collections::{HashSet, HashMap};


type CSet = HashSet<char>;
fn str2cset(s: &str) -> CSet {
    s.chars().collect()
}


fn solution(input_s: &str) -> Result<(usize, usize)> {
    let input: Vec<(Vec<&str>, Vec<&str>)> = input_s
        .trim()
        .split("\n")
        .map(|s| {
            let mut p=s.split(" | ");
            (p.next().unwrap().split_whitespace().collect_vec(), 
                p.next().unwrap().split_whitespace().collect_vec())
        })
        .collect();

    // unique lengths: 2, 4, 3, 7
    let p1=input.iter().map(|(_a, b)| b.iter().map(|d| d.len()).filter(|n| (*n==2) || (*n==3) || (*n==4) || (*n==7 )).count()).sum();
    println!("Part 1: {}", p1);

    // part 2
    let digits: [&str;10] = [
        "abcefg", //0 - 6
        "cf",     //1 - 2
        "acdeg",  //2 - 5
        "acdfg",  //3 - 5
        "bcdf",   //4 - 4
        "abdfg",  //5 - 5
        "abdefg", //6 - 6
        "acf",    //7 - 3
        "abcdefg",//8 - 7
        "abcdfg", //9 - 6
    ];

    // a: l(3) - l(2)

    let mut p_out: Vec<CSet> = (0..=9).map(|_| str2cset("")).collect(); //possible outputs at each len
    for d in digits {
        let dst = &mut p_out[d.len()];
        for c in str2cset(d) {
            dst.insert(c);
        }
    }
    println!("Length to possible output segments map: {:?}", p_out);


    let vs=&input[0].0;
    let mut map: HashMap<char, CSet> = HashMap::new();
    for v in vs.iter().cloned() {
        // v is an observed output. for each segment only keep possible output segments according to length
        let possible = &p_out[v.len()]; // possible output segments given length
        for c in v.chars() {
            let cc = map.entry(c).or_insert(str2cset("abcdefg"));
            *cc = cc.intersection(possible).cloned().collect();
        }
    }
    println!("Possible outputs: {:?}", map);

    Ok((p1, 0))
}

#[test]
fn test_solution() -> Result<()> {
    let (a, b) = solution(&fs::read_to_string("test00.txt")?)?;
    assert!(a==1);
    assert!(b==0);
    Ok(())
}

fn main() -> Result<()> {
    println!("*** Test ****");
    solution(&fs::read_to_string("test02.txt")?)?;
    println!("\n*** Input ****");
    solution(&fs::read_to_string("input.txt")?)?;
    Ok(())
}
