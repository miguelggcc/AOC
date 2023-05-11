use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::once,
    time::Instant,
};

pub fn day24(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't read input file");
    let time = Instant::now();
    //Part 1
    println!("Fewest number of minutes  {}", do_day24_part1(&input));

    println!("{:?}", time.elapsed());

    let time = Instant::now();
    //Part 2
    println!("Number of rounds {}", do_day24_part2(&input));

    println!("{:?}", time.elapsed());
}

fn do_day24_part1(input: &str) -> i16 {
    let (start, exit, blizzards, width, height) = parse_input(input);
    let exit_previous = (exit.0, exit.1 - 1);
    do_bfs(start, exit_previous, &blizzards, width, height, 0) + 1
}

fn do_day24_part2(input: &str) -> i16 {
    let (start, exit, blizzards, width, height) = parse_input(input);
    let start_next = (start.0, start.1 + 1);
    let exit_previous = (exit.0, exit.1 - 1);
    let first_trip = do_bfs(start, exit_previous, &blizzards, width, height, 0) + 1;
    let second_trip = do_bfs(exit, start_next, &blizzards, width, height, first_trip) + 1;
    do_bfs(start, exit_previous, &blizzards, width, height, second_trip) + 1
}

fn do_bfs(
    start: Point,
    exit: Point,
    blizzards: &[Blizzard],
    width: i16,
    height: i16,
    time_0: i16,
) -> i16 {
    let mut queue = VecDeque::new();
    queue.push_back((start, time_0));
    let mut max_time = 0;
    let mut forbidden_pos: HashSet<(i16, Point)> = HashSet::from_iter(
        blizzards
            .iter()
            .map(|b| (time_0, b.get_pos_at_time(width, height, time_0))),
    );

    while let Some((pos, time)) = queue.pop_front() {
        if time > max_time {
            forbidden_pos.extend(
                blizzards
                    .iter()
                    .map(|b| (time, b.get_pos_at_time(width, height, time))),
            );
            max_time = time;
        }
        for next_pos in get_neighbours(pos, width, height).chain(once(pos)) {
            if !forbidden_pos.contains(&(time, next_pos)) {
                forbidden_pos.insert((time, next_pos));
                if next_pos == exit {
                    return time + 1;
                }
                queue.push_back((next_pos, time + 1));
            }
        }
    }
    panic!("path not found");
}

type Point = (i16, i16);
const DELTAS: [Point; 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

fn get_neighbours(pos: Point, width: i16, height: i16) -> impl Iterator<Item = Point> {
    DELTAS
        .iter()
        .map(move |(dx, dy)| (pos.0 + dx, pos.1 + dy))
        .filter(move |(x, y)| x > &0 && x <= &width && y > &0 && y <= &height)
}

#[derive(Debug)]
struct Blizzard {
    x: i16,
    y: i16,
    direction: Direction,
}

impl Blizzard {
    fn get_pos_at_time(&self, width: i16, height: i16, time: i16) -> Point {
        match self.direction {
            Direction::N => (self.x, (self.y - (time + 1) - 1).rem_euclid(height) + 1),
            Direction::S => (self.x, (self.y + (time + 1) - 1) % height + 1),
            Direction::E => ((self.x + (time + 1) - 1) % width + 1, self.y),
            Direction::W => ((self.x - (time + 1) - 1).rem_euclid(width) + 1, self.y),
        }
    }
}

#[derive(Debug)]
enum Direction {
    N,
    S,
    E,
    W,
}

fn parse_input(input: &str) -> (Point, Point, Vec<Blizzard>, i16, i16) {
    let mut lines: Vec<_> = input.lines().collect();
    let height = lines.len() - 2;
    let width = lines[0].len() - 2;
    let start = (
        lines[0]
            .chars()
            .position(|c| c == '.')
            .expect("no entry position") as i16,
        0,
    );

    let exit_x = lines
        .pop()
        .unwrap()
        .chars()
        .position(|c| c == '.')
        .expect("no exit position") as i16;

    let blizzards: Vec<_> = lines
        .iter()
        .enumerate()
        .skip(1)
        .map(|(j, l)| {
            l.char_indices().skip(1).flat_map(move |(i, c)| match c {
                '>' => Some(Blizzard {
                    x: i as i16,
                    y: j as i16,
                    direction: Direction::E,
                }),
                '<' => Some(Blizzard {
                    x: i as i16,
                    y: j as i16,
                    direction: Direction::W,
                }),
                '^' => Some(Blizzard {
                    x: i as i16,
                    y: j as i16,
                    direction: Direction::N,
                }),
                'v' => Some(Blizzard {
                    x: i as i16,
                    y: j as i16,
                    direction: Direction::S,
                }),
                _ => None,
            })
        })
        .flatten()
        .collect();
    (
        start,
        (exit_x, lines.len() as i16),
        blizzards,
        width as i16,
        height as i16,
    )
}

#[cfg(test)]
mod tests {

    use super::do_day24_part1;
    use super::do_day24_part2;

    const INPUT: &'static str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn part_1() {
        assert_eq!(do_day24_part1(INPUT), 18);
    }
    #[test]
    fn part_2() {
        assert_eq!(do_day24_part2(INPUT), 54);
    }
}
