use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    time::Instant,
};

pub fn day23(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't read input file");
    let time = Instant::now();

    //Part 1
    //for _ in 0..100{
    println!("Empty ground tiles: {}", do_day23_part1(&input));
    //}
    //Part 2
    //println!("Part 2, password is: {}", do_day23_part2(&input));

    println!("{:?}", time.elapsed());
}

fn do_day23_part1(input: &str) -> usize {
    let mut elves: HashSet<_> = input
        .lines()
        .enumerate()
        .map(|(j, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| c == &'#')
                .map(move |(i, _)| (i as i32, j as i32))
        })
        .flatten()
        .collect();
    let mut dirs = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];
    let mut positions = HashMap::with_capacity(elves.len());

    for _ in 0..10 {
        elves.iter().for_each(|pos| {
            if let Some(new_position) = try_move(&pos.0, &pos.1, &elves, &dirs) {
                if let Some(value) = positions.remove(&new_position) {
                    positions.extend([(*pos, *pos), (value, value)].into_iter());
                } else {
                    positions.insert(new_position, *pos);
                }
            } else {
                positions.insert(*pos, *pos);
            }
        });

        elves.clear();
        elves.extend(positions.keys().copied());
        dirs.rotate_left(1);
        positions.clear();
    }
    let (min_x, max_x, min_y, max_y) = elves.iter().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
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

fn do_day23_part2(input: &str) -> i32 {
    todo!()
}

type Point = (i32, i32);
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

fn try_move(x: &i32, y: &i32, others: &HashSet<Point>, dirs: &[Direction]) -> Option<Point> {
    for dir in dirs {
        let (collision_dir, collision_rest) = match dir {
            Direction::North => (
                DELTAS
                    .iter()
                    .take(3)
                    .any(|(dx, dy)| others.contains(&(x + dx, y + dy))),
                DELTAS
                    .iter()
                    .skip(3)
                    .any(|(dx, dy)| others.contains(&(x + dx, y + dy))),
            ),
            Direction::South => (
                DELTAS
                    .iter()
                    .take(3)
                    .any(|(dx, dy)| others.contains(&(x + dx, y - dy))),
                DELTAS
                    .iter()
                    .skip(3)
                    .any(|(dx, dy)| others.contains(&(x + dx, y - dy))),
            ),
            Direction::West => (
                DELTAS
                    .iter()
                    .take(3)
                    .any(|(dy, dx)| others.contains(&(x + dx, y - dy))),
                DELTAS
                    .iter()
                    .skip(3)
                    .any(|(dy, dx)| others.contains(&(x + dx, y - dy))),
            ),
            Direction::East => (
                DELTAS
                    .iter()
                    .take(3)
                    .any(|(dy, dx)| others.contains(&(x - dx, y + dy))),
                DELTAS
                    .iter()
                    .skip(3)
                    .any(|(dy, dx)| others.contains(&(x - dx, y + dy))),
            ),
        };
        if !collision_dir && !collision_rest {
            return None;
        }
        if !collision_dir {
            return match dir {
                Direction::North => Some((*x, y - 1)),
                Direction::South => Some((*x, y + 1)),
                Direction::West => Some((x - 1, *y)),
                Direction::East => Some((x + 1, *y)),
            };
        }
    }
    None
}

/*fn display_grid(elves: &[Point], min_x: i32, max_x: i32, min_y: i32, max_y: i32){
let mut grid = vec![vec!['.'; (1 + max_x - min_x) as usize]; (1 + max_y - min_y) as usize];
    elves
        .iter()
        .for_each(|p| grid[(p.1 - min_y) as usize][(p.0 - min_x) as usize] = '#');

    for j in 0..grid.len() {
        println!("{}", grid[j].iter().collect::<String>());
    }
}*/

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[cfg(test)]
mod tests {

    use super::do_day23_part1;

    #[test]
    fn part_1() {
        let input = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

        assert_eq!(do_day23_part1(input), 110);
    }
    //No testing for part 2 because the solution is hardcoded for input :(
}
