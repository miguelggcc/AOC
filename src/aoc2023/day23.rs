use std::collections::VecDeque;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let nodes = parse(input, false);
    get_max(nodes)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let nodes = parse(input, true);
    get_max(nodes)
}

const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const SLOPES: [char; 4] = ['^', '>', 'v', '<'];

fn get_max(nodes: Vec<Vec<(usize, u16)>>) -> impl std::fmt::Display {
    let mut q = VecDeque::from([(0, 0u16, 0u64)]);
    let mut total = 0;
    while let Some((i, d, mut seen)) = q.pop_front() {
        if nodes[i].is_empty() {
            total = total.max(d);
            continue;
        }
        seen |= 1 << i;
        for &(new_i, new_d) in nodes[i].iter() {
            if (seen >> new_i) & 1 == 0 {
                q.push_back((new_i, d + new_d, seen))
            }
        }
    }
    total
}

fn parse(input: &str, part2: bool) -> Vec<Vec<(usize, u16)>> {
    let nx = input.lines().next().unwrap().len() as isize;
    let grid: Vec<_> = input.lines().flat_map(|l| l.chars()).collect();
    let mut visited = vec![None; grid.len()];
    let ny = grid.len() as isize / nx;
    let mut nodes = vec![vec![]];
    let mut q = VecDeque::from([(1, 2, 0, 0)]);

    while let Some((i, dir, d, last_node)) = q.pop_front() {
        if i == nx * ny - 2 {
            nodes.push(vec![]);
            let this_node = nodes.len() - 1;
            nodes[last_node].push((this_node, d));
            continue;
        }
        let neighbours = DIRS
            .iter()
            .enumerate()
            .filter_map(|(i_dir, (dx, dy))| {
                let new_i = i + dx + nx * (dy);
                match grid.get(new_i as usize) {
                    None => None,
                    _ if i_dir == (dir + 2) % 4 => None,
                    Some('#') => None,
                    _ if part2 => Some((new_i, i_dir)),
                    Some('.') => Some((new_i, i_dir)),
                    Some(c) if SLOPES.iter().position(|s| s == c).unwrap() == i_dir => {
                        Some((new_i, i_dir))
                    }
                    _ => None,
                }
            })
            .collect::<Vec<_>>();

        if neighbours.len() == 1 {
            q.push_back((neighbours[0].0, neighbours[0].1, d + 1, last_node));
        } else if !neighbours.is_empty() {
            if let Some(this_node) = visited[i as usize] {
                nodes[last_node].push((this_node, d));
            } else {
                let this_node = nodes.len();
                nodes[last_node].push((this_node, d));
                nodes.push(vec![]);
                for (new_i, new_dir) in neighbours {
                    q.push_back((new_i, new_dir, 1, this_node));
                }
                let (dx, dy) = DIRS[(dir + 2) % 4];
                q.push_back((i + dx + nx * dy, (dir + 2) % 4, 1, this_node));
                visited[i as usize] = Some(this_node);
            }
        }
    }
    nodes
}

#[cfg(test)]
mod day23 {

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
