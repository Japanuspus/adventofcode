#![allow(unused)]

type V3 = Vec<isize>;

#[derive(Debug, Clone)]
struct Moon {
    r: V3,
    v: V3
}

fn abssum(v: &V3) -> isize {
    v.iter().map(|x| x.abs()).sum()
}

impl Moon {
    fn etot(&self) -> isize {
        abssum(&self.r)*abssum(&self.v)
    }
}


// <x=-1, y=-4, z=0>
fn parse_moon(s: &str) -> Moon {
    let p:Vec<_> = s.split(|c| c==',' || c=='=' || c=='>').collect();
    let mut xyz = Vec::new();
    xyz.push(p[1]);
    xyz.push(p[3]);
    xyz.push(p[5]);
    return Moon{r: xyz.iter().map(|n| n.parse().unwrap()).collect(), v: vec![0,0,0]}
}

fn acc1(d: &Moon, s: &Moon) -> V3 {
    d.r.iter()
    .zip(s.r.iter())
    .map(|(a,b)| if a>b {-1} else {if a<b {1} else {0}})
    .collect()
}

fn add(a: &V3, b: &V3) -> V3 {
    a.iter().zip(b.iter()).map(|(a,b)| a+b).collect()
}

fn acc(m: &Moon, src: &Vec<Moon>) -> V3 {
    src.iter()
    .map(|s| acc1(m,s))
    .fold(vec![0,0,0], |s, da| add(&s, &da))
}

fn next_pos(m: &Moon, acc: &V3) -> Moon {
    let v = add(&m.v, acc);
    let r = add(&m.r, &v);
    Moon{v, r}
}

fn step(ms: &Vec<Moon>) -> Vec<Moon> {
    ms.iter()
    .map(|m| next_pos(m, &acc(m, ms)))
    .collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
    .expect("Error reading input file");

    let moons: Vec<_> = input.lines().map(|s| parse_moon(s)).collect();
    let ms1000 = (0..1000).fold(moons.clone(), |ms,_| step(&ms));
    let energy:isize = ms1000.iter()
        .map(|m| m.etot())
        .sum();
    println!("Part 1: {}", energy);
} 