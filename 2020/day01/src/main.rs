use std::collections::BTreeSet;

use aoclib::{get_inputs_pwd, Result};

fn main() -> Result<()> {
    let inputs: String = get_inputs_pwd()?;
    let numbers: BTreeSet<usize> = inputs.lines().filter_map(|s| s.parse().ok()).collect();
    
    // part 1
    for n in numbers.iter() {
        if let Some(d) = 2020usize.checked_sub(*n) {
            if numbers.contains(&d) {
                println!("n: {}, d: {}, p: {}", n,d, n*d)
            }
        }
    }

    // part 2
    for p in numbers.iter().filter_map(|n1| 
        2020usize.checked_sub(*n1)
        .and_then(|d1| 
            numbers.iter().filter_map(|n2|
                d1.checked_sub(*n2).and_then(|d2|
                    if numbers.contains(&d2) {Some(n1*n2*d2)} else {None}
                )
            ).next()
        )
    ) {
        println!("{}", p)
    }

    Ok(())
}
