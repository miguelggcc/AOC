use std::time::Instant;

use nom::{
    bytes::complete::tag,
    character::complete,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    Finish, IResult,
};

pub fn day15(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't read input file");
    let time = Instant::now();
    //Part 1
    println!("pos that cannot contain a beacon: {}", do_15_part1(&input));
    //Part 2
    //println!("Part 2, decoder key: {}", do_15_part2(&input));

    println!("{:?}", time.elapsed());
}

fn do_15_part1(input: &str) -> usize {
    let pairs = input
        .lines()
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1);
    let (x_min, x_max, y_min, y_max) = pairs.clone().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(x_min, x_max, y_min, y_max), (sensor, beacon)| {
            (
                x_min.min(sensor.x).min(beacon.x),
                x_max.max(sensor.x).max(beacon.x),
                y_min.min(sensor.y).min(beacon.y),
                y_max.max(sensor.y).max(beacon.y),
            )
        },
    );
    let (sensors, beacons): (Vec<_>, Vec<_>) =
        pairs.map(|(sensor, beacon)| (sensor, beacon)).unzip();
    let y = 2000000;
    (x_min - 2000000..x_max + 2000000)
        .filter(|x| {
            let p = Point::new(*x, y);
            sensors.iter().zip(&beacons).any(|(s, b)| {
                (s.manhattan_distance(&p) <= s.manhattan_distance(b)) ^ (&p == s || &p == b)
            })
        })
        .count()
}

/*fn do_15_part2(input: &str) -> i32 {

}*/

struct Grid {
    data: Vec<Device>,
    nx: usize,
    ny: usize,
}

impl Grid {
    /*fn build(rock_points: &[Point]) -> Self {

    } */
}

#[derive(Clone, Copy)]
enum Device {
    Sensor,
    Beacon,
    Air,
}

/*impl fmt::Display for Grid {
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
}*/

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

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
    //use super::do_15_part2;

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

        assert_eq!(do_15_part1(input), 26);
        //assert_eq!(do_15_part2(input), 93)
    }
}
