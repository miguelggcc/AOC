pub fn part1(input: &str) -> u32 {
    let digits: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().rev().map(|c| c.to_digit(2).unwrap()).collect())
        .collect();
    let n_len = digits[0].len();
    let half = digits.len() as u32 / 2;
    let gamma_rate = (0..n_len).fold(0, |acc, i| {
        acc | (((digits.iter().map(|d| d[i]).sum::<u32>() / half).min(1)) << i)
    });
    let mask = (1u32 << n_len) - 1;
    let epsilon_rate = (!gamma_rate) & mask;

    gamma_rate * epsilon_rate
}

pub fn part2(input: &str) -> u32 {
    let n_len = input.lines().next().unwrap().len();
    let digits: Vec<u32> = input
        .lines()
        .map(|l| {
            l.chars()
                .rev()
                .enumerate()
                .fold(0, |acc, (i, b)| acc | (b.to_digit(2).unwrap() << i))
        })
        .collect();

    let oxygen = get_rating(&digits, n_len, 1);
    let co2 = get_rating(&digits, n_len, 0);

    oxygen * co2
}

fn get_rating(digits: &[u32], n_len: usize, switch: u32) -> u32 {
    let mut digits = digits.to_vec();
    for i in (0..n_len).rev() {
        let max_bit = (digits.iter().map(|d| d >> i & 1).sum::<u32>()
            / ((1 + digits.len() as u32) / 2))
            ^ switch;
        digits.retain(|d| d >> i & 1 == max_bit);

        if digits.len() == 1 {
            break;
        }
    }
    digits[0]
}

#[cfg(test)]
mod day3 {

    use super::*;

    const INPUT: &'static str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 198);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 230);
    }
}
