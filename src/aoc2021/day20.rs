use std::iter::once;

pub fn part1(input: &str) -> usize {
    let (iea, nx, ny, grid) = parse(input);
    enhance(iea, grid.into_iter().flatten().collect(), nx, ny, 2)
}

pub fn part2(input: &str) -> usize {
    let (iea, nx, ny, grid) = parse(input);
    enhance(iea, grid.into_iter().flatten().collect(), nx, ny, 50)
}

fn enhance(iea: Vec<bool>, mut grid: Vec<bool>, mut nx: isize, mut ny: isize, ss: usize) -> usize {
    let mut copy;
    for step in 0..ss {
        copy = grid.clone();
        nx += 2;
        ny += 2;
        grid.extend(once(false).cycle().take((2 * (nx + 2) + 2 * ny) as usize));
        for y in 0..ny {
            for x in 0..nx {
                let index =
                    DELTAS
                        .iter()
                        .map(move |(dx, dy)| (x + dx, y + dy))
                        .fold(0, |acc, (x2, y2)| {
                            acc << 1
                                | usize::from(if x2 < 1 || x2 >= nx - 1 || y2 < 1 || y2 >= ny - 1 {
                                    iea[0] && step % 2 == 1
                                } else {
                                    copy[(x2 - 1 + (nx - 2) * (y2 - 1)) as usize]
                                })
                        });
                grid[(x + nx * y) as usize] = iea[index];
            }
        }
    }
    grid.into_iter().filter(|&c| c).count()
}

fn parse(input: &str) -> (Vec<bool>, isize, isize, Vec<Vec<bool>>) {
    let mut lines = input.lines();
    let iea: Vec<_> = lines.next().unwrap().chars().map(|c| c == '#').collect();
    assert!(lines.next().unwrap().is_empty());
    let grid: Vec<Vec<_>> = lines
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();
    (iea, grid[0].len() as isize, grid.len() as isize, grid)
}

const DELTAS: [(isize, isize); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[cfg(test)]
mod day20 {

    use super::*;

    const INPUT: &'static str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 35);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 3351);
    }
}
