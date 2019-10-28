extern crate nom;
extern crate kdtree;

use std::io;

use nom::bytes::complete::{tag};

fn number(input: &str) -> nom::IResult<&str, f32> {
    nom::combinator::map_res(
        nom::character::complete::digit1, 
        |s: &str| s.parse::<f32>())(input)
}

fn parse_point(input: &str) -> [f32; 2] {
    let (_, (x, _, y)) = nom::sequence::tuple((
        number, tag(", "), number
    ))(input).unwrap();
    [x, y]
}

fn manhattan_dist(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter())
        .map(|(x, y)| f32::abs((*x) - (*y)))
        .sum()
}

fn min_max<T> (numbers: T) -> (usize, usize) 
where
    T: IntoIterator<Item=f32> 
{ 
    let mut min = std::f32::INFINITY;
    let mut max = std::f32::NEG_INFINITY;
    for n in numbers {
        min = f32::min(min, n);
        max = f32::max(max, n);
    }
    (min as usize, max as usize)
}

enum Domain {
    Finite(u32),
    Infinite,
}

fn inc(d: &Domain) -> Domain {
    if let Domain::Finite(c) = d {
        Domain::Finite(c+1)
    } else { Domain::Infinite }
}

fn solve(d: &str) -> Result<u32, u32> {
    let pts: Vec<_> = d.lines().map(parse_point).collect();
    // find bound
    let x_bounds = min_max(pts.iter().map(|pt| pt[0]));
    let y_bounds = min_max(pts.iter().map(|pt| pt[1]));
    println!("Bounds are: x: {:?}, y: {:?}", x_bounds, y_bounds);

    // add input pts to kd-tree
    let mut pt_tree = kdtree::KdTree::new(2);
    for (i, pt) in pts.iter().enumerate() {
        pt_tree.add(pt, i).unwrap();
    }

    // *** part 1
    // iterate over full area, accumulatint domain sizes
    let mut sizes: Vec<_> = pts.iter().map(|_| Domain::Finite(0)).collect();
    for x in x_bounds.0..(x_bounds.1+1) {
        for y in y_bounds.0..(y_bounds.1+1) {
            let pt = [x as f32, y as f32];
            let at_bounds = x==x_bounds.0 || x==x_bounds.1 || y==y_bounds.0 || y==y_bounds.1;
            let nearest = pt_tree.nearest(&(pt[..]), 2, &manhattan_dist).unwrap();
            if nearest[0].0 == nearest[1].0 {
                // this pt is contested. do nothing
            } else {
                let index = *nearest[0].1;
                sizes[index] = if at_bounds {
                        Domain::Infinite
                    } else {
                        inc(&sizes[index])
                    }     
            }                
        }
    }

    // find max element in sizes vector
    let (k, d) = sizes.iter().enumerate()
        .filter_map(|(k, s)| if let Domain::Finite(d)=s {Some((k, d))} else {None})
        .max_by_key(|(_k, d)| *d).unwrap();
    println!("Largest finite domain is {} with {} closest neighbors", k, d);

    // *** part 2
    Ok(0)
}

fn get_input_path() -> io::Result<std::path::PathBuf> {
    let pwd = std::env::current_dir()?;
    let day_name = pwd.file_name().ok_or(io::Error::new(io::ErrorKind::Other, "No pwd name"))?;
    let parent = pwd.parent().ok_or(io::Error::new(io::ErrorKind::Other, "No parent folder"))?;
    let mut input = parent.join("inputs").join(day_name);
    input.set_extension("txt");
    Ok(input)
}

fn main() {
    let input_path = get_input_path().unwrap();
    println!("Reading input from {}", input_path.to_string_lossy());
    let data = std::fs::read_to_string(input_path).expect("Error reading input file");
    solve(&data).unwrap();
}
