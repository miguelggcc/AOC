use itertools::*;

pub fn part1(input: &str) -> u32 {
    let (grid, size) = parse(input);
    let los = get_los(size);
    (1..size.0)
        .cartesian_product(1..size.1)
        .filter(|&(x, y)| grid[(x + y * size.0) as usize])
        .map(|pos| {
            los.iter()
                .filter(|&&l| find_asteroid(&grid, l, pos, size).is_some())
                .count()
        })
        .max()
        .unwrap() as u32
}

pub fn part2(input: &str) -> u32 {
    let (mut grid, size) = parse(input);
    let los = get_los(size);
    let pos0 = (1..size.0)
        .cartesian_product(1..size.1)
        .filter(|&(x, y)| grid[(x + y * size.0) as usize])
        .map(|pos| {
            (
                los.iter()
                    .filter(|&&l| find_asteroid(&grid, l, pos, size).is_some())
                    .count(),
                pos,
            )
        })
        .max_by(|&a, &b| a.0.cmp(&b.0))
        .unwrap()
        .1;

    los.iter()
        .cycle()
        .filter_map(|&l| {
            find_asteroid(&grid, l, pos0, size).and_then(|(x_as, y_as)| {
                grid[(x_as + y_as * size.0) as usize] = false;
                Some(x_as * 100 + y_as)
            })
        })
        .nth(199)
        .expect("less than 200 asteroids") as u32
}

fn get_los((nx, ny): Point) -> Vec<Point> {
    let mut quadrant: Vec<_> = (1..nx)
        .cartesian_product((1..ny).rev())
        .filter(|&(x, y)| (gcd(x, y) == 1))
        .collect();
    quadrant.sort_by_key(|&(x, y)| ((x as f32).atan2(y as f32) * 100000.0) as i32);
    let mut los: Vec<_> = vec![(0, -1)];
    los.extend(quadrant.iter().map(|&(x, y)| (x, -y)));
    los.push((1, 0));
    los.extend(quadrant.iter().copied().rev());
    los.push((0, 1));
    los.extend(quadrant.iter().map(|&(x, y)| (-x, y)));
    los.push((-1, 0));
    los.extend(quadrant.iter().rev().map(|&(x, y)| (-x, -y)));
    los
}

fn find_asteroid(grid: &[bool], l: Point, (mut x, mut y): Point, (nx, ny): Point) -> Option<Point> {
    loop {
        x += l.0;
        y += l.1;
        if x < 0 || x >= nx || y < 0 || y >= ny {
            return None;
        }
        if grid[(x + y * nx) as usize] {
            return Some((x, y));
        }
    }
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

type Point = (isize, isize);
fn parse(input: &str) -> (Vec<bool>, Point) {
    let nx = input.lines().next().unwrap().len();
    let grid: Vec<_> = input
        .lines()
        .flat_map(|l| l.chars().map(|c| c == '#'))
        .collect();
    let ny = grid.len() / nx;
    (grid, (nx as isize, ny as isize))
}

#[cfg(test)]
mod day10 {

    use super::*;

    const INPUT: &'static str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 210);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 802);
    }
}
