pub fn part1(input: &str) -> impl std::fmt::Display {
    get_total_distance(input, 2)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    get_total_distance(input, 1_000_000)
}

fn get_total_distance(input: &str, replace_with: usize) -> usize {
    let nx = input.lines().next().unwrap().len();
    let grid: Vec<_> = input.lines().flat_map(|l| l.chars()).collect();
    let ny = grid.len() / nx;

    let gaps_y: Vec<usize> = (0..ny)
        .scan(0, |acc, y| {
            if grid[y * nx..(y + 1) * nx].iter().all(|&c| c == '.') {
                *acc += 1;
            }

            Some(*acc)
        })
        .collect();
    let gaps_x: Vec<usize> = (0..nx)
        .scan(0, |acc, x| {
            if grid.iter().skip(x).step_by(nx).all(|&c| c == '.') {
                *acc += 1;
            }
            Some(*acc)
        })
        .collect();

    let galaxies: Vec<_> = grid
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == '#')
        .map(|(i, _)| {
            let (x, y) = (i % nx, i / nx);
            (
                x + gaps_x[x] * (replace_with - 1),
                y + gaps_y[y] * (replace_with - 1),
            )
        })
        .collect();

    galaxies[..galaxies.len() - 1]
        .iter()
        .enumerate()
        .flat_map(|(i, (x0, y0))| {
            galaxies[i + 1..]
                .iter()
                .map(|(x1, y1)| x1.abs_diff(*x0) + y1.abs_diff(*y0))
        })
        .sum()
}

#[cfg(test)]
mod day11 {

    use super::*;

    const INPUT: &'static str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "374");
    }
    #[test]
    fn part_2() {
        assert_eq!(get_total_distance(INPUT, 100).to_string(), "8410");
    }
}
