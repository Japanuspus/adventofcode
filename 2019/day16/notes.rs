fn print_vec<'a>(w: impl Iterator<Item=&'a isize>) {
    println!("{}", w
        .map(|v| format!("{:2}", v))
        .collect::<Vec<String>>().join(", "));
}

fn main() {
    let n = 18;
    for r in 1..=n {
        print_vec(
            [0,1,0,-1]
            .iter()
            .cycle()
            .flat_map(|k| std::iter::repeat(k).take(r))
            .skip(1)
            .take(n));
    }
    
    let input = "12345678";
    let digits: Vec<isize> = input
        .chars()
        .filter(|c| char::is_digit(*c,10))
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    let val = digits.iter().take(3).fold(0, |acc, d| acc*10+d);
    dbg!(&val);
    let rdigits: Vec<isize> = digits.iter().rev().cloned().collect();
    dbg!(&rdigits);
    let csum: Vec<isize> = rdigits.iter().scan(0, |acc, d| {*acc = (*acc + d) % 10; Some(*acc)}).collect();
    dbg!(&csum);
}