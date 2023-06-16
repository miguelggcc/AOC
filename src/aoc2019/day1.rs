use std::iter::successors;

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.parse::<u32>().unwrap() / 3 - 2)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let fuel = |m: &u32| (m / 3).checked_sub(2);
    input
        .lines()
        .map(|l| successors(fuel(&l.parse::<u32>().unwrap()), fuel).sum::<u32>())
        .sum()
}

#[cfg(test)]
mod day1 {

    use super::*;

    const INPUT: &'static str = "100756";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 33583);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 50346);
    }
}
