pub fn part1(input: &str) -> impl std::fmt::Display {
    input
        .split("\r\n\r\n")
        .flat_map(|s| s.split("\n\n"))
        .map(|pattern| {
            let (xs, ys) = parse_pattern(pattern);
            get_reflections(xs, 0) * 100 + get_reflections(ys, 0)
        })
        .sum::<usize>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    input
        .split("\r\n\r\n")
        .flat_map(|s| s.split("\n\n"))
        .map(|pattern| {
            let (xs, ys) = parse_pattern(pattern);
            get_reflections(xs, 1) * 100 + get_reflections(ys, 1)
        })
        .sum::<usize>()
}

fn parse_pattern(input: &str) -> (Vec<u32>, Vec<u32>) {
    let nx = input.lines().next().unwrap().chars().count();
    let xs: Vec<_> = input
        .lines()
        .map(|l| l.chars().fold(0, |acc, c| (acc << 1) + (c == '#') as u32))
        .collect();
    let ys: Vec<_> = (0..nx)
        .rev()
        .map(|i| xs.iter().fold(0, |acc, x| (acc << 1) + ((x >> i) & 1)))
        .collect();
    (xs, ys)
}

fn get_reflections(values: Vec<u32>, n_of_smudges: u32) -> usize {
    (1..values.len())
        .find(|&i| {
            values[i..]
                .iter()
                .zip(values[..i].iter().rev())
                .map(|(&x0, &x1)| (x0 ^ x1).count_ones())
                .sum::<u32>()
                == n_of_smudges
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod day13 {

    use super::*;

    const INPUT: &'static str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "405");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "400");
    }
}
