use std::collections::VecDeque;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let grid = Grid::parse(input, 1);
    calculate_steps(&grid, 64)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let grid = Grid::parse(input, 5);
    let seq = (0..3)
        .map(|i| calculate_steps(&grid, 131 * i + 65) as i64)
        .collect();
    lagrange_extrapolation((26501365 - 65) / 131, seq)
}

const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn neighbours(grid: &Grid, i: isize) -> impl Iterator<Item = isize> + '_ {
    DIRS.iter().filter_map(move |(dx, dy)| {
        let x = i % grid.n + dx;
        let y = i / grid.n + dy;
        if x >= 0 && x < grid.n && y >= 0 && y < grid.n && grid.get(x, y) != '#' {
            return Some(x + grid.n * y);
        }
        None
    })
}

fn calculate_steps(grid: &Grid, total_steps: isize) -> usize {
    let mut q = VecDeque::from([(grid.start, 0)]);
    let mut cache = vec![false; grid.grid.len()];
    let mut total = 0;

    while let Some((i, steps)) = q.pop_front() {
        if cache[i as usize] {
            continue;
        }
        total += (steps & 1 == total_steps & 1) as usize; //(https://t.ly/fpVfX)
        cache[i as usize] = true;
        if steps == total_steps {
            continue;
        }
        for new_i in neighbours(&grid, i) {
            q.push_back((new_i, steps + 1))
        }
    }
    total
}

struct Grid {
    grid: Vec<char>,
    n: isize, //grid is a square with side n
    start: isize,
}

impl Grid {
    fn parse(input: &str, times: isize) -> Self {
        let n = input.lines().next().unwrap().len() as isize;
        let grid: Vec<char> = input
            .lines()
            .flat_map(|l| std::iter::repeat(l.chars()).take(times as usize).flatten())
            .collect();
        let start = (grid.iter().position(|&c| c == 'S').unwrap()) as isize
            + times as isize / 2 * n
            + times as isize / 2 * n * n * times; //has to be in the center
        Self {
            grid: grid.repeat(times as usize),
            n: n * times,
            start,
        }
    }
    fn get(&self, x: isize, y: isize) -> char {
        self.grid[(x + y * self.n) as usize]
    }
}

fn lagrange_extrapolation(new_x: i64, seq: Vec<i64>) -> i64 {
    let k = seq.len() as i64;
    (0..k)
        .zip(seq)
        .map(|(j, y)| {
            let basis = (0..k)
                .filter(|m| *m != j)
                .fold((1, 1), |acc, m| (acc.0 * (new_x - m), acc.1 * (j - m)));
            (basis.0 / basis.1) * y
        })
        .sum::<i64>()
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
        let grid = Grid::parse(INPUT, 1);
        assert_eq!(calculate_steps(&grid, 6).to_string(), "16");
    }
}
