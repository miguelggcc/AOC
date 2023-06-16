use itertools::*;

pub fn part1(input: &str) -> u32 {
    let mut p: Vec<_> = input
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    p[1] = 12;
    p[2] = 2;
    
    execute(p)
}

pub fn part2(input: &str) -> u32 {
    let output = 19690720;
    let p: Vec<_> = input
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let (noun, verb) = (0..100)
        .cartesian_product(0..100)
        .find(|&(noun, verb)| {
            let mut p_new = p.clone();
            p_new[1] = noun;
            p_new[2] = verb;
            execute(p_new) == output
        })
        .expect("noun and verb not found");
    (noun * 100 + verb) as u32
}

fn execute(mut p: Vec<usize>) -> u32 {
    for i in (0..).step_by(4) {
        let in1 = p[i + 1];
        let in2 = p[i + 2];
        let out = p[i + 3];
        match p[i] {
            1 => p[out] = p[in1] + p[in2],
            2 => p[out] = p[in1] * p[in2],
            99 => return p[0] as u32,
            e => panic!("uknown command {e}"),
        };
    }
    panic!("program never halts")
}
