use itertools::*;

pub fn part1(input: &str) -> usize {
    let (start, end) = input
        .split_once('-')
        .map(|(s, e)| (s.parse::<u32>().unwrap(), e.parse::<u32>().unwrap()))
        .unwrap();

    (start..=end)
        .filter(|n| {
            let digits = DIGITS.map(|d| (n / d) % 10);
            digits.iter().tuple_windows().all(|(d0, d1)| d1 >= d0)
                && digits.iter().tuple_windows().any(|(d0, d1)| d1 == d0)
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    let (start, end) = input
        .split_once('-')
        .map(|(s, e)| (s.parse::<u32>().unwrap(), e.parse::<u32>().unwrap()))
        .unwrap();

    (start..=end)
        .filter(|n| {
            let digits = DIGITS.map(|d| (n / d) % 10);
            digits.iter().tuple_windows().all(|(d0, d1)| d1 >= d0) && get_repetitions(digits)
        })
        .count()
}

const DIGITS: [u32; 6] = [100000, 10000, 1000, 100, 10, 1];

fn get_repetitions(digits: [u32; 6]) -> bool {
    let mut i = digits.iter();
    while let Some(d) = i.next() {
        let c = i.take_while_ref(|&nd| nd == d).count() + 1;
        if c == 2 {
            return true;
        }
    }
    false
}
