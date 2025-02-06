use std::collections::HashMap;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let (mut left, mut right) = input
        .lines()
        .fold((vec![], vec![]), |(mut l, mut r), line| {
            let mut ns = line.split("   ");
            l.push(ns.next().unwrap().parse::<u32>().unwrap());
            r.push(ns.next().unwrap().parse::<u32>().unwrap());
            (l, r)
        });
    left.sort();
    right.sort();
    left.iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u32>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let (mut left, mut right) = (vec![], HashMap::new());
    for line in input.lines() {
        let mut ns = line.split("   ");
        left.push(ns.next().unwrap().parse::<u32>().unwrap());
        *right
            .entry(ns.next().unwrap().parse::<u32>().unwrap())
            .or_insert(0) += 1;
    }
    left.iter()
        .map(|l| l * right.get(l).unwrap_or(&0))
        .sum::<u32>()
}

#[cfg(test)]
mod day1 {

    use super::*;

    const INPUT: &'static str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "11");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "31");
    }
}
