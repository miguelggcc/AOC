use std::{
    collections::{hash_map::Entry, HashMap},
    iter::once,
};

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
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1)
        .filter(|(p1, p2)| p1.x == p2.x || p1.y == p2.y);
    get_overlaps(pairs)
}

pub fn part2(input: &str) -> usize {
    let pairs = input
        .lines()
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1);
    get_overlaps(pairs)
}

fn get_overlaps(pairs: impl Iterator<Item = (Point, Point)>) -> usize {
    let mut points = HashMap::new();
    for (p1, p2) in pairs {
        for p in p1.interpolate_points(p2) {
            match points.entry(p.get_key()) {
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

#[derive(Eq, PartialEq, Hash)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
    fn interpolate_points(&self, other: Point) -> impl Iterator<Item = Point> + '_ {
        let delta_x = other.x - self.x;
        let delta_y = other.y - self.y;
        once((delta_x.signum(), delta_y.signum()))
            .cycle()
            .take(1 + delta_x.abs().max(delta_y.abs()) as usize)
            .enumerate()
            .map(|(i, (dx, dy))| Point::new(self.x + i as i16 * dx, self.y + i as i16 * dy))
    }

    fn get_key(&self)->i32{
        (self.y as i32)|(self.x as i32)<<16
    }
}

fn parse_line(input: &str) -> IResult<&str, (Point, Point)> {
    separated_pair(parse_point, tag(" -> "), parse_point)(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(complete::i16, complete::char(','), complete::i16),
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
        assert_eq!(part2(INPUT), 12);
    }
}
