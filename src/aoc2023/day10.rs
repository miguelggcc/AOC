use std::collections::VecDeque;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let (nx, ny, mut grid, start, first_move) = parse(input);

    let mut q = VecDeque::from([(start as isize, 0)]);
    let mut max = 0;
    while let Some((p, d)) = q.pop_front() {
        for new_pos in get_new_positions(p, &grid, nx, ny, &first_move) {
            max = max.max(d + 1);
            q.push_back((new_pos, d + 1));
        }
        grid[p as usize] = '.';
    }
    max
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let (nx, ny, gridi, start, first_move) = parse(input);
    let mut grid = gridi.clone();

    let mut q = VecDeque::from([start as isize]);
    let mut is_loop = vec![false; grid.len()];
    is_loop[start] = true;
    while let Some(p) = q.pop_front() {
        for new_pos in get_new_positions(p, &grid, nx, ny, &first_move) {
            is_loop[new_pos as usize] = true;
            q.push_back(new_pos);
        }
        grid[p as usize] = '.';
    }
    is_loop
        .iter()
        .enumerate()
        .filter(|(p, n)| !*n && is_in(*p, &is_loop, &gridi, 1 + nx as usize))
        .count()
}

const ALLOWED: [&str; 4] = ["|7F", "-J7", "|LJ", "-LF"];
const VERTICAL: [(isize, isize); 2] = [(0, -1), (0, 1)];
const HOR: [(isize, isize); 2] = [(-1, 0), (1, 0)];
const L: [(isize, isize); 2] = [(0, -1), (1, 0)];
const J: [(isize, isize); 2] = [(0, -1), (-1, 0)];
const SEVEN: [(isize, isize); 2] = [(0, 1), (-1, 0)];
const F: [(isize, isize); 2] = [(0, 1), (1, 0)];

fn get_new_positions<'a>(
    p: isize,
    grid: &'a [char],
    nx: isize,
    ny: isize,
    first_move: &'a [(isize, isize)],
) -> impl Iterator<Item = isize> + 'a {
    match grid[p as usize] {
        '|' => &VERTICAL,
        '-' => &HOR,
        'L' => &L,
        'J' => &J,
        '7' => &SEVEN,
        'F' => &F,
        'S' => first_move,
        _ => &[],
    }
    .iter()
    .filter_map(move |&(dx, dy)| {
        let (x, y) = (p % nx + dx, p / nx + dy);
        (x >= 0 && x < nx && y >= 0 && y < ny && grid[(x + y * nx) as usize] != '.')
            .then_some(x + y * nx)
    })
}

fn is_in(mut p: usize, is_loop: &[bool], grid: &[char], delta: usize) -> bool {
    let mut crosses = 0;
    while let Some(check_p) = is_loop.get(p) {
        if grid[p] == 'L' || grid[p] == '7' {
            p += delta;
            continue;
        }
        crosses += *check_p as u32;
        p += delta;
    }
    crosses % 2 != 0
}

fn parse(input: &str) -> (isize, isize, Vec<char>, usize, Vec<(isize, isize)>) {
    let nx = input.lines().next().unwrap().len() as isize;
    let grid: Vec<_> = input.lines().flat_map(|l| l.chars()).collect();
    let ny = grid.len() as isize / nx;
    let start = grid.iter().position(|p| *p == 'S').unwrap();
    let first_move: Vec<_> = [(0, -1), (1, 0), (0, 1), (-1, 0)]
        .into_iter()
        .zip(ALLOWED)
        .filter_map(|((dx, dy), a)| {
            let pos = start + (dx + dy * nx) as usize;
            grid.get(pos).filter(|&&c| a.contains(c)).map(|_| (dx, dy))
        })
        .collect();
    (nx, ny, grid, start, first_move)
}

#[cfg(test)]
mod day10 {

    use super::*;

    const INPUT1: &'static str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const INPUT2: &'static str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT1).to_string(), "8");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT2).to_string(), "10");
    }
}
