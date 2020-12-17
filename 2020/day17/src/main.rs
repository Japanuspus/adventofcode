use anyhow::Result;
use std::fs;

mod part1 {
    use itertools::Itertools;
    use std::collections::HashSet;
    use vecmath;

    type Coord = i16;
    type Vec3 = vecmath::Vector3<Coord>;

    fn directions() -> impl Iterator<Item=Vec3> {
        (0..3).map(|_| (-1..2)).multi_cartesian_product().filter(|v| v != &[0,0,0]).map(|v| [v[0], v[1], v[2]])
    }

    fn neighbors(v: &Vec3) -> impl Iterator<Item=Vec3> {
        let vc = v.clone();
        directions().map(move |d| vecmath::vec3_add(vc, d))
    }

    #[test]
    fn test_dirs() {
        assert_eq!(directions().count(), 3*3*3-1);
    }

    type Grid = HashSet<Vec3>;

    fn step1(t0: &Grid) -> Grid {
        let field: HashSet<Vec3> = t0.iter().flat_map(|v| neighbors(v)).collect();
        field.into_iter().filter_map(|cell| {
            let nb_count = neighbors(&cell).filter(|v| t0.contains(v)).count();
            if t0.contains(&cell) {
                if (nb_count>=2) & (nb_count<=3) {Some(cell)} else {None}
            } else {
                if nb_count==3 {Some(cell)} else {None}
            }
        }).collect::<HashSet<Vec3>>()
    }

    pub fn run(input: &str) {
        let t0: HashSet<Vec3> = input
            .lines()
            .enumerate()
            .flat_map(|(y, ln)| {
                ln.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(x, _)| [x as Coord, y as Coord, 0])
            })
            .collect();

        let mut g = t0;
        println!("After {}: {}", 0, g.len());
        for i in 1..7 {
            g = step1(&g);
            println!("After {}: {}", i, g.len());
        }
    }
}

mod part2 {
    use itertools::Itertools;
    use std::collections::HashSet;
    use vecmath;

    type Coord = i16;
    type Vec3 = vecmath::Vector4<Coord>;

    fn directions() -> impl Iterator<Item=Vec3> {
        (0..4).map(|_| (-1..2)).multi_cartesian_product().filter(|v| v != &[0,0,0]).map(|v| [v[0], v[1], v[2], v[3]])
    }

    fn neighbors(v: &Vec3) -> impl Iterator<Item=Vec3> {
        let vc = v.clone();
        directions().map(move |d| vecmath::vec4_add(vc, d))
    }

    type Grid = HashSet<Vec3>;

    fn step1(t0: &Grid) -> Grid {
        let field: HashSet<Vec3> = t0.iter().flat_map(|v| neighbors(v)).collect();
        field.into_iter().filter_map(|cell| {
            let nb_count = neighbors(&cell).filter(|v| t0.contains(v)).count();
            if t0.contains(&cell) {
                if (nb_count>=2) & (nb_count<=3) {Some(cell)} else {None}
            } else {
                if nb_count==3 {Some(cell)} else {None}
            }
        }).collect::<HashSet<Vec3>>()
    }

    pub fn run(input: &str) {
        let t0: HashSet<Vec3> = input
            .lines()
            .enumerate()
            .flat_map(|(y, ln)| {
                ln.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(x, _)| [x as Coord, y as Coord, 0, 0])
            })
            .collect();

        let mut g = t0;
        println!("After {}: {}", 0, g.len());
        for i in 1..7 {
            g = step1(&g);
            println!("After {}: {}", i, g.len());
        }
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    part1::run(&input);
    println!("\n Part 2");
    part2::run(&input);

    Ok(())
}