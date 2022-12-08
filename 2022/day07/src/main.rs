use anyhow::{Result};
use itertools::Itertools;
use std::{collections::HashMap, fs, time::Instant};

type FolderPath = Vec<String>;

struct Folder {
    files_size: usize,
    children: Vec<FolderPath>,
}

fn solution(input_s: &str) -> Result<(String, String)> {
    let mut dirs: HashMap<FolderPath, Folder> = HashMap::new();
    {
        let mut pwd: FolderPath = Vec::new();
        // Trick: split on $ -- https://www.reddit.com/r/adventofcode/comments/zesk40/2022_day_7_solutions/iz8f2r7/
        for part in input_s.trim().split('$').skip(1) {
            let mut lines = part.trim().split('\n');
            match lines.next() {
                Some("ls") => {
                    let mut d = Folder{files_size:0, children: Vec::new()};
                    for ln in lines {
                        if ln.starts_with("dir") {
                            let mut child = pwd.clone();
                            child.push(ln[4..].to_string());
                            d.children.push(child);
                        } else {
                            d.files_size +=ln.split(' ').next().unwrap().parse::<usize>()?
                        }
                    }
                    dirs.insert(pwd.clone(), d);
                },
                Some("cd /") => {pwd = Vec::new();},
                Some("cd ..") => {pwd.pop();},
                Some(cd_a) => {pwd.push(cd_a[3..].to_string())},
                None => panic!(),
            }
        }
    }

    // Fill sizes map by starting from paths with the most levels and working towards root
    let mut sizes: HashMap<&FolderPath, usize> = HashMap::new();
    for (folder_path, entry) in dirs.iter().sorted_by_key(|(v, _)| -(v.len() as isize)) {
        sizes.insert(folder_path, entry.files_size + entry.children.iter().map(|c| sizes.get(c).unwrap()).sum::<usize>());
    }
    let part1: usize = sizes.values().filter(|&v| *v <= 100000).sum();

    let current_total = sizes.get(&Vec::new()).unwrap();
    let min_free = current_total +  30000000 - 70000000;
    let part2 = sizes.values().sorted().find(|&size| *size >= min_free).unwrap();

    Ok((part1.to_string(), part2.to_string()))
}

#[test]
fn test_solution() -> Result<()> {
    let res = solution(&fs::read_to_string("test01.txt")?)?;
    println!("Part 1: {}\nPart 2: {}", res.0, res.1);
    assert!(res.0 == "95437");
    assert!(res.1 == "24933642");
    Ok(())
}

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?;
    let start = Instant::now();
    let res = solution(&input)?;
    println!("Part 1: {}\nPart 2: {}\nRuntime: {}us", res.0, res.1, start.elapsed().as_micros());
    Ok(())
}
