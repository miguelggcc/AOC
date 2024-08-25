use std::{cmp::Ordering, collections::BinaryHeap};

pub fn part1(input: &str) -> impl std::fmt::Display {
    dijkstra(Grid::parse(input), 1, 3)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    dijkstra(Grid::parse(input), 4, 10)
}

const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const ROTATIONS: [[usize; 2]; 4] = [[1, 3], [2, 0], [1, 3], [2, 0]];

fn dijkstra(grid: Grid, min: usize, max: usize) -> u32 {
    let mut distances = vec![u32::MAX; grid.data.len() * 2];
    let start = [State { i: 0, c: 0, dir: 1 }, State { i: 0, c: 0, dir: 2 }];
    let mut q = BinaryHeap::from(start);

    while let Some(state) = q.pop() {
        if state.i == grid.nx * grid.ny - 1 {
            return state.c;
        }
        if state.c > distances[state.to_key()] {
            continue;
        }
        for new_dir in ROTATIONS[state.dir] {
            let mut new_state = State {
                i: state.i,
                c: state.c,
                dir: new_dir,
            };
            for repeated in 1..=max {
                if let Some((cost, new_i)) = grid.get_checked(new_state.i, DIRS[new_dir]) {
                    new_state.c += cost;
                    new_state.i = new_i;
                    if repeated >= min && distances[new_state.to_key()] > new_state.c {
                        distances[new_state.to_key()] = new_state.c;
                        q.push(new_state.clone())
                    }
                } else {
                    break;
                }
            }
        }
    }
    panic!("path not found")
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct State {
    i: isize,
    c: u32,
    dir: usize,
}

impl State {
    fn to_key(&self) -> usize {
        self.dir % 2 + self.i as usize * 2
        //only matters if the direction is vertical (0 and 2) or horizontal (1 and 3)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.c.cmp(&self.c)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid {
    data: Vec<u32>,
    nx: isize,
    ny: isize,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let nx = input.lines().next().unwrap().len() as isize;
        let data: Vec<_> = input
            .lines()
            .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap()))
            .collect();
        let ny = data.len() as isize / nx;
        Self { data, nx, ny }
    }
    fn get_checked(&self, i: isize, (dx, dy): (isize, isize)) -> Option<(&u32, isize)> {
        let new_i = i + dx + dy * self.nx;
        let x = i % self.nx + dx;
        if new_i >= 0 && x >= 0 && x < self.nx {
            return self.data.get(new_i as usize).map(|c| (c, new_i));
        }
        None
    }
}

#[cfg(test)]
mod day17 {

    use super::*;

    const INPUT: &'static str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "102");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "94");
        let input_extra1 = "111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(part2(input_extra1).to_string(), "71");
    }
}
