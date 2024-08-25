pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut perimeter = 0;
    let coords = input
        .lines()
        .scan((0, 0), |acc, l| {
            let mut data = l.split_ascii_whitespace();
            let (dir, n) = (
                data.next().unwrap(),
                data.next().unwrap().parse::<i64>().unwrap(),
            );
            perimeter += n;
            match dir {
                "R" => *acc = (acc.0 + n, acc.1),
                "L" => *acc = (acc.0 - n, acc.1),
                "U" => *acc = (acc.0, acc.1 + n),
                "D" => *acc = (acc.0, acc.1 - n),
                e => panic!("unknown {:?}", e),
            };
            Some(*acc)
        })
        .collect::<Vec<_>>();

    picks_theorem(coords, perimeter)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut perimeter = 0;
    let coords = input
        .lines()
        .scan((0, 0), |acc, l| {
            let (n_str, dir_str) = l.split_once('#').unwrap().1.split_at(5);
            let n = i64::from_str_radix(n_str, 16).unwrap();
            perimeter += n;
            match dir_str.chars().next().unwrap() {
                '0' => *acc = (acc.0 + n, acc.1),
                '1' => *acc = (acc.0, acc.1 - n),
                '2' => *acc = (acc.0 - n, acc.1),
                '3' => *acc = (acc.0, acc.1 + n),
                e => panic!("unknown {:?}", e),
            };
            Some(*acc)
        })
        .collect::<Vec<_>>();

    picks_theorem(coords, perimeter)
}

fn picks_theorem(coords: Vec<(i64, i64)>, perimeter: i64) -> i64 {
    let area = (0..coords.len())
        .fold(0, |acc, i| {
            let (x1, y1) = coords[i];
            let (x2, y2) = coords[(i + 1) % coords.len()];
            acc + x1 * y2 - x2 * y1
        })
        .abs()
        / 2;

    area + perimeter / 2 + 1
}

#[cfg(test)]
mod day18 {

    use super::*;

    const INPUT: &'static str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "62");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "952408144115");
    }
}
