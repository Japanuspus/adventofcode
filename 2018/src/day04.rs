use std::collections::HashMap;
//use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::iter::Iterator;

//use nom::IResult;
//use nom::Err;
// nom IResult has been siginificantly changed for 4.1
// http://unhandledexpression.com/general/2018/05/14/nom-4-0-faster-safer-simpler-parsers.html
// pub type IResult<I, O, E = u32> = Result<(I, O), Err<I, E>>;
//use std::str::from_utf8;

#[derive(Debug, PartialEq)]
pub enum LogTypes {
    Sleep,
    Wake,
    Guard(u32),
}

#[derive(Debug, PartialEq)]
pub struct LogLine {
    pub ltime: u8,
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
    assert_eq!(parse_datetime("1518-11-09 23:58"), Ok(("", 58)));
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
    assert_eq!(
        parse_msg("Guard #2887 begins shift"),
        Ok(("", LogTypes::Guard(2887)))
    );
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
        Ok((
            "",
            LogLine {
                ltime: 58,
                ltype: LogTypes::Guard(853)
            }
        ))
    );
}

#[derive(Debug, PartialEq)]
pub struct Period {
    pub guard: u32,
    pub period: (u8, u8),
}

pub fn parse_periods(d: &str) -> Vec<Period> {
    let mut lsorted: Vec<_> = d.lines().collect();
    lsorted.sort();
    let loglines: Vec<_> = lsorted
        .iter()
        .map(|l| parse_logline(l).unwrap().1)
        .collect();

    let mut guard: u32 = 0;
    let mut r: &[LogLine] = &loglines;
    let mut periods: Vec<Period> = Vec::new();
    while r.len() > 0 {
        match r[0].ltype {
            LogTypes::Guard(g) => {
                guard = g;
                r = &r[1..];
            }
            _ => {
                periods.push(Period {
                    guard,
                    period: (r[0].ltime, r[1].ltime),
                });
                r = &r[2..];
            }
        }
    }
    periods
}

// Interval iterator: return (t, <active intervals indices>) at interesting values of t
#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
pub enum EdgeType {
    Edge0,
    Edge1,
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
pub struct IntervalEdge<T>
where
    T: Ord,
{
    time: T,
    edge_type: EdgeType,
    index: usize,
}

#[derive(Debug)]
pub struct IntervalSet<T>
where
    T: Ord,
{
    edges: Box<Vec<IntervalEdge<T>>>, //edges of intervals sorted by time then type
    // below is iterator state -- could be moved to an IntervalSetIterator class...
    // research into_iterator 
    active: Box<BTreeSet<usize>>, // indices of active intervals
    edge_index: usize, //index of next edge
}

impl<T: Ord + Clone> IntervalSet<T> {
    fn new(v: &[(T, T)]) -> IntervalSet<T> {
        let mut edges: Vec<IntervalEdge<T>> = Vec::new();
        for (index, (t0, t1)) in v.iter().enumerate() {
            edges.push(IntervalEdge::<T> {
                time: t0.clone(),
                edge_type: EdgeType::Edge0,
                index,
            });
            edges.push(IntervalEdge::<T> {
                time: t1.clone(),
                edge_type: EdgeType::Edge1,
                index,
            });
        }
        edges.sort();
        IntervalSet {
            edges: Box::new(edges),
            active: Box::new(BTreeSet::new()),
            edge_index: 0,
        }
    }

    fn step(&mut self) {
        self.edge_index += match self.edges.get(self.edge_index) {
            Some(IntervalEdge::<T> {
                time: _,
                edge_type: EdgeType::Edge0,
                index: idx,
            }) => {(*self.active).insert(*idx); 1},
            Some(IntervalEdge::<T> {
                time: _,
                edge_type: EdgeType::Edge1,
                index: idx,
            }) => {(*self.active).remove(&idx); 1},
            None => 0
        };
    }

    pub fn peek_t(&self) -> Option<T> {
        self.edges.get(self.edge_index).and_then(|ee| Some(ee.time.clone()))
    }
}

impl<T: Ord+Clone> Iterator for IntervalSet<T> {
    type Item = (T, usize);
    fn next(&mut self) -> Option<(T, usize)> {
        // take all entries with same time entry
        self.peek_t().and_then(|t| {
            while self.peek_t() == Some(t.clone()) {
                self.step();
            };
            Some((t, self.active.len()))
        })
    }
}

#[test]
fn test_intervalset() {
    // 0 1 2 3 4 5 6 7 8 9 10 11 12
    //       < <
    //   <       <
    //     <       <
    //     <   <
    //                 <         <
    let mut intset = IntervalSet::<u8>::new(&[(3, 4), (1, 5), (2, 6), (2, 4), (8, 12)]);
    assert_eq!(intset.next(), Some((1, 1)));
    assert_eq!(intset.next(), Some((2, 3)));
    assert_eq!(intset.next(), Some((3, 4)));
    assert_eq!(intset.next(), Some((4, 2)));
    assert_eq!(intset.next(), Some((5, 1)));
    assert_eq!(intset.next(), Some((6, 0)));
    assert_eq!(intset.next(), Some((8, 1)));
    assert_eq!(intset.next(), Some((12, 0)));
    assert_eq!(intset.next(), None);
}

fn interval_sum(ps: &[(u8, u8)]) -> u32 {
    ps.iter().map(|p| (p.1 - p.0) as u32).sum()
}

fn get_periods_by_guard(d: &str) -> HashMap<u32, Vec<(u8, u8)>> {
    let periods = parse_periods(d);

    // collect all sleep periods for a given guard
    let mut by_guard: HashMap<u32, Vec<(u8, u8)>> = HashMap::new();
    for p in periods {
        let entry = by_guard.entry(p.guard).or_insert(Vec::new());
        (*entry).push(p.period);
    }
    //println!("{:?}", by_guard);
    by_guard
}

fn sleepy_minute(v: &Vec<(u8,u8)>) -> (Option<u8>, usize) {
    match IntervalSet::<u8>::new(v).max_by_key(|(_, count)| *count) {
        Some((m, c)) => (Some(m), c),
        None => (None, 0),
    }
}


pub fn part1_01(d: &str) -> i64 {
    let by_guard = get_periods_by_guard(d);

    // find most sleeping guard
    let guard: u32 = *by_guard
        .iter()
        .map(|(g, ps)| (g, interval_sum(&ps)))
        .max_by_key(|(_, total_sleep)| *total_sleep)
        .unwrap()
        .0;

    println!("Most sleeping: {}", guard);
    // find most sleepy minute. Specification: [i1, i2[
    let minute = sleepy_minute(by_guard.get(&guard).unwrap()).0.unwrap();
    println!("Drowsiest minute: {}", minute);

    guard as i64 * minute as i64
}

pub fn part2_01(d: &str) -> i64 {
    let by_guard = get_periods_by_guard(d);

    // find guard with most sleepy minute
    let rr =  by_guard
        .iter()
        .map(|(g, ps)| (g, sleepy_minute(ps)))
        .max_by_key(|(_g, (_m, ct))| *ct)
        .unwrap();

    println!("Most sleeping stats: {:?}", rr);
    *rr.0 as i64 * (rr.1).0.unwrap() as i64
}

pub fn run(data: &str) {
    println!("Part 1: {}", part1_01(&data));
    println!("Part 2: {}", part2_01(&data));
}
