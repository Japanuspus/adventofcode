#![allow(unused_imports, dead_code)]

use anyhow::{Context, Result};
use itertools::Itertools;
use std::{collections::HashMap, fs};

use parse_display::{Display, FromStr};

#[derive(Debug, Display, FromStr)]
#[display("{size} {name}")]
struct FileEntry {
    size: usize,
    name: String,
}

type FolderPath = Vec<String>;

struct Folder {
    files_size: usize,
    children: Vec<FolderPath>,
}
impl Folder {
    fn new() -> Self {
        Self {
            files_size: 0,
            children: Vec::new(),
        }
    }
}

fn solution(input_s: &str) -> Result<(String, String)> {
    let mut dirs: HashMap<FolderPath, Folder> = HashMap::new();
    {
        let mut pwd: FolderPath = Vec::new();
        let mut pwd_data: Option<Folder> = None;
        for ln in input_s.trim().split("\n") {
            if ln.starts_with('$') {
                pwd_data.take().and_then(|v| dirs.insert(pwd.clone(), v));
                if ln.starts_with("$ cd ") {
                    match &ln[5..] {
                        "/" => {
                            pwd = Vec::new();
                        }
                        ".." => {
                            pwd.pop();
                        }
                        a => {
                            pwd.push(a.to_string());
                        }
                    }
                } else {
                    pwd_data = Some(Folder::new());
                }
            } else {
                if ln.starts_with("dir") {
                    let mut child = pwd.clone();
                    child.push(ln[4..].to_string());
                    if let Some(d) = &mut pwd_data {
                        d.children.push(child)
                    };
                } else {
                    let file_entry: FileEntry = ln
                        .parse::<FileEntry>()
                        .with_context(|| format!("Parsing '{}'", ln))?;
                    if let Some(d) = &mut pwd_data {
                        d.files_size += file_entry.size
                    };
                }
            }
        }
        pwd_data.take().and_then(|v| dirs.insert(pwd, v));
    }

    // Fill sizes map by starting from paths with the most levels and working towards root
    let mut sizes: HashMap<&FolderPath, usize> = HashMap::new();
    for (folder_path, entry) in dirs.iter().sorted_by_key(|(v, _)| -(v.len() as isize)) {
        let tot: usize = entry.files_size
            + entry
                .children
                .iter()
                .map(|c| sizes.get(c).unwrap())
                .sum::<usize>();
        sizes.insert(folder_path, tot);
    }
    let part1: usize = sizes.values().filter(|&v| *v <= 100000).sum();

    let max_total: usize = 70000000 - 30000000;
    let current_total = sizes.get(&Vec::new()).unwrap();
    let min_free = current_total - max_total;
    let part2 = sizes
        .values()
        .sorted()
        .find(|&size| *size >= min_free)
        .unwrap();

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
    let res = solution(&fs::read_to_string("input.txt")?)?;
    println!("Part 1: {}\nPart 2: {}", res.0, res.1);
    Ok(())
}
