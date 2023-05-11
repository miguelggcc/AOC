use std::{
    collections::{HashSet, VecDeque},
    iter::once,
    time::Instant,
};

pub fn day24(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't read input file");
    let time = Instant::now();
    //Part 1
    println!("Fewest number of minutes: {}", do_day24_part1(&input));

    println!("{:?}", time.elapsed());

    let time = Instant::now();
    //Part 2
    println!(
        "Three trips, fewest number of minutes: {}",
        do_day24_part2(&input)
    );

    println!("{:?}", time.elapsed());
}

fn do_day24_part1(input: &str) -> usize {
    let (start, exit, blizzards, width, height) = parse_input(input);
    let exit_previous = (exit.0, exit.1 - 1);
    do_bfs(start, exit_previous, &blizzards, width, height, 0) + 1
}

fn do_day24_part2(input: &str) -> usize {
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
    width: usize,
    height: usize,
    time_0: usize,
) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((start, time_0));
    let mut max_time = 0;
    let mut forbidden_pos: HashSet<u32> = HashSet::from_iter(
        blizzards
            .iter()
            .map(|b| get_bits(time_0, b.get_pos_at_time(width, height, time_0))),
    );

    while let Some((pos, time)) = queue.pop_front() {
        if time > max_time {
            forbidden_pos.extend(
                blizzards
                    .iter()
                    .map(|b| get_bits(time, b.get_pos_at_time(width, height, time))),
            );
            max_time = time;
        }
        for next_pos in get_neighbours(pos, width, height).chain(once(pos)) {
            let state = get_bits(time, next_pos);
            if !forbidden_pos.contains(&state) {
                forbidden_pos.insert(state);
                if next_pos == exit {
                    return time + 1;
                }
                queue.push_back((next_pos, time + 1));
            }
        }
    }
    panic!("path not found");
}

#[inline(always)]
fn get_bits(time: usize, pos: Point) -> u32 {
    time as u32 + ((pos.0 as u32) << 16) + ((pos.1 as u32) << 24)
}

type Point = (usize, usize);
const DELTAS: [(i16, i16); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

fn get_neighbours(pos: Point, width: usize, height: usize) -> impl Iterator<Item = Point> {
    DELTAS
        .iter()
        .map(move |(dx, dy)| ((pos.0 as i16 + dx) as usize, (pos.1 as i16 + dy) as usize))
        .filter(move |(x, y)| x > &0 && x <= &width && y > &0 && y <= &height)
}

#[derive(Debug)]
struct Blizzard {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Blizzard {
    fn get_pos_at_time(&self, width: usize, height: usize, time: usize) -> Point {
        match self.direction {
            Direction::N => (
                self.x,
                (self.y as i16 - (time + 1) as i16 - 1).rem_euclid(height as i16) as usize + 1,
            ),
            Direction::S => (self.x, (self.y + (time + 1) - 1) % height + 1),
            Direction::E => ((self.x + (time + 1) - 1) % width + 1, self.y),
            Direction::W => (
                (self.x as i16 - (time + 1) as i16 - 1).rem_euclid(width as i16) as usize + 1,
                self.y,
            ),
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

fn parse_input(input: &str) -> (Point, Point, Vec<Blizzard>, usize, usize) {
    let mut lines: Vec<_> = input.lines().collect();
    let height = lines.len() - 2;
    let width = lines[0].len() - 2;
    let start = (
        lines[0]
            .chars()
            .position(|c| c == '.')
            .expect("no start position"),
        0,
    );

    let exit = (
        lines
            .pop()
            .unwrap()
            .chars()
            .position(|c| c == '.')
            .expect("no exit position"),
        lines.len(),
    );

    let blizzards: Vec<_> = lines
        .iter()
        .enumerate()
        .skip(1)
        .flat_map(|(y, l)| {
            l.char_indices()
                .skip(1)
                .filter(|(_, c)| c != &'#' && c != &'.')
                .map(move |(x, c)| Blizzard {
                    x,
                    y,
                    direction: match c {
                        '>' => Direction::E,
                        '<' => Direction::W,
                        '^' => Direction::N,
                        'v' => Direction::S,
                        e => panic!("unexpected char {e}"),
                    },
                })
        })
        .collect();
    (start, exit, blizzards, width, height)
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
