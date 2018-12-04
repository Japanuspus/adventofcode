//use nom::IResult;
//use std::str::from_utf8;

#[cfg(test)]
mod tests {
    use super::*;
    const TT:& str = "";
    #[test]
    fn part1() {
        assert_eq!(part1_01(TT),0);
    }
}

pub fn part1_01(_d: &str) -> i64{
    0
}

pub fn part2_01(_d: &str) -> i64 {
    0
}

#[derive(Debug, PartialEq)]
pub enum LogTypes {
  Sleep,
  Wake, 
  ID(u32),
}

#[derive(Debug,PartialEq)]
pub struct LogLine {
  pub ltime:  u8,
  pub ltype: LogTypes,
}

named!(parse_datetime<u8>,
    do_parse!(
        take!(14) >>
        mm: be_u8 >>
        mm
    )
);


named!(get_greeting<&str,&str>,
    alt!( tag!("hi") | tag!("bye"))
);

    // "[1518-11-09 23:58] Guard #853 begins shift";


#[test]
fn test_get_greeting() {
    assert_eq!(get_greeting("hi"), Ok(("","hi")));
}


pub fn run(data: &str) {
    println!("Part 1: {}", part1_01(&data));
    println!("Part 2: {}", part2_01(&data));
}