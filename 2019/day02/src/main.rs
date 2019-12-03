#![allow(unused)]

fn eval_intcode(input: &Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut state = input.clone();
    state[1]=noun;
    state[2]=verb;
    let mut pc=0;
    loop {
        let x = state[pc];
        if x==99 {
            break;
        }
        let r = state[pc+3];
        state[r] = if x==1 {
            state[state[pc+1]] + state[state[pc+2]]
        } else if x==2 {
            state[state[pc+1]] * state[state[pc+2]]
        } else {
            panic!("Unexpected operand")
        };
        pc+=4;
    }
    state[0]
}

fn main() {
    let input:Vec<usize> = std::fs::read_to_string("input.txt")
        .expect("Error reading input file")
        .split(',').filter_map(|s| s.parse().ok()).collect();

    // part 1 (moved to eval intcode)
    println!("Part 1: {}", eval_intcode(&input, 12, 02));

    // part2
    let y = 19690720;
    for n in 0..99 {
        for v in 0..99 {
            if eval_intcode(&input, n, v)==y {
                println!("Part 2: {}", 100*n+v);
                break;
            }
        }
    }
}

