use super::intcode::IntCode;

pub fn part1(input: &str) -> String {
    let mut computer = IntCode::new(input);
    computer.execute(vec![1]);
    computer.output.into_iter().map(|n| n.to_string()).collect()
}

pub fn part2(input: &str) -> String {
    let mut computer = IntCode::new(input);
    computer.execute(vec![2]);
    computer.output.into_iter().map(|n| n.to_string()).collect()
}

#[cfg(test)]
mod day9 {

    use super::*;

    #[test]
    fn part_1() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut computer = IntCode::new(input);
        let program = computer.p.iter().copied().collect::<Vec<_>>();
        computer.execute(vec![]);
        assert_eq!(computer.output, program);
    }
    #[test]
    fn part_1_2() {
        let input = "109,1,203,2,204,2,99";
        let mut computer = IntCode::new(input);
        computer.execute(vec![1]);
        assert_eq!(computer.output.pop().unwrap(), 1);
    }
}
