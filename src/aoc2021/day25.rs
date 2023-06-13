pub fn part1(input: &str) -> u32 {
    let (mut cucumbers, size) = parse(input);
    let mut copy = cucumbers.clone();
    for step in 1..10_000 {
        if !try_move(&mut cucumbers, &mut copy, size) {
            return step;
        }
    }
    panic!("they never stop moving")
}

pub fn part2(_: &str) -> String {
    String::from("No part 2")
}

fn try_move(
    grid: &mut Vec<Option<Cucumber>>,
    old_grid: &mut Vec<Option<Cucumber>>,
    (nx, ny): (usize, usize),
) -> bool {
    let mut moved = false;
    *old_grid = grid.clone();
    for y in 0..ny {
        for x in 0..nx {
            let pos = x + y * nx;
            if old_grid[pos] == Some(Cucumber::E) {
                let new_pos = (x + 1) % nx + y * nx;
                if old_grid[new_pos].is_none() {
                    moved = true;
                    grid.swap(pos, new_pos);
                }
            }
        }
    }
    *old_grid = grid.clone();
    for y in 0..ny {
        for x in 0..nx {
            let pos = x + y * nx;
            if old_grid[pos] == Some(Cucumber::S) {
                let new_pos = x + ((y + 1) % ny) * nx;
                if old_grid[new_pos].is_none() {
                    moved = true;
                    grid.swap(pos, new_pos);
                }
            }
        }
    }
    moved
}

#[derive(Debug, Clone, PartialEq)]
enum Cucumber {
    E,
    S,
}

fn parse(input: &str) -> (Vec<Option<Cucumber>>, (usize, usize)) {
    let nx = input.lines().next().unwrap().len();
    let grid: Vec<_> = input
        .lines()
        .flat_map(|l| {
            l.chars().map(|c| match c {
                'v' => Some(Cucumber::S),
                '>' => Some(Cucumber::E),
                _ => None,
            })
        })
        .collect();
    let ny = grid.len() / nx;
    (grid, (nx, ny))
}

#[cfg(test)]
mod day25 {

    use super::*;

    const INPUT: &'static str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 58);
    }
}
