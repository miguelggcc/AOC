pub fn part1(input: &str) -> u32 {
    let (mut octopuses, nx, ny) = parse(input);
    let mut flashes = 0;

    for _ in 0..100 {
        flashes += step(&mut octopuses, nx, ny);
    }
    flashes
}

pub fn part2(input: &str) -> u32 {
    let (mut octopuses, nx, ny) = parse(input);

    for s in 1..10000 {
        if step(&mut octopuses, nx, ny) == octopuses.len() as u32 {
            return s;
        }
    }
    panic!("synchronization not found")
}

fn step(octopuses: &mut Vec<u32>, nx: isize, ny: isize) -> u32 {
    let mut flashes = 0;
    let mut stack = Vec::from_iter(octopuses.iter_mut().enumerate().flat_map(
        |(index, octopus)| {
            *octopus += 1;
            if *octopus == 10 {
                *octopus = 0;
                Some(index)
            } else {
                None
            }
        },
    ));
    flashes += stack.len() as u32;
    while let Some(index) = stack.pop() {
        for nindex in neighbours(index as isize, nx, ny) {
            let octopus = octopuses.get_mut(nindex).unwrap();
            if *octopus > 0 {
                *octopus += 1;
                if *octopus == 10 {
                    flashes += 1;
                    *octopus = 0;
                    stack.push(nindex);
                }
            }
        }
    }
    flashes
}

fn parse(input: &str) -> (Vec<u32>, isize, isize) {
    let nx = input.lines().next().unwrap().len() as isize;
    let octopuses: Vec<_> = input
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();
    let ny = octopuses.len() as isize / nx;
    (octopuses, nx, ny)
}

const DELTAS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn neighbours(index: isize, nx: isize, ny: isize) -> impl Iterator<Item = usize> {
    let x = index % nx;
    let y = index / nx;
    DELTAS
        .into_iter()
        .filter(move |(dx, dy)| x + dx >= 0 && y + dy >= 0 && x + dx < nx && y + dy < ny)
        .map(move |(dx, dy)| (index + dx + dy * nx) as usize)
}

#[cfg(test)]
mod day11 {

    use super::*;

    const INPUT: &'static str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 1656);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 195);
    }
}
