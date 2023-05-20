use core::fmt;
use nom::combinator::map;
use nom::sequence::separated_pair;

use nom::{
    bytes::complete::tag, character::complete, combinator::all_consuming, multi::separated_list1,
    Finish, IResult,
};

pub fn part1(input: &str) -> u32 {
    let parsed_points = input
        .lines()
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1);

    let mut rock_points = vec![];

    parsed_points.for_each(|row: Vec<Point>| {
        rock_points.push(*row.first().unwrap());
        row.windows(2)
            .for_each(|points| rock_points.extend(points[0].interpolate_points(&points[1])))
    });

    let mut grid = Grid::build(&rock_points);

    let mut total = 0;
    let mut path = vec![grid.sand_spawn];

    while let Some(output_point) = grid.calculate_falling_sand(&mut path) {
        grid.new_fallen_sand(output_point);
        total += 1;
    }
    //println!("{grid}");
    total
}

pub fn part2(input: &str) -> u32 {
    let parsed_points = input
        .lines()
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1);

    let mut rock_points = vec![];

    parsed_points.for_each(|row: Vec<Point>| {
        rock_points.push(*row.first().unwrap());
        row.windows(2)
            .for_each(|ptuple| rock_points.extend(ptuple[0].interpolate_points(&ptuple[1])))
    });

    let mut grid = Grid::build_part2(&rock_points);

    let mut total = 0;
    let mut path = vec![grid.sand_spawn];
    while let Some(output_point) = grid.calculate_falling_sand(&mut path) {
        grid.new_fallen_sand(output_point);
        total += 1;
        if output_point == grid.sand_spawn {
            break;
        }
    }
    //println!("{grid}");
    total
}

struct Grid {
    data: Vec<Material>,
    nx: usize,
    ny: usize,
    sand_spawn: Point,
    min_x: i32,
    max_x: i32,
}

impl Grid {
    fn build(rock_points: &[Point]) -> Self {
        let (min_x, max_x, max_y) = rock_points
            .iter()
            .fold((i32::MAX, 0, 0), |(min_x, max_x, max_y), p| {
                (min_x.min(p.x), max_x.max(p.x), max_y.max(p.y))
            });

        let nx = (max_x + 1 - min_x) as usize;
        let ny = (max_y + 1) as usize;

        let mut grid_data = vec![Material::Air; nx * ny];

        rock_points.iter().for_each(|p| {
            grid_data[p.to_index(min_x as usize, nx)] = Material::Rock;
        });

        let sand_spawn = Point::new(500, 0);
        Self {
            data: grid_data,
            nx,
            ny,
            sand_spawn,
            min_x,
            max_x,
        }
    }

    fn build_part2(rock_points: &[Point]) -> Self {
        let max_y = rock_points.iter().fold(0, |max_y, p| max_y.max(p.y));

        let padding = (max_y + 1) * 2; //45º slope

        let min_x = 500 - padding;
        let max_x = 500 + padding;

        let nx = (max_x + 1 - min_x) as usize;
        let ny = (max_y + 1) as usize;

        let mut grid_data = vec![Material::Air; nx * ny];

        rock_points.iter().for_each(|p| {
            grid_data[p.to_index(min_x as usize, nx)] = Material::Rock;
        });

        grid_data.extend(vec![Material::Air; nx]);
        grid_data.extend(vec![Material::Rock; nx]);

        let sand_spawn = Point::new(500, 0);
        Self {
            data: grid_data,
            nx,
            ny: ny + 2,
            sand_spawn,
            min_x,
            max_x,
        }
    }

    fn calculate_falling_sand(&self, path: &mut Vec<Point>) -> Option<Point> {
        let mut tests = [Point::new(0, 0); 3];

        fn get_tests(tests: &mut [Point; 3], starting_point: &Point) {
            let new_y = starting_point.y + 1;
            tests[0].x = starting_point.x;
            tests[0].y = new_y;
            tests[1].x = starting_point.x - 1;
            tests[1].y = new_y;
            tests[2].x = starting_point.x + 1;
            tests[2].y = new_y;
        }
        'outer: loop {
            get_tests(&mut tests, path.last().expect("Vector is length 0"));
            'inner: for p in tests {
                match self.get(&p) {
                    Some(m) => {
                        if m.is_solid() {
                            continue 'inner;
                        } else {
                            path.push(p);
                            continue 'outer;
                        }
                    }
                    None => return None,
                }
            }
            break 'outer;
        }
        Some(path.pop().expect("Vector is length 0"))
    }

    fn new_fallen_sand(&mut self, sand_point: Point) {
        self.data[sand_point.to_index(self.min_x as usize, self.nx)] = Material::Sand;
    }

    fn get(&self, point: &Point) -> Option<&Material> {
        if point.x >= self.min_x && point.x <= self.max_x {
            return self.data.get(point.to_index(self.min_x as usize, self.nx));
        }
        None
    }
}

#[derive(Clone, Copy)]
enum Material {
    Rock,
    Air,
    Sand,
}

impl Material {
    fn is_solid(&self) -> bool {
        matches!(&self, Material::Rock | Material::Sand)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for j in 0..self.ny {
            for i in 0..self.nx {
                let c = match self.data[i + j * self.nx] {
                    Material::Rock => '#',
                    Material::Air => '.',
                    Material::Sand => 'o',
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn interpolate_points(&self, other: &Point) -> Vec<Point> {
        let delta_x = other.x - self.x;
        let delta_y = other.y - self.y;
        if delta_y == 0 {
            (1..delta_x.abs() + 1)
                .map(|dx| Point::new(self.x + delta_x.signum() * dx, self.y))
                .collect()
        } else {
            (1..delta_y.abs() + 1)
                .map(|dy| Point::new(self.x, self.y + delta_y.signum() * dy))
                .collect()
        }
    }

    #[inline(always)]
    fn to_index(self, min_x: usize, nx: usize) -> usize {
        self.x as usize - min_x + self.y as usize * nx
    }
}

fn parse_line(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(tag(" -> "), parse_point)(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(complete::i32, complete::char(','), complete::i32),
        |(x, y)| Point { x, y },
    )(input)
}

#[cfg(test)]
mod tests {

    use super::part1;
    use super::part2;

    #[test]
    fn part_1_2() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        assert_eq!(part1(input), 24);
        assert_eq!(part2(input), 93)
    }
}
