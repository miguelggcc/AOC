use super::intcode::IntCode;

pub fn part1(input: &str) -> String {
    let mut computer = IntCode::new(input);
    computer.execute(vec![1]);
    computer.output.into_iter().map(|n| n.to_string()).collect()
}

pub fn part2(input: &str) -> String {
    let mut computer = IntCode::new(input);
    computer.execute(vec![5]);
    computer.output.into_iter().map(|n| n.to_string()).collect()
}
