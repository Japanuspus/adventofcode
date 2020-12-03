use aoclib;

fn main() -> aoclib::Result<()> {
    let input: String = aoclib::get_inputs_pwd()?;
    let rows: Vec<Vec<bool>> = input.lines().map(|r| r.chars().map(|c| match c {
        '#' => true,
        '.' => false,
        _ => panic!("bad char: {}", c)
    }).collect()).collect();

    println!("part 1: {}", 
        rows.iter().enumerate().filter(|(i, r)| r[(i*3) % r.len()]).count()
    );  
    Ok(())
}
