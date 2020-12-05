use aoclib;
use itertools::Itertools;

fn main() -> aoclib::Result<()> {
    let seats: Vec<u32> = aoclib::get_inputs_pwd()?
    .lines().map(|ln| 
        ln.chars().rev().enumerate().map(|(i, c)| (1<<i)*match c {
            'F'|'L' => 0,
            'B'|'R' => 1,
            _ => panic!(),
        }).sum()
    ).sorted().collect();

    println!("Part 1: {}", seats.iter().last().unwrap_or(&0));

    println!("Part 2: {}", 
        seats.windows(2)
        .filter_map(|s12| if s12[1]==s12[0]+1 {None} else {Some(s12[0]+1)})
        .next().unwrap_or(0));

    Ok(())
}
