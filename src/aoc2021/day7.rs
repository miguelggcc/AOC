pub fn part1(input: &str) -> i32 {
    let mut crabs: Vec<_> = input
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
    crabs.sort();

    let median = crabs[crabs.len() / 2];
    crabs.iter().map(|c| (c - median).abs()).sum()
}

pub fn part2(input: &str) -> i32 {
    let crabs: Vec<_> = input
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
    let (min, max) = crabs.iter().fold((i32::MAX, i32::MIN), |(min, max), &c| {
        (min.min(c), max.max(c))
    });

    (min..max)
        .map(|align| {
            crabs
                .iter()
                .map(|c| {
                    let d = (c - align).abs();
                    d * (d + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod day7 {

    use super::*;

    const INPUT: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 37);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 168);
    }
}
