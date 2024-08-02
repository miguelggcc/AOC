pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut winning = [false; 100];

    input
        .lines()
        .map(|l| {
            let (_, card) = l.split_once(':').unwrap();
            let (left, right) = card.split_once('|').unwrap();
            winning = [false; 100];
            left.split_ascii_whitespace()
                .for_each(|n| winning[n.parse::<usize>().unwrap()] = true);
            let exp = right
                .split_ascii_whitespace()
                .filter(|n| winning[n.parse::<usize>().unwrap()])
                .count() as u32;

            2_u32.pow(exp) >> 1
        })
        .sum::<u32>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut copies = [1; 256];
    let mut winning = [false; 100];

    input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let (_, card) = l.split_once(':').unwrap();
            let (left, right) = card.split_once('|').unwrap();
            winning = [false; 100];
            left.split_ascii_whitespace()
                .for_each(|n| winning[n.parse::<usize>().unwrap()] = true);
            let matching = right
                .split_ascii_whitespace()
                .filter(|n| winning[n.parse::<usize>().unwrap()])
                .count() as usize;
            let instances = copies[i];
            copies
                .iter_mut()
                .skip(i + 1)
                .take(matching)
                .for_each(|c| *c += instances);

            instances
        })
        .sum::<usize>()
}

#[cfg(test)]
mod day4 {

    use super::*;

    const INPUT: &'static str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "13");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "30");
    }
}
