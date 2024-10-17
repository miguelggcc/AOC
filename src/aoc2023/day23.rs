use std::collections::{HashMap, HashSet, VecDeque};

pub fn part1(input: &str) -> impl std::fmt::Display {
    let (graph, nodes, start, end) = parse(input, false);
    let mut q = VecDeque::from([(start, 0, 0u128)]);
    let mut total = 0;
    dbg!(&graph.len(), nodes.len());
    while let Some((i, d, mut seen)) = q.pop_front() {
        if i == end {
            total = total.max(d);
            dbg!(total);
            continue;
        }
        seen |= 1 << i;
        for &(new_i, new_d) in graph[i].iter() {
            if (seen >> new_i) & 1 == 0 {
                q.push_back((new_i, d + new_d, seen))
            }
        }
    }
    total
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let (graph, nodes, start, end) = parse(input, true);
    let mut q = VecDeque::from([(start, 0, 0u128)]);
    let mut total = 0;
    dbg!(&graph.len(), nodes.len());
    while let Some((i, d, mut seen)) = q.pop_front() {
        if i == end {
            total = total.max(d);
            continue;
        }
        seen |= 1 << i;
        for &(new_i, new_d) in graph[i].iter() {
            if (seen >> new_i) & 1 == 0 {
                q.push_back((new_i, d + new_d, seen))
            }
        }
    }
    total
}

fn parse(input: &str, part2: bool) -> (Vec<Vec<(usize, usize)>>, Vec<usize>, usize, usize) {
    let nx = input.lines().next().unwrap().len() as isize;
    let grid: Vec<_> = input.lines().flat_map(|l| l.chars()).collect();
    let ny = grid.len() as isize / nx;
    let mut neighbours: Vec<_> = grid
        .iter()
        .enumerate()
        .map(|(i, c)| {
            match c {
                '#' => &[],
                _ if part2 => DIRS.as_slice(),
                '.' => DIRS.as_slice(),
                _ => &DIRS[SLOPES.iter().position(|s| s == c).unwrap()..][..1],
            }
            .iter()
            .filter_map(|(dx, dy)| {
                let x = i as isize % nx + dx;
                let y = i as isize / nx + dy;
                let neighbour = grid.get((x + y * nx) as usize);
                if neighbour.is_some_and(|&c| c != '#') {
                    return Some(((x + y * nx) as usize, 1));
                }
                None
            })
            .collect::<Vec<(usize, usize)>>()
        })
        .collect();
    let mut nodes = vec![1];
    for i in 2..neighbours.len() - 2 {
        let corridor = neighbours[i].clone();
        if corridor.len() == 2 {
            corridor
                .iter()
                .zip(corridor.iter().rev())
                .for_each(|(&(i1, d1), &(i2, d2))| {
                    if let Some(new_n) = neighbours[i1].iter_mut().find(|(ii, _)| *ii == i) {
                        *new_n = (i2, d1 + d2)
                    }
                });
        } else if corridor.len() > 0 {
            nodes.push(i);
        }
    }
    nodes.push((nx * ny - 2) as usize);
    neighbours.retain_mut(|n| {
        if n.len() > 0 && n.len() != 2 {
            n.iter_mut()
                .for_each(|(ii, _)| *ii = nodes.binary_search(&ii).unwrap());
            return true;
        }
        false
    });
    let end: usize = nodes.len() - 1;
    (neighbours, nodes, 0, end)
}

const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const SLOPES: [char; 4] = ['^', '>', 'v', '<'];

#[cfg(test)]
mod day22 {

    use super::*;

    const INPUT: &'static str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "94");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "154");
    }
}
