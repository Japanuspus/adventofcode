#![allow(unused)]

fn is_pin(x: isize) -> bool {
    let mut cl: Option<u8> = None;
    let mut double: bool = false;
    for c in x.to_string().as_bytes() {
        if let Some(last) = cl {
            if last == *c {double = true};
            if *c<last {return false};
        }
        cl = Some(*c);
    }
    return double
}

fn is_pin2(x: isize) -> bool {
    let mut cl: Option<u8> = None;
    let mut double: bool = false;
    let mut lastcount: isize = 0;

    for c in x.to_string().as_bytes() {
        if let Some(last) = cl {
            if last == *c {
                lastcount += 1;
            } else {
                double = double || lastcount == 1;
                lastcount = 0;
            };
            if *c<last {return false};
        }
        cl = Some(*c);
    }
    return double || lastcount == 1
}

fn main() {
    let mut input = std::fs::read_to_string("input.txt").expect("Error reading input.txt");
    let mut lines = input.lines();
    let mut bounds = lines.next().unwrap().split('-');
    let r0: isize=bounds.next().unwrap().parse().unwrap();
    let r1: isize=bounds.next().unwrap().parse().unwrap();

    let n1 = (r0..r1).filter(|x| is_pin(*x)).count();
    println!("Part 1: {}", n1);

    let n2 = (r0..r1).filter(|x| is_pin2(*x)).count();
    println!("Part 2: {}", n2);

}
