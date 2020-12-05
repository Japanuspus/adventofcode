use std::collections::BTreeSet;
use aoclib;

fn main() -> aoclib::Result<()> {
    let input = aoclib::get_inputs_pwd()?;
    let seats: Vec<u32> = input.lines().map(|ln| 
        ln.chars().rev().enumerate().map(|(i, c)| (1<<i)*match c {
            'F'|'L' => 0,
            'B'|'R' => 1,
            _ => panic!(),
        }).sum::<>()).collect();

    println!("Part 1: {}", seats.iter().max().unwrap_or(&0));

    let seatset: BTreeSet<_> = seats.iter().collect();
    println!("Part 2: {}", seats.iter().filter(|&s| 
            seatset.contains(&(s+2)) && !seatset.contains(&(s+1))).next().unwrap_or(&0));

    Ok(())
}
