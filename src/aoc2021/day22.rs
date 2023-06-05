use std::{collections::HashSet, ops::Range};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    Finish, IResult,
};

pub fn part1(input: &str) -> usize {
    let steps = parse(input).finish().ok().unwrap().1;
    let mut cubes = HashSet::new();
    for (switch, ranges) in steps
        .into_iter()
        .filter(|(_, rs)| rs.iter().all(|r| r.start >= -50 && r.end <= 51))
    {
        for z in ranges[2].clone() {
            for y in ranges[1].clone() {
                for x in ranges[0].clone() {
                    if switch {
                        cubes.insert((x, y, z));
                    } else {
                        cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    cubes.len()
}

pub fn part2(_input: &str) -> String {
    String::from("Not implemented")
}
type Step = (bool, [Range<i32>; 3]);
fn parse(input: &str) -> IResult<&str, Vec<Step>> {
    separated_list1(complete::line_ending, parse_step)(input)
}
fn parse_step(input: &str) -> IResult<&str, Step> {
    let parse_switch = alt((map(tag("on"), |_| true), map(tag("off"), |_| false)));
    separated_pair(
        parse_switch,
        tag(" x="),
        map(
            tuple((
                parse_range,
                preceded(tag(",y="), parse_range),
                preceded(tag(",z="), parse_range),
            )),
            |rs| [rs.0, rs.1, rs.2],
        ),
    )(input)
}

fn parse_range(input: &str) -> IResult<&str, Range<i32>> {
    map(
        separated_pair(complete::i32, tag(".."), complete::i32),
        |(start, end)| start..(end + 1),
    )(input)
}

#[cfg(test)]
mod day22 {

    use super::*;

    #[test]
    fn part_1() {
        let input = "on x=-20..26,y=-36..17,z=-47..7\non x=-20..33,y=-21..23,z=-26..28\non x=-22..28,y=-29..23,z=-38..16\non x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28\non x=2..47,y=-22..22,z=-23..27\non x=-27..23,y=-28..26,z=-21..29\non x=-39..5,y=-6..47,z=-3..44\non x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19\noff x=-48..-32,y=26..41,z=-47..-37\non x=-12..35,y=6..50,z=-50..-2\noff x=-48..-32,y=-32..-16,z=-15..-5\non x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41\non x=-16..35,y=-41..10,z=-47..6\noff x=-32..-23,y=11..30,z=-14..3\non x=-49..-5,y=-3..45,z=-29..18\noff x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15\non x=-54112..-39298,y=-85059..-49293,z=-27449..7877\non x=967..23432,y=45373..81175,z=27513..53682";
        assert_eq!(part1(input), 590784);
    }
    #[test]
    #[ignore]
    fn part_2() {
        assert_eq!(part2(""), "");
    }
}
