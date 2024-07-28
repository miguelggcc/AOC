use super::intcode::IntCode;

pub fn part1(input: &str) -> String {
    let mut ic = IntCode::new(input);
    loop {
        let stdin = std::io::stdin();
        let mut command = String::new();
        stdin.read_line(&mut command).unwrap();
        command = command.trim_end().to_owned();
        dbg!(&command);
        ic.execute_string(command);
        println!("{}", ic.get_output_ascii());
    }
}

pub fn part2(_input: &str) -> String {
    String::from("Not implemented")
}

#[cfg(test)]
mod day25 {

    use super::*;

    const INPUT: &'static str = "";

    #[test]
    #[ignore]
    fn part_1() {
        assert_eq!(part1(INPUT), "");
    }
    #[test]
    #[ignore]
    fn part_2() {
        assert_eq!(part2(INPUT), "");
    }
}
