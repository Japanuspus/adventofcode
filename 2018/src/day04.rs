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

pub fn part2_01(_d: &str) -> i64 {
    0
}

#[derive(Debug, PartialEq)]
pub enum LogTypes {
  Sleep,
  Wake, 
  Guard(u32),
}

#[derive(Debug,PartialEq)]
pub struct LogLine {
  pub ltime:  u8,
  pub ltype: LogTypes,
}

// "1518-11-09 23:58" -> 58
fn parse_mm(input: &str) -> Result<u8, std::num::ParseIntError> {
  u8::from_str_radix(&input[14..], 10)
}
fn is_digit(c: char) -> bool {
  c.is_digit(10)
}
fn parse_dec(s: &str) -> Result<u32, std::num::ParseIntError> {
  u32::from_str_radix(&s, 10)
}

#[test]
fn test_parse_mm() {
    assert_eq!(parse_mm("1518-11-09 23:58"), Ok(58));
}

named!(parse_datetime<&str, u8>,
  map_res!(take!(16), parse_mm)
);
#[test]
fn test_parse_datetime() {
    assert_eq!(parse_datetime("1518-11-09 23:58"), Ok(("",58)));
}

named!(parse_msg<&str, LogTypes>,
  alt!(
    value!(LogTypes::Sleep, tag!("falls asleep")) |
    value!(LogTypes::Wake, tag!("wakes up")) |
    do_parse!(
      tag!("Guard #") >> 
      gid: map_res!(take_while!(is_digit), parse_dec) >> 
      tag!(" begins shift") >> 
      (LogTypes::Guard(gid))
    )
  )
);
#[test]
fn test_parse_msg() {
    assert_eq!(parse_msg("falls asleep"), Ok(("", LogTypes::Sleep)));
    assert_eq!(parse_msg("Guard #2887 begins shift"), Ok(("", LogTypes::Guard(2887))));
}

named!(parse_logline<&str, LogLine>,
  do_parse!(
    ltime: delimited!(char!('['), parse_datetime, char!(']')) >>
    tag!(" ") >>
    ltype: parse_msg >>

    (LogLine{ltime, ltype})
  )
);
#[test]
fn test_parse_logline() {
    assert_eq!(
      parse_logline("[1518-11-09 23:58] Guard #853 begins shift"),
      Ok(("",LogLine{ltime: 58, ltype: LogTypes::Guard(853)}))
    );
}

#[derive(Debug,PartialEq)]
pub struct Period {
  pub guard: u32,
  pub period: (u8, u8),
}

pub fn part1_01(d: &str) -> i64{
    let mut lsorted: Vec<_> = d.lines().collect();
    lsorted.sort();

    let mut guard_periods: HashMap<u32, Period> = HashMap::new();
    let mut gid: u32 = 0;
    let mut t0: u8 =0;


    for l in lsorted {
      println!("> {}", l);
    }

    0
}


pub fn run(data: &str) {
    println!("Part 1: {}", part1_01(&data));
    println!("Part 2: {}", part2_01(&data));
}