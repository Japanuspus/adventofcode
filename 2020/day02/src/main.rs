use aoclib::{get_inputs_pwd, Result};
use parse_display::{FromStr};

#[derive(Debug, FromStr)]
#[display("{min}-{max} {c}: {pass}")]
struct PassRow {
    min: usize,
    max: usize,
    c: char,
    pass: String,
}

fn main() -> Result<()> {
    let input = get_inputs_pwd()?;
    let rows: Vec<PassRow> = input.lines().filter_map(|s| s.parse().ok()).collect();

    println!("Part 1: {}", rows.iter().filter(|r| {
        let n = r.pass.chars().filter(|c| *c==r.c).count();
        (r.min <= n) && (n <= r.max)
    }).count());

    println!("Part 2: {}", rows.into_iter().filter(|r| {
        (r.pass.chars().nth(r.min-1).unwrap()==r.c) != 
        (r.pass.chars().nth(r.max-1).unwrap()==r.c)
    }).count());

    Ok(())
}
