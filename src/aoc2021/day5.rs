use std::collections::{hash_map::Entry, HashMap};

use nom::{
    bytes::complete::tag,
    character::complete,
    combinator::{all_consuming, map},
    sequence::separated_pair,
    Finish, IResult,
};

pub fn part1(input: &str) -> usize {
    let pairs = input
        .lines()
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1);
    let mut points = HashMap::new();
    for (p1, p2) in pairs {
        for p in p1.interpolate_points(&p2) {
            match points.entry(p) {
                Entry::Occupied(mut o) => {
                    *o.get_mut() += 1;
                }
                Entry::Vacant(v) => {
                    v.insert(1);
                }
            }
        }
    }
    points.into_values().filter(|&v| v > 1).count()
}

pub fn part2(_input: &str) -> String {
    String::from("Not implemented")
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
            (0..delta_x.abs() + 1)
                .map(|dx| Point::new(self.x + delta_x.signum() * dx, self.y))
                .collect()
        } else if delta_x == 0 {
            (0..delta_y.abs() + 1)
                .map(|dy| Point::new(self.x, self.y + delta_y.signum() * dy))
                .collect()
        } else {
            vec![]
        }
    }
}

fn parse_line(input: &str) -> IResult<&str, (Point, Point)> {
    separated_pair(parse_point, tag(" -> "), parse_point)(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(complete::i32, complete::char(','), complete::i32),
        |(x, y)| Point { x, y },
    )(input)
}

#[cfg(test)]
mod day5 {

    use super::*;

    const INPUT: &'static str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 5);
    }
    #[test]
    #[ignore]
    fn part_2() {
        assert_eq!(part2(INPUT), "");
    }
}
