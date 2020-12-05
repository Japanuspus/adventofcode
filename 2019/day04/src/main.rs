#![allow(unused)]
use std::iter;

fn run_lengths<'a, T>(a: &'a[T]) -> impl Iterator<Item=usize> + 'a
where
    T: std::cmp::PartialEq 
{
    a
    .windows(2)
    .enumerate()
    .filter_map(|(i, ab)| if ab[0]==ab[1] {None} else {Some(i as isize)})
    .chain(iter::once(a.len() as isize - 1)) //append edge at end of slice
    .scan(-1, |last_pos, pos| { //first edge is before slice
        let res = pos - *last_pos;
        *last_pos=pos; 
        Some(res as usize)
    })
}

fn main() {
    let mut input = std::fs::read_to_string("input.txt").expect("Error reading input.txt");
    let mut bounds = input.lines().next().unwrap().split('-');
    let r0: isize=bounds.next().unwrap().parse().unwrap();
    let r1: isize=bounds.next().unwrap().parse().unwrap();

    let increasing = (r0..r1).map(|x| x.to_string().into_bytes())
        .filter(|v| v.windows(2).all(|ab| ab[0] <= ab[1]));

    println!("Part 1: {}", increasing.clone()
        .filter(|v| run_lengths(v).any(|k| k>=2)).count());

    println!("Part 2: {}", increasing
        .filter(|v| run_lengths(v).any(|k| k==2)).count());
}
