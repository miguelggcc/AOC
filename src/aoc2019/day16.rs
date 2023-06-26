pub fn part1(input: &str) -> u32 {
    let len = input.bytes().len();
    let mut signal: Vec<_> = input.bytes().map(|b| (b - b'0') as i32).collect();
    for _ in 0..100 {
        signal = (1..=len)
            .map(|i| {
                signal
                    .iter()
                    .zip(get_iterator(i))
                    .map(|(d, p)| d * p)
                    .sum::<i32>()
                    .abs()
                    % 10
            })
            .collect();
    }
    signal
        .into_iter()
        .take(8)
        .fold(0, |acc, d| acc * 10 + d as u32)
}

fn get_iterator(n: usize) -> impl Iterator<Item = i32> {
    BLUEPRINT
        .iter()
        .flat_map(move |&i| std::iter::repeat(i).take(n))
        .cycle()
        .skip(1)
}

const BLUEPRINT: [i32; 4] = [0, 1, 0, -1];

pub fn part2(input: &str) -> u32 {
    let small_signal: Vec<_> = input.bytes().map(|b| (b - b'0') as i32).collect();
    let len = small_signal.len();
    let output_index = small_signal
        .iter()
        .take(7)
        .fold(0, |acc, &d| acc * 10 + d as usize);
    assert!(output_index >= len * 10_000 / 2);
    let mut signal = small_signal[(output_index) % len..].to_vec();
    signal.append(&mut small_signal.repeat((len * 10_000 - output_index) / len));

    for _ in 0..100 {
        for i in (0..signal.len() - 1).rev() {
            signal[i] += signal[i + 1];
            signal[i] %= 10;
        }
    }

    signal
        .into_iter()
        .take(8)
        .fold(0, |acc, d| acc * 10 + d as u32)
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
