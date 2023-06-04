pub fn part1(input: &str) -> usize {
    enhance(input, 2)
}

pub fn part2(input: &str) -> usize {
    enhance(input, 50)
}

fn enhance(input: &str, steps: usize) -> usize {
    let mut lines = input.lines();
    let iea: Vec<_> = lines.next().unwrap().chars().map(|c| c == '#').collect();
    assert!(lines.next().unwrap().is_empty());
    let inside: Vec<_> = lines
        .map(|l| {
            let mut row = vec![false; steps];
            row.extend(l.chars().map(|c| c == '#'));
            row.extend(vec![false; steps]);
            row
        })
        .collect();
    let nx = inside[0].len() as isize;
    let mut grid = vec![vec![false; nx as usize]; steps];
    grid.extend(inside.into_iter().chain(grid.clone()));
    let ny = grid.len() as isize;
    let mut grid: Vec<_> = grid.into_iter().flatten().collect();
    let mut copy;

    for step in 0..steps as isize {
        copy = grid.clone();
        for y in 0..ny {
            for x in 0..nx {
                let index = DELTAS
                    .iter()
                    .map(move |(dx, dy)| (x as isize + dx, y as isize + dy))
                    .fold(0, |acc, (x2, y2)| {
                        acc << 1
                            | usize::from(
                                if x2 < 0 || x2 >= nx || y2 < 0 || y2 >= ny {
                                    iea[0] && step % 2 == 1
                                } else {
                                    copy[(x2 + nx * y2) as usize]
                                },
                            )
                    });
                grid[(x + nx * y) as usize] = iea[index];
            }
        }
    }
    grid.into_iter().filter(|&c| c).count()
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
