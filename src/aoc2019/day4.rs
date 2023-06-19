use itertools::*;

pub fn part1(input: &str) -> usize {
    let (start, end) = parse(input);

    (start..=end)
        .filter(|n| {
            let digits = DIGITS.map(|d| (n / d) % 10);
            digits.iter().tuple_windows().all(|(d0, d1)| d1 >= d0)
                && digits.iter().tuple_windows().any(|(d0, d1)| d1 == d0)
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    let (start, end) = parse(input);

    (start..=end)
        .filter(|n| {
            let digits = DIGITS.map(|d| (n / d) % 10);
            digits.iter().tuple_windows().all(|(d0, d1)| d1 >= d0) && repetitions(digits)
        })
        .count()
}

const DIGITS: [u32; 6] = [100000, 10000, 1000, 100, 10, 1];

fn repetitions(digits: [u32; 6]) -> bool {
    let mut i = digits.iter();
    while let Some(d) = i.next() {
        if i.take_while_ref(|&nd| nd == d).count() == 1 {
            return true;
        }
    }
    false
}

fn parse(input: &str) -> (u32, u32) {
    input
        .split('-')
        .map(|n| n.parse::<u32>().unwrap())
        .collect_tuple()
        .unwrap()
}
