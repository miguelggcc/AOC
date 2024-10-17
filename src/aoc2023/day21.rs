use std::collections::VecDeque;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let nx = input.lines().next().unwrap().len() as isize;
    let grid: Vec<_> = input.lines().flat_map(|l| l.chars()).collect();
    let start = grid.iter().position(|&c| c == 'S').unwrap() as isize;
    let ny = grid.len() as isize / nx;
    let mut q = VecDeque::from([(start, 0)]);
    let n: isize = 6;
    let mut cache = vec![false; grid.len() * (n + 1) as usize];
    let mut total = 0;

    while let Some((i, steps)) = q.pop_front() {
        if cache[(i * (n + 1) + steps) as usize] {
            continue;
        }
        cache[(i * (n + 1) + steps) as usize] = true;
        if steps == n {
            total += 1;
            continue;
        }
        for new_i in neighbours(&grid, i, nx, ny) {
            q.push_back((new_i, steps + 1))
        }
    }
    total
}

pub fn part2(_input: &str) -> impl std::fmt::Display {
    "Not implemented"
}

const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn neighbours(grid: &[char], i: isize, nx: isize, ny: isize) -> impl Iterator<Item = isize> + '_ {
    DIRS.iter().filter_map(move |(dx, dy)| {
        let x = i % nx + dx;
        let y = i / nx + dy;
        if x >= 0 && x < nx && y >= 0 && y < ny && grid[(x + y * nx) as usize] != '#' {
            return Some(x + ny * y);
        }
        None
    })
}

#[cfg(test)]
mod day21 {

    use super::*;

    const INPUT: &'static str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "16");
    }
    #[test]
    #[ignore]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "");
    }
}
