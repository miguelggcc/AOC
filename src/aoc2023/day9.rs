pub fn part1(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| {
            let seq = parse_seq(l);
            lagrange_extrapolation(seq.len() as i128, seq)
        })
        .sum::<i64>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| {
            let seq = parse_seq(l);
            lagrange_extrapolation(-1, seq)
        })
        .sum::<i64>()
}

fn lagrange_extrapolation(new_x: i128, seq: Vec<i64>) -> i64 {
    let k = seq.len() as i128;
    (0..k)
        .zip(seq)
        .map(|(j, y)| {
            let basis = (0..k)
                .filter(|m| *m != j)
                .fold((1, 1), |acc, m| (acc.0 * (new_x - m), acc.1 * (j - m)));
            (basis.0 / basis.1) as i64 * y
        })
        .sum::<i64>()
}

fn parse_seq(line: &str) -> Vec<i64> {
    line.split_ascii_whitespace()
        .map(|d| d.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod day9 {

    use super::*;

    const INPUT: &'static str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "114");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "2");
    }
}
