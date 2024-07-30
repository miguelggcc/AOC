use std::collections::HashMap;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let (nx, ny, grid) = parse(input);
    let mut total = 0;
    let mut it = grid.clone().into_iter().enumerate();

    while let Some((i, c)) = it.next() {
        if let Some(mut n) = c.to_digit(10) {
            let mut is_part = find_symbols(&grid, i as isize, nx, ny);
            while let Some((i2, digit)) = it
                .next()
                .and_then(|(i2, c)| c.to_digit(10).map(|d| (i2, d)))
            {
                n = n * 10 + digit;
                is_part |= find_symbols(&grid, i2 as isize, nx, ny);
                if i2 as isize % nx >= nx - 1 {
                    break;
                }
            }
            total += n * is_part as u32;
        }
    }
    total
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let (nx, ny, grid) = parse(input);
    let mut gears = HashMap::new();
    let mut total = 0;
    let mut it = grid.clone().into_iter().enumerate();

    while let Some((i, c)) = it.next() {
        if let Some(mut n) = c.to_digit(10) {
            let mut gears_here = find_gears(&grid, i as isize, nx, ny).collect::<Vec<_>>();
            while let Some((i2, digit)) = it
                .next()
                .and_then(|(i2, c)| c.to_digit(10).map(|d| (i2, d)))
            {
                n = n * 10 + digit;
                gears_here.extend(find_gears(&grid, i2 as isize, nx, ny));
                if i2 as isize % nx >= nx - 1 {
                    break;
                }
            }
            gears_here.dedup();
            gears_here.into_iter().for_each(|ig| {
                if let std::collections::hash_map::Entry::Occupied(o) = gears.entry(ig) {
                    total += n * o.get();
                } else {
                    gears.insert(ig, n);
                }
            });
        }
    }
    total
}

fn parse(input: &str) -> (isize, isize, Vec<char>) {
    let nx = input.lines().next().unwrap().len() as isize;
    let grid: Vec<_> = input.lines().flat_map(|l| l.chars()).collect();
    (nx as isize, grid.len() as isize / nx, grid)
}

const MOVES: [(isize, isize); 8] = [
    (0, -1),
    (-1, 0),
    (1, 0),
    (0, 1),
    (1, -1),
    (-1, -1),
    (-1, 1),
    (1, 1),
];

fn find_symbols(grid: &[char], i: isize, nx: isize, ny: isize) -> bool {
    MOVES.iter().any(|(dx, dy)| {
        let x = i % nx + dx;
        let y = i / nx + dy;
        x >= 0
            && x < nx
            && y >= 0
            && y < ny
            && grid[(x + y * nx) as usize] != '.'
            && !grid[(x + y * nx) as usize].is_ascii_digit()
    })
}

fn find_gears(grid: &[char], i: isize, nx: isize, ny: isize) -> impl Iterator<Item = isize> + '_ {
    MOVES.iter().filter_map(move |(dx, dy)| {
        let x = i % nx + dx;
        let y = i / nx + dy;
        if x >= 0 && x < nx && y >= 0 && y < ny && grid[(x + y * nx) as usize] == '*' {
            return Some(x + ny * y);
        }
        None
    })
}

#[cfg(test)]
mod day3 {

    use super::*;

    const INPUT: &'static str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "4361");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "467835");
    }
}
