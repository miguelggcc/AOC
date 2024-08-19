use std::collections::{HashSet, VecDeque};

pub fn part1(input: &str) -> impl std::fmt::Display {
    let nx = input.lines().next().unwrap().len() as isize;
    let grid: Vec<_> = input.lines().flat_map(|l| l.chars()).collect();
    let ny = grid.len() as isize / nx;

    calculate_energized_cells(&grid, nx, ny, (-1, 0), 0, None)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let nx = input.lines().next().unwrap().len() as isize;
    let grid: Vec<_> = input.lines().flat_map(|l| l.chars()).collect();
    let ny = grid.len() as isize / nx;
    let mut out = HashSet::new();

    usize::max(
        (0..ny)
            .map(|y| {
                usize::max(
                    calculate_energized_cells(&grid, nx, ny, (-1, y), 0, Some(&mut out)),
                    calculate_energized_cells(&grid, nx, ny, (nx, y), 2, Some(&mut out)),
                )
            })
            .max()
            .unwrap(),
        (0..nx)
            .map(|x| {
                usize::max(
                    calculate_energized_cells(&grid, nx, ny, (x, -1), 3, Some(&mut out)),
                    calculate_energized_cells(&grid, nx, ny, (x, ny), 1, Some(&mut out)),
                )
            })
            .max()
            .unwrap(),
    )
}

const DIRS: [(isize, isize); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];
const BAR1: [usize; 4] = [1, 0, 3, 2];
const BAR2: [usize; 4] = [3, 2, 1, 0];

fn calculate_energized_cells(
    grid: &[char],
    nx: isize,
    ny: isize,
    start: (isize, isize),
    dir: usize,
    mut out: Option<&mut HashSet<(isize, isize)>>,
) -> usize {
    if out.as_ref().is_some_and(|out_set| out_set.contains(&start)) {
        // light entering from a place where it has exited before will never reach a higher number of energized cells (https://t.ly/15Wdr)
        return 0;
    }

    let mut energized = vec![0u8; grid.len()];
    let mut q = VecDeque::from([(start, dir)]);

    while let Some(((mut x, mut y), mut i_dir)) = q.pop_front() {
        loop {
            let (dx, dy) = DIRS[i_dir];
            (x, y) = (x + dx, y + dy);
            if x >= 0 && x < nx && y >= 0 && y < ny {
                let new_i = (x + y * nx) as usize;
                let e = energized.get_mut(new_i).unwrap();
                if (*e >> i_dir) & 1 != 0 {
                    break;
                }
                *e += 1 << i_dir;
                match (grid[new_i], (dx, dy)) {
                    ('/', _) => i_dir = BAR1[i_dir],
                    ('\\', _) => i_dir = BAR2[i_dir],
                    ('|', (_, 0)) => {
                        i_dir = 1;
                        q.push_back(((x, y), 3));
                    }
                    ('-', (0, _)) => {
                        i_dir = 0;
                        q.push_back(((x, y), 2));
                    }
                    _ => (),
                };
            } else {
                if let Some(out_set) = out.as_mut() {
                    out_set.insert((x, y));
                }

                break;
            }
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
