use std::{collections::HashSet, ops::Range, time::Instant};

use nom::{
    bytes::complete::tag,
    character::complete,
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

pub fn day15(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't read input file");
    let time = Instant::now();
    //Part 1
    println!(
        "pos that cannot contain a beacon: {}",
        do_15_part1(&input, 2000000)
    );
    //Part 2
    println!("Part 2, decoder key: {}", do_15_part2(&input, 4000000));

    println!("{:?}", time.elapsed());
}

fn do_15_part1(input: &str, y: i32) -> usize {
    let pairs = input
        .lines()
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1);

    let mut forbidden_xs = HashSet::new();

    let mut ranges = pairs
        .map(|(s, b)| {
            if b.y == y {
                forbidden_xs.insert(b.x);
            }
            let distance = s.manhattan_distance(&b);
            let width_in_line = distance - (s.y - y).abs();
            s.x - width_in_line..s.x + width_in_line + 1
        })
        .collect::<Vec<_>>();

    ranges.sort_by_key(|r| r.start);

    let mut merged = vec![];
    for r in ranges {
        if merged.is_empty() || !overlaps(&r, merged.last().unwrap()) {
            merged.push(r);
        } else {
            let last = merged.pop().unwrap();
            merged.push(last.start..r.end.max(last.end));
        }
    }
    merged.into_iter().fold(0, |acc, r| acc + r.len()) - forbidden_xs.len()
}

fn do_15_part2(input: &str, max_coord: usize) -> u64 {
    let pairs = input
        .lines()
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1);

    let (sensors, beacons): (Vec<_>, Vec<_>) = pairs.clone().unzip();

    let distances: Vec<_> = pairs.map(|(s, b)| s.manhattan_distance(&b)).collect();

    for y in 0..max_coord as i32 {
        let mut ranges = sensors
            .iter()
            .zip(&distances)
            .map(|(s, d)| {
                let width_in_line = d - (s.y - y).abs();
                s.x - width_in_line..s.x + width_in_line + 1
            })
            .filter(|r| !r.is_empty())
            .collect::<Vec<_>>();

        ranges.sort_by_key(|r| r.start);
        let (merged, splitted_range) = ranges.split_first_mut().unwrap();
        for r in splitted_range {
            if overlaps(r, merged) {
                merged.end = r.end.max(merged.end);
            } else if !beacons.contains(&Point { x: merged.end, y }) {
                return 4000000 * merged.end as u64 + y as u64;
            }
        }
    }
    panic!("couldn't find beacon")
}

fn overlaps<T>(a: &Range<T>, b: &Range<T>) -> bool
where
    T: PartialOrd,
{
    a.start <= b.end && b.start <= a.end
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}

fn parse_line(input: &str) -> IResult<&str, (Point, Point)> {
    preceded(
        tag("Sensor at x="),
        separated_pair(parse_point, tag(": closest beacon is at x="), parse_point),
    )(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(complete::i32, tag(", y="), complete::i32),
        |(x, y)| Point { x, y },
    )(input)
}

#[cfg(test)]
mod tests {

    use super::do_15_part1;
    use super::do_15_part2;

    #[test]
    fn part_1() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        assert_eq!(do_15_part1(input, 10), 26);
        assert_eq!(do_15_part2(input, 20), 56000011)
    }
}
