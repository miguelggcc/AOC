use core::fmt;
use nom::branch::alt;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::{bytes::complete::is_a, combinator::map};
use std::time::Instant;

use nom::{character::complete, combinator::all_consuming, Finish, IResult};

pub fn day22(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't read input file");
    let time = Instant::now();

    //Part 1
    println!("Password is: {}", do_day22_part1(&input));
    //Part 2
    println!("Part 2, password is: {}", do_day22_part2(&input));

    println!("{:?}", time.elapsed());
}

fn do_day22_part1(input: &str) -> i32 {
    let (board, path) = parse_input(input);

    let mut walker = Walker {
        x: board.data[0..board.nx]
            .iter()
            .position(|t| t == &Tile::Open)
            .expect("no open tiles in first row") as i32,
        y: 0,
        direction: Direction::right(),
    };

    path.into_iter().for_each(|instruction| match instruction {
        Instruction::Move(n) => {
            for _ in 0..n {
                match board.get(walker.x + walker.direction.x, walker.y + walker.direction.y) {
                    Some(&Tile::Wall) => continue,
                    Some(&Tile::Open) => {
                        walker.step();
                    }
                    Some(&Tile::Nothing) | None => {
                        let mut temp_x = walker.x;
                        let mut temp_y = walker.y;
                        while let Some(tile) =
                            board.get(temp_x - walker.direction.x, temp_y - walker.direction.y)
                        {
                            if tile == &Tile::Nothing {
                                break;
                            }
                            temp_x -= walker.direction.x;
                            temp_y -= walker.direction.y;
                        }
                        if board.get(temp_x, temp_y) != Some(&Tile::Wall) {
                            walker.x = temp_x;
                            walker.y = temp_y;
                        }
                    }
                }
            }
        }
        Instruction::R => {
            walker.direction.clockwise();
        }
        Instruction::L => {
            walker.direction.anticlockwise();
        }
    });
    walker.get_password()
}

fn do_day22_part2(input: &str) -> i32 {
    let (board, path) = parse_input(input);

    let mut walker = Walker {
        x: board.data[0..board.nx]
            .iter()
            .position(|t| t == &Tile::Open)
            .expect("no open tiles in first row") as i32,
        y: 0,
        direction: Direction::right(),
    };

    let face_length = (board.nx / 3) as i32;

    path.into_iter().for_each(|instruction| {
        match instruction {
            Instruction::Move(n) => {
                for _ in 0..n {
                    match board.get(walker.x + walker.direction.x, walker.y + walker.direction.y) {
                        Some(&Tile::Wall) => continue,
                        Some(&Tile::Open) => {
                            walker.step();
                        }
                        Some(&Tile::Nothing) | None => {
                            let current_face = match (
                                3 * walker.x as usize / board.nx,
                                4 * walker.y as usize / board.ny,
                            ) {
                                (1, 0) => 1,
                                (2, 0) => 2,
                                (1, 1) => 3,
                                (0, 2) => 4,
                                (1, 2) => 5,
                                (0, 3) => 6,
                                _ => panic!("out of bounds"),
                            };
                            //Facing is 0 for right, 1 for down, 2 for left, and 3 for up
                            let (temp_x, temp_y, temp_direction) =
                                match (current_face, walker.direction.get_value()) {
                                    (1, 2) => {
                                        (0, 3 * face_length - 1 - walker.y, Direction::right())
                                    }
                                    (1, 3) => (
                                        0,
                                        3 * face_length + walker.x - face_length,
                                        Direction::right(),
                                    ),
                                    (2, 3) => (
                                        walker.x - face_length * 2,
                                        face_length * 4 - 1,
                                        Direction::up(),
                                    ),
                                    (2, 0) => (
                                        2 * face_length - 1,
                                        3 * face_length - 1 - walker.y,
                                        Direction::left(),
                                    ),
                                    (2, 1) => (
                                        2 * face_length - 1,
                                        walker.x - 2 * face_length + face_length,
                                        Direction::left(),
                                    ),
                                    (3, 2) => {
                                        (walker.y - face_length, 2 * face_length, Direction::down())
                                    }
                                    (3, 0) => (
                                        walker.y - face_length + 2 * face_length,
                                        face_length - 1,
                                        Direction::up(),
                                    ),
                                    (4, 2) => (
                                        face_length,
                                        3 * face_length - 1 - walker.y,
                                        Direction::right(),
                                    ),
                                    (4, 3) => {
                                        (face_length, walker.x + face_length, Direction::right())
                                    }
                                    (5, 0) => (
                                        3 * face_length - 1,
                                        3 * face_length - 1 - walker.y,
                                        Direction::left(),
                                    ),
                                    (5, 1) => (
                                        face_length - 1,
                                        walker.x - face_length + 3 * face_length,
                                        Direction::left(),
                                    ),
                                    (6, 2) => (
                                        walker.y - 3 * face_length + face_length,
                                        0,
                                        Direction::down(),
                                    ),
                                    (6, 1) => (walker.x + 2 * face_length, 0, Direction::down()),
                                    (6, 0) => (
                                        walker.y - 3 * face_length + face_length,
                                        3 * face_length - 1,
                                        Direction::up(),
                                    ),
                                    _ => panic!("impossible maneuver"),
                                };

                            if board.get(temp_x, temp_y) != Some(&Tile::Wall) {
                                walker.x = temp_x;
                                walker.y = temp_y;
                                walker.direction = temp_direction;
                            }
                        }
                    }
                }
            }
            Instruction::R => {
                walker.direction.clockwise();
            }
            Instruction::L => {
                walker.direction.anticlockwise();
            }
        }
    });
    walker.get_password()
}

