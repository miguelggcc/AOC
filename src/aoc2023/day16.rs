use std::collections::VecDeque;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let nx = input.lines().next().unwrap().len() as isize;
    let grid: Vec<_> = input.lines().flat_map(|l| l.chars()).collect();
    let ny = grid.len() as isize / nx;

    calculate_energized_cells(&grid, nx, ny, (-1, 0), 0)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let nx = input.lines().next().unwrap().len() as isize;
    let grid: Vec<_> = input.lines().flat_map(|l| l.chars()).collect();
    let ny = grid.len() as isize / nx;

    (0..ny)
        .map(|y| {
            usize::max(
                calculate_energized_cells(&grid, nx, ny, (-1, y), 0),
                calculate_energized_cells(&grid, nx, ny, (nx, y), 2),
            )
        })
        .chain((0..nx).map(|x| {
            usize::max(
                calculate_energized_cells(&grid, nx, ny, (x, -1), 3),
                calculate_energized_cells(&grid, nx, ny, (x, ny), 1),
            )
        }))
        .max()
        .unwrap()
}

const DIRS: [(isize, isize); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];
const BAR1: [usize; 4] = [1, 0, 3, 2];
const BAR2: [usize; 4] = [3, 2, 1, 0];

#[inline(always)]
fn calculate_energized_cells(
    grid: &[char],
    nx: isize,
    ny: isize,
    start: (isize, isize),
    dir: usize,
) -> usize {
    let mut energized = vec![0u8; grid.len()];

    let mut q = VecDeque::from([(start, dir)]);

    while let Some(((old_x, old_y), i_dir)) = q.pop_front() {
        let (dx, dy) = DIRS[i_dir];
        let (x, y) = (old_x + dx, old_y + dy);
        if x >= 0 && x < nx && y >= 0 && y < ny {
            let new_i = (x + y * nx) as usize;
            let e = energized.get_mut(new_i).unwrap();
            if (*e >> i_dir) & 1 != 0 {
                continue;
            }
            *e += 1 << i_dir;
            match (grid[new_i], (dx, dy)) {
                ('/', _) => q.push_back(((x, y), BAR1[i_dir])),
                ('\\', _) => q.push_back(((x, y), BAR2[i_dir])),
                ('|', (_, 0)) => {
                    q.push_back(((x, y), 1));
                    q.push_back(((x, y), 3));
                }
                ('-', (0, _)) => {
                    q.push_back(((x, y), 0));
                    q.push_back(((x, y), 2));
                }
                _ => q.push_back(((x, y), i_dir)),
            };
        }
    }
    energized.into_iter().filter(|&e| e != 0).count()
}

#[cfg(test)]
mod day16 {

    use super::*;

    const INPUT: &'static str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "46");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "51");
    }
}
