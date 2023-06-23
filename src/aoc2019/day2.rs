use super::intcode::IntCode;
use itertools::*;

pub fn part1(input: &str) -> u32 {
    let mut computer = IntCode::new(input);
    computer.p[1] = 12;
    computer.p[2] = 2;

    computer.execute();
    computer.p[0] as u32
}

pub fn part2(input: &str) -> u32 {
    let output = 19690720;
    let computer = IntCode::new(input);

    let (noun, verb) = (0..100)
        .cartesian_product(0..100)
        .find(|&(noun, verb)| {
            let mut copy = computer.clone();
            copy.p[1] = noun;
            copy.p[2] = verb;
            copy.execute();
            copy.p[0] == output
        })
        .expect("noun and verb not found");
    (noun * 100 + verb) as u32
}
