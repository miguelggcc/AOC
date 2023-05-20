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
    let data = input
        .lines()
        .map(|d| d.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    data.windows(3)
        .zip(data.windows(3).skip(1))
        .filter(|(ds1, ds2)| ds2.iter().sum::<u32>() > ds1.iter().sum::<u32>())
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
