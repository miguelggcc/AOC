use std::{cmp::Ordering, collections::BinaryHeap};

pub fn part1(input: &str) -> u32 {
    let (map, nx, ny) = parse(input);
    find_best_path(map.into_iter().flatten().collect(), nx, ny)
}

pub fn part2(input: &str) -> u32 {
    let (map, nx, ny) = parse(input);
    let get_risk = |i, n| (n + i as u32) % 10 + (n + i as u32) / 10;
    let mut big_map: Vec<u32> = map
        .into_iter()
        .flat_map(|row| {
            (0..5).flat_map(move |i| row.clone().into_iter().map(move |n| get_risk(i, n)))
        })
        .collect();
    big_map.extend(
        (1..5)
            .flat_map(|i| big_map.iter().map(move |n| get_risk(i, *n)))
            .collect::<Vec<_>>(),
    );
    find_best_path(big_map, nx * 5, ny * 5)
}

fn find_best_path(map: Vec<u32>, nx: usize, ny: usize) -> u32 {
    let walker_root = Walker {
        index: 0,
        risk: 0,
        dist: 0,
    };
    let end = (nx * ny) - 1;
    let mut heap = BinaryHeap::from([walker_root]);

    let mut visited = vec![u32::MAX; nx * ny];

    while let Some(p_walker) = heap.pop() {
        if p_walker.index == end {
            return p_walker.risk;
        }
        if p_walker.risk <= visited[p_walker.index] {
            for nindex in neighbours(p_walker.index, nx as isize, ny as isize) {
                let risk = p_walker.risk + map[nindex];
                if risk < visited[nindex] {
                    visited[nindex] = risk;
                    let walker = Walker {
                        index: nindex,
                        risk,
                        dist: nindex % nx + nindex / nx,
                    };
                    heap.push(walker);
                }
            }
        }
    }
    panic!("path not found")
}

#[derive(Clone, Eq, PartialEq)]
struct Walker {
    risk: u32,
    index: usize,
    dist: usize,
}

impl Ord for Walker {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.dist.cmp(&other.dist))
    }
}

impl PartialOrd for Walker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const DELTAS: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
fn neighbours(index: usize, nx: isize, ny: isize) -> impl Iterator<Item = usize> {
    let x = index as isize % nx;
    let y = index as isize / nx;
    DELTAS
        .iter()
        .filter(move |(dx, dy)| x + dx >= 0 && y + dy >= 0 && x + dx < nx && y + dy < ny)
        .map(move |(dx, dy)| (x + dx + (y + dy) * nx) as usize)
}

fn parse(input: &str) -> (Vec<Vec<u32>>, usize, usize) {
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let nx = map[0].len();
    let ny = map.len();
    (map, nx, ny)
}

#[cfg(test)]
mod day15 {

    use super::*;

    const INPUT: &'static str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 40);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 315);
    }
}
