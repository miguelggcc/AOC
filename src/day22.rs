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
    //println!("Part 2, decoder key: {}", do_day22_part2(&input));

    println!("{:?}", time.elapsed());
}

fn do_day22_part1(input: &str) -> i32 {
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
        .map(|line| all_consuming(parse_path)(line).finish().unwrap().1)
        .flatten();

    let mut walker = Walker {
        x: board.data[0..board.nx]
            .iter()
            .position(|m| m == &Material::Open)
            .expect("No open tiles in first row") as i32,
        y: 0,
        direction: [1, 0],
    };

    path.for_each(|instruction| {
        match instruction {
            Instruction::Move(n) => {
                for _ in 0..n {
                    match board.get(
                        walker.x + walker.direction[0],
                        walker.y + walker.direction[1],
                    ) {
                        Some(&Material::Wall) => continue,
                        Some(&Material::Open) => {
                            walker.x += walker.direction[0];
                            walker.y += walker.direction[1];
                        }
                        Some(&Material::Nothing) | None => {
                            let mut temp_x = walker.x;
                            let mut temp_y = walker.y;
                            while board
                                .get(temp_x - walker.direction[0], temp_y - walker.direction[1])
                                .is_some()
                                && board
                                    .get(temp_x - walker.direction[0], temp_y - walker.direction[1])
                                    != Some(&Material::Nothing)
                            {
                                temp_x -= walker.direction[0];
                                temp_y -= walker.direction[1];
                            }
                            if board.get(temp_x, temp_y) != Some(&Material::Wall) {
                                walker.x = temp_x;
                                walker.y = temp_y;
                            }
                        }
                    }
                }
            }
            Instruction::R => {
                walker.direction.swap(0, 1);
                walker.direction[0] *= -1;
            }
            Instruction::L => {
                walker.direction.swap(0, 1);
                walker.direction[1] *= -1;
            }
        }
        //println!("x: {}, y: {}, next_material: {:?}", walker.x,walker.y, board.get(walker.x+walker.direction[0], walker.y+walker.direction[1]));
    });
    (walker.y + 1) * 1000 + 4 * (walker.x + 1)
}

fn do_day22_part2(input: &str) -> u32 {
    todo!()
}

struct Grid {
    data: Vec<Material>,
    nx: usize,
    ny: usize,
}

impl Grid {
    fn build(rock_points: &[Vec<Material>]) -> Self {
        let nx = rock_points
            .iter()
            .fold(0, |max_x, row| max_x.max(row.len()));
        let ny: usize = rock_points.len();

        let mut grid_data = vec![Material::Nothing; nx * ny];

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
    fn get(&self, x: i32, y: i32) -> Option<&Material> {
        if y < 0 || x < 0 || x >= self.nx as i32 {
            return None;
        }
        self.data.get(x as usize + y as usize * self.nx)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for j in 0..self.ny {
            for i in 0..self.nx {
                let c = match self.data[i + j * self.nx] {
                    Material::Wall => '#',
                    Material::Open => '.',
                    Material::Nothing => ' ',
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Material {
    Wall,
    Open,
    Nothing,
}

struct Walker {
    x: i32,
    y: i32,
    direction: [i32; 2],
}

#[derive(Debug)]
enum Instruction {
    Move(i32),
    R,
    L,
}

fn parse_row(input: &str) -> IResult<&str, Vec<Material>> {
    map(is_a(" #."), |r: &str| {
        r.chars()
            .map(|c| match c {
                ' ' => Material::Nothing,
                '.' => Material::Open,
                '#' => Material::Wall,
                e => panic!("error with character {e} in board "),
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
    use super::do_day22_part2;

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
        //assert_eq!(do_day22_part2(input), 93)
    }
}
