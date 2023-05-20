pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|d| d.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|ds| ds[1] > ds[0])
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|d| d.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .windows(4)
        .filter(|ds| ds[3] > ds[0])
        .count()
}

#[cfg(test)]
mod day1 {

    use super::*;

    const INPUT: &'static str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 7);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 5);
    }
}
