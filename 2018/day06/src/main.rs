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

fn min_max<T,N> (numbers: T) -> (N, N) 
where
    T: IntoIterator<Item=N>,
    N: std::cmp::PartialOrd + Copy
{ 
    let mut iter = numbers.into_iter();
    let first = iter.next().unwrap();
    let mut min = first;
    let mut max = first;
    for n in iter {
        if n > max {max = n;}
        if n < min {min = n;}
    }
    (min , max)
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

fn solve01(d: &str) -> Result<u32, u32> {
    let pts: Vec<_> = d.lines().map(parse_point).collect();
    // find bound
    let x_bounds = min_max(pts.iter().map(|pt| pt[0] as u32));
    let y_bounds = min_max(pts.iter().map(|pt| pt[1] as u32));
    println!("Bounds are: x: {:?}, y: {:?}", x_bounds, y_bounds);

    // add input pts to kd-tree
    let mut pt_tree = kdtree::KdTree::new(2);
    for (i, pt) in pts.iter().enumerate() {
        pt_tree.add(pt, i).unwrap();
    }

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

    Ok(*d as u32)
}

fn dsum(pt: &(i32, i32), pegs: &[(i32, i32)]) -> i32 {
    pegs.iter().map(|p| i32::abs(pt.0-p.0)+i32::abs(pt.1-p.1)).sum()
}

fn p2above(mut k: u32) -> u32 {
    let mut p = 1;
    loop {
        p <<= 1;
        k >>= 1;
        if k==0 {break p}
    } 
}

fn count_ok<T>(tl: &(i32, i32), s: u32, f: &T) -> u32 
where T: FnMut(&(i32, i32))->bool 
{
    8
}

struct WorkZone<S,U> {
    top_left: (S, S),
    size: U
}

fn solve02(d: &str, dmax: i32) -> Result<u32, u32> {
    let pts: Vec<_> = d.lines().map(parse_point).map(|l| (l[0] as i32, l[1] as i32)).collect();
    let adist = (dmax/pts.len() as i32)+1;
    println!("Parsed {} points, 1+dmax/n={}", pts.len(), adist);
    // find bound
    let x_bounds = min_max(pts.iter().map(|pt| pt.0));
    let y_bounds = min_max(pts.iter().map(|pt| pt.1));
    println!("Bounds are: x: {:?}, y: {:?}", x_bounds, y_bounds);
    let top_left = (x_bounds.0 - adist, y_bounds.0 - adist);
    let p2 = p2above((
            2*adist+[&x_bounds, &y_bounds].into_iter().map(|b| b.1-b.0).max().unwrap()
        ) as u32) as i32;
    println!("Top left: {:?}, p2: {:?}", top_left, p2);

    let okfun = |pt| dsum(&pt, &pts) <= dmax;

    // let ct = count_ok(&top_left, p2, &okfun);
    let mut workstack = vec![WorkZone{top_left: top_left.clone(), size: p2}];
    let mut ct = 0i32;
    let corners = [(0i32, 0i32), (0,1), (1,1), (1,0)];

    loop {
        if let Some(w)=workstack.pop() {
            let corner_count = corners.iter()
                .filter(|c| okfun((w.top_left.0+(w.size-1)*c.0, w.top_left.1+(w.size-1)*c.1)))
                .count();
            if corner_count==4 {
                ct+=w.size*w.size;
                continue
            }
            if corner_count==0 && w.size<adist{
                continue
            }
            if w.size==2 {
                ct+=corner_count as i32;
                continue
            }
            // recurse into four quadrants 
            let half=w.size/2;
            for c in corners.iter() {
                workstack.push(WorkZone{
                    top_left: (w.top_left.0+half*c.0, w.top_left.1+half*c.1),
                    size: half
                })
            }
        } else {
            break
        }
    }
    println!("Size is {}", ct);

    Ok(ct as u32)
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
    solve01(&data).unwrap();
    solve02(&data, 10000).unwrap();
}
