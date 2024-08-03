pub fn part1(input: &str) -> impl std::fmt::Display {
    let inputs = input.split_once("\n").unwrap();
    let (time, distance) = (parse(inputs.0), parse(inputs.1));

    time.into_iter()
        .zip(distance)
        .map(|(t, d)| get_whole_numbers_parabole(t, d))
        .product::<u32>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let input = input.replace(" ", "");
    let inputs = input.split_once("\n").unwrap();
    let (t, d) = (parse(inputs.0)[0], parse(inputs.1)[0]);

    get_whole_numbers_parabole(t, d)
}

fn get_whole_numbers_parabole(t: f64, d: f64) -> u32 {
    let discriminant = (t * t - 4.0 * d).sqrt();
    let root1 = (t + discriminant) / 2.0;
    let root2 = (t - discriminant) / 2.0;

    1 + (root1 - 0.51).round() as u32 - (root2 + 0.51).round().ceil() as u32
}

fn parse(line: &str) -> Vec<f64> {
    line.split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|n| n.parse::<f64>().unwrap())
        .collect()
}

#[cfg(test)]
mod day6 {

    use super::*;

    const INPUT: &'static str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "288");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "71503");
    }
}
