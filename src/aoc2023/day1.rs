pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            10 * l.chars().find_map(|c| c.to_digit(10)).unwrap()
                + l.chars().rev().find_map(|c| c.to_digit(10)).unwrap()
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            10 * ((0..l.len())
                .find_map(|i| LUT.iter().position(|d| l[i..].starts_with(d)))
                .unwrap() as u32
                % 9
                + 1)
                + (0..l.len())
                    .rev()
                    .find_map(|i| LUT.iter().position(|d| l[i..].starts_with(d)))
                    .unwrap() as u32
                    % 9
                + 1
        })
        .sum()
}

const LUT: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

#[cfg(test)]
mod day1 {

    use super::*;

    const INPUT1: &'static str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const INPUT2: &'static str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT1), 142);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT2), 281);
    }
}
