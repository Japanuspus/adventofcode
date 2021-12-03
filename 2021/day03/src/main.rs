use anyhow::Result;
use std::fs;

fn bit_pack(v: &[bool]) -> usize {
    v.iter().fold(0, |s, &c| (s<<1)+if c {1} else {0})
}

fn bit_rate(v: &Vec<Vec<bool>>, tgt: bool) -> &Vec<bool> {
    let mut buf: Vec<&Vec<bool>> = v.iter().collect();
    let mut idx = 0usize;
    while buf.len()>1 {
        let count: isize = buf.iter().filter(|r| r[idx]==tgt).count() as isize;
        let rem = (buf.len() as isize) - count;
        let mark = if count == rem {tgt} else {count>rem};
        buf = buf.into_iter().filter(|r| r[idx]==mark).collect();
        idx+=1;
    }
    buf[0]
}

fn main() -> Result<()> {
    let input_s = fs::read_to_string("input.txt")?;
    let input: Vec<Vec<bool>> = input_s.trim()
        .split("\n")
        .map(|s| 
            s.chars().map(|c| match c {
                '0' => false, '1' => true, _ => panic!("Bad char")
            }).collect()
        )
        .collect();

    let n = input.len();
    let m = input[0].len();
    let input_t: Vec<Vec<bool>> = (0..m).map(|i| input.iter().map(|r| r[i]).collect::<Vec<_>>()).collect();

    let gamma_v: Vec<bool> = input_t.iter().map(|col| col.iter().filter(|v| **v).count() > n/2).collect();
    let gamma = bit_pack(&gamma_v);
    let epsilon = (1i64<<m)-1-(gamma as i64);
    //println!("Gamma_v: {:?}, {:b}, {:b}", gamma_v, &gamma, &epsilon);
    println!("Part 1: {}", (gamma as i64)*epsilon);

    println!("Part 2: {}", bit_pack(bit_rate(&input, true)) * bit_pack(bit_rate(&input, false)));
    Ok(())
}    