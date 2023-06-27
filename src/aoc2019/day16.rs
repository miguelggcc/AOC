pub fn part1(input: &str) -> u32 {
    let mut signal: Vec<_> = input.bytes().map(|b| (b - b'0') as i32).collect();

    for _ in 0..100 {
        signal = (1..signal.len() + 1)
            .map(|digit| {
                (signal[digit - 1..]
                    .chunks(digit)
                    .step_by(4)
                    .flatten()
                    .sum::<i32>()
                    - signal[(3 * (digit) - 1).min(signal.len())..]
                        .chunks(digit)
                        .step_by(4)
                        .flatten()
                        .sum::<i32>())
                .abs()
                    % 10
            })
            .collect::<Vec<_>>()
    }

    signal
        .into_iter()
        .take(8)
        .fold(0, |acc, d| acc * 10 + d as u32)
}

pub fn part2(input: &str) -> u32 {
    let small_signal: Vec<_> = input.bytes().map(|b| (b - b'0') as u32).collect();
    let len = small_signal.len();
    let index = small_signal[..7].iter().fold(0, |acc, &d| acc * 10 + d) as usize;

    assert!(index >= len * 10_000 / 2);

    let mut signal = small_signal[index % len..].to_vec();
    signal.append(&mut small_signal.repeat((len * 10_000 - index) / len));

    for _ in 0..100 {
        let mut prev = *signal.last().unwrap();
        for d in (signal.iter_mut()).rev().skip(1) {
            *d = (*d + prev) % 10;
            prev = *d;
        }
    }

    signal.into_iter().take(8).fold(0, |acc, d| acc * 10 + d)
}

#[cfg(test)]
mod day16 {

    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(part1("80871224585914546619083218645595"), 24176176);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2("03036732577212944063491565474664"), 84462026);
    }
}
