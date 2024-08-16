use std::collections::HashMap;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut grid = Grid::parse(input);
    grid.rotate_clockwise();
    grid.tilt();
    grid.load()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut grid = Grid::parse(input);
    let mut cache = HashMap::new();
    let mut key = 0;
    let mut loads = Vec::with_capacity(200);
    grid.rotate_clockwise();

    let (cycles, cycles0) = (0..200)
        .find_map(|i| {
            for _ in 0..4 {
                grid.tilt();
                grid.rotate_clockwise();
            }
            let load = grid.load() as u64;
            key = (key << 16) + load;
            if let Some(i0) = cache.get(&key) {
                return Some((i, *i0));
            }
            cache.insert(key, i);
            loads.push(load);
            None
        })
        .unwrap();
    loads[cycles0 + (1000000000 - cycles0) % (cycles - cycles0) - 1]
}

struct Grid {
    grid: Vec<u8>,
    n: usize, //always a square
}

impl Grid {
    fn parse(input: &str) -> Self {
        let n = input.lines().next().unwrap().len();
        let grid: Vec<_> = input
            .lines()
            .flat_map(|l| {
                l.chars().map(|c| match c {
                    'O' => 2,
                    '#' => 1,
                    _ => 0,
                })
            })
            .collect();
        Self { grid, n }
    }
    fn load(&self) -> usize {
        self.grid
            .chunks(self.n)
            .flat_map(|s| {
                s.iter()
                    .enumerate()
                    .filter(|(_, &c)| c == 2)
                    .map(|(i, _)| 1 + i)
            })
            .sum()
    }
    fn rotate_clockwise(&mut self) {
        for i in 0..self.n - 1 {
            for j in i + 1..self.n {
                self.grid.swap(i + j * self.n, j + i * self.n)
            }
        }
        self.grid.chunks_mut(self.n).for_each(|s| s.reverse());
    }

    fn tilt(&mut self) {
        self.grid
            .chunks_mut(self.n)
            .for_each(|row| row.split_mut(|&c| c == 1).for_each(|s| s.sort()))
    }
}

#[cfg(test)]
mod day14 {

    use super::*;

    const INPUT: &'static str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "136");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "64");
    }
}