fn parse_input(input: &str) -> (Grid, Vec<Instruction>) {
    let mut lines = input.lines();
    let parsed_points = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_row)(line)
                .finish()
                .ok()
                .map(|(_input, l)| l)
        })
        .collect::<Vec<_>>();

    let board = Grid::build(&parsed_points);

    let path = lines
        .flat_map(|line| all_consuming(parse_path)(line).finish().unwrap().1)
        .collect::<Vec<Instruction>>();
    (board, path)
}

struct Grid {
    data: Vec<Tile>,
    nx: usize,
    ny: usize,
}

impl Grid {
    fn build(rock_points: &[Vec<Tile>]) -> Self {
        let nx = rock_points
            .iter()
            .fold(0, |max_x, row| max_x.max(row.len()));
        let ny: usize = rock_points.len();

        let mut grid_data = vec![Tile::Nothing; nx * ny];

        rock_points
            .iter()
            .enumerate()
            .for_each(|(j, row)| grid_data[j * nx..j * nx + row.len()].copy_from_slice(row));

        Self {
            data: grid_data,
            nx,
            ny,
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<&Tile> {
        if x >= self.nx as i32 {
            return None;
        }
        let x_us = usize::try_from(x).ok()?;
        let y_us = usize::try_from(y).ok()?;
        self.data.get(x_us + y_us * self.nx)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for j in 0..self.ny {
            for i in 0..self.nx {
                let c = match self.data[i + j * self.nx] {
                    Tile::Wall => '#',
                    Tile::Open => '.',
                    Tile::Nothing => ' ',
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Wall,
    Open,
    Nothing,
}

struct Walker {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Walker {
    fn step(&mut self) {
        self.x += self.direction.x;
        self.y += self.direction.y;
    }
    fn get_password(&self) -> i32 {
        1000 * (self.y + 1) + 4 * (self.x + 1) + self.direction.get_value()
    }
}

#[derive(Debug)]
struct Direction {
    x: i32,
    y: i32,
}

impl Direction {
    fn right() -> Self {
        Self { x: 1, y: 0 }
    }
    fn down() -> Self {
        Self { x: 0, y: 1 }
    }
    fn left() -> Self {
        Self { x: -1, y: 0 }
    }
    fn up() -> Self {
        Self { x: 0, y: -1 }
    }
    fn get_value(&self) -> i32 {
        match (self.x, self.y) {
            (1, 0) => 0,
            (0, 1) => 1,
            (-1, 0) => 2,
            (0, -1) => 3,
            _ => panic!("impossible direction"),
        }
    }
    fn clockwise(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
        self.x *= -1;
    }
    fn anticlockwise(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
        self.y *= -1;
    }
}

#[derive(Debug)]
enum Instruction {
    Move(i32),
    R,
    L,
}

fn parse_row(input: &str) -> IResult<&str, Vec<Tile>> {
    map(is_a(" .#"), |r: &str| {
        r.chars()
            .map(|c| match c {
                ' ' => Tile::Nothing,
                '.' => Tile::Open,
                '#' => Tile::Wall,
                e => panic!("error with character {e} in board"),
            })
            .collect::<Vec<_>>()
    })(input)
}

fn parse_path(input: &str) -> IResult<&str, Vec<Instruction>> {
    let move_parser = map(complete::digit1, |n: &str| {
        Instruction::Move(n.parse().expect("error parsing move instruction"))
    });
    let direction_parser = map(complete::alpha1, |d| match d {
        "L" => Instruction::L,
        "R" => Instruction::R,
        e => panic!("error with character {e} in path"),
    });
    let instruction_parser = alt((move_parser, direction_parser));
    preceded(complete::space0, many1(instruction_parser))(input)
}

#[cfg(test)]
mod tests {

    use super::do_day22_part1;

    #[test]
    fn part_1() {
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

        assert_eq!(do_day22_part1(input), 6032);
    }
    //No testing for part 2 because the solution is hardcoded for input :(
}
