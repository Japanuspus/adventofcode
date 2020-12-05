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

    let dirs: Vec<(usize, usize)> = vec![(1, 1),(3, 1),(5, 1),(7, 1),(1, 2), ];
    println!("part 2: {}", 
        dirs.iter().map(|(right, down)| 
            rows.iter().step_by(*down).enumerate().filter(|(i, r)| r[(i*right) % r.len()]).count()
        ).product::<usize>()
    );

    Ok(())
}
