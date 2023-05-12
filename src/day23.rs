use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

pub fn day23(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't read input file");
    let time = Instant::now();
    //Part 1
    println!("Empty ground tiles: {}", do_day23_part1(&input));
    println!("{:?}", time.elapsed());

    let time = Instant::now();
    //Part 2
    println!("Number of rounds {}", do_day23_part2(&input));

    println!("{:?}", time.elapsed());
}

fn do_day23_part1(input: &str) -> usize {
    let mut elves = parse_input(input);
    let mut dirs = vec![Direction::N, Direction::S, Direction::W, Direction::E];
    let mut positions = HashMap::with_capacity(elves.len());
    let mut last_elves;
    let mut is_free = Vec::with_capacity(8);

    for _ in 0..10 {
        last_elves = elves.clone();
        elves.retain(|pos| {
            if let Some(new_pos) = try_move(pos.0, pos.1, &last_elves, &dirs, &mut is_free) {
                if let Some(other_pos) = positions.remove(&new_pos) {
                    positions.insert(other_pos, other_pos);
                    true
                } else {
                    positions.insert(new_pos, *pos);
                    false
                }
            } else {
                true
            }
        });

        elves.extend(positions.drain().map(|(k, _)| k));
        dirs.rotate_left(1);
    }
    let (min_x, max_x, min_y, max_y) = elves.iter().fold(
        (i16::MAX, i16::MIN, i16::MAX, i16::MIN),
        |(min_x, max_x, min_y, max_y), p| {
            (
                min_x.min(p.0),
                max_x.max(p.0),
                min_y.min(p.1),
                max_y.max(p.1),
            )
        },
    );
    ((1 + max_x - min_x) * (1 + max_y - min_y)) as usize - elves.len()
}

fn do_day23_part2(input: &str) -> usize {
    let mut elves = parse_input(input);
    let mut dirs = vec![Direction::N, Direction::S, Direction::W, Direction::E];
    let mut positions = HashMap::with_capacity(elves.len());
    let mut last_elves;
    let mut is_free = Vec::with_capacity(8);

    for i in 1..10000 {
        let mut moved_elves = 0;
        last_elves = elves.clone();
        elves.retain(|pos| {
            if let Some(new_pos) = try_move(pos.0, pos.1, &last_elves, &dirs, &mut is_free) {
                if let Some(other_pos) = positions.remove(&new_pos) {
                    moved_elves -= 1;
                    positions.insert(other_pos, other_pos);
                    true
                } else {
                    moved_elves += 1;
                    positions.insert(new_pos, *pos);
                    false
                }
            } else {
                true
            }
        });

        if moved_elves == 0 {
            return i;
        }

        elves.extend(positions.drain().map(|(k, _)| k));
        dirs.rotate_left(1);
    }
    panic!("simulation going for too long")
}

type Point = (i16, i16);
const DELTAS: [Point; 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[inline(always)]
fn try_move(
    x: i16,
    y: i16,
    others: &HashSet<Point>,
    dirs: &[Direction],
    is_free: &mut Vec<bool>,
) -> Option<Point> {
    is_free.clear();
    is_free.extend(
        DELTAS
            .iter()
            .map(|(dx, dy)| !others.contains(&(x + dx, y + dy))),
    );
    if is_free.iter().all(|c| *c) {
        return None;
    }
    for dir in dirs {
        match dir {
            Direction::N => {
                if is_free[0] && is_free[1] && is_free[2] {
                    return Some((x, y - 1));
                }
            }
            Direction::S => {
                if is_free[5] && is_free[6] && is_free[7] {
                    return Some((x, y + 1));
                }
            }
            Direction::W => {
                if is_free[0] && is_free[3] && is_free[5] {
                    return Some((x - 1, y));
                }
            }
            Direction::E => {
                if is_free[2] && is_free[4] && is_free[7] {
                    return Some((x + 1, y));
                }
            }
        }
    }
    None
}

fn parse_input(input: &str) -> HashSet<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(j, l)| {
            l.char_indices()
                .filter(|(_, c)| c == &'#')
                .map(move |(i, _)| (i as i16, j as i16))
        })
        .collect()
}

#[derive(Debug)]
enum Direction {
    N,
    S,
    W,
    E,
}

#[cfg(test)]
mod tests {

    use super::do_day23_part1;
    use super::do_day23_part2;

    const INPUT: &'static str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn part_1() {
        assert_eq!(do_day23_part1(INPUT), 110);
    }
    #[test]
    fn part_2() {
        assert_eq!(do_day23_part2(INPUT), 20);
    }
}
