#![allow(unused)]

fn recursive_fuel(m: i64) -> i64 {
    let r = m/3-2;
    if r>0 {r+recursive_fuel(r)} else {0}
}

fn main() {
    let input:Vec<i64> = std::fs::read_to_string("input.txt")
        .expect("Error reading input file")
        .lines().filter_map(|s| s.parse().ok()).collect();

    let fuel_req: i64 = input.iter()
        .map(|m| m/3-2).sum();
    dbg!(&fuel_req);

    let fuel_req_rec: i64 = input.iter()
        .map(|m| recursive_fuel(*m)).sum();
    dbg!(&fuel_req_rec);
}
