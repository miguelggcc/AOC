use super::intcode::IntCode;

pub fn part1(input: &str) -> String {
    let mut computer = IntCode::new(input);
    computer.execute_input(1);
    computer.output.into_iter().map(|n| n.to_string()).collect()
}

pub fn part2(input: &str) -> String {
    let mut computer = IntCode::new(input);
    computer.execute_input(5);
    computer.output.into_iter().map(|n| n.to_string()).collect()
}

#[cfg(test)]
mod day5 {

    use super::*;

    #[test]
    fn test_intcode() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let mut computer = IntCode::new(input);
        computer.execute_input(5);
        assert_eq!(
            computer
                .output
                .into_iter()
                .map(|n| n.to_string())
                .collect::<String>(),
            "999"
        );
    }
}
