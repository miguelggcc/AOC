use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_till, take_until},
    character::complete::{self, line_ending, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};

pub fn part1(input: &str) -> impl std::fmt::Display {
    let (seeds, maps) = parse(input).unwrap().1;
    seeds
        .into_iter()
        .map(|s| get_location(s, &maps))
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let (seeds, maps) = parse(input).unwrap().1;
    seeds
        .into_iter()
        .tuples()
        .map(|(min, l)| find_min(min, min + l, &maps))
        .min()
        .unwrap()
}

fn get_location(s: u64, maps: &[Vec<Range>]) -> u64 {
    maps.iter().fold(s, |acc, ranges| {
        ranges
            .iter()
            .find_map(|r| {
                (acc >= r.source && acc < r.source + r.len).then(|| acc - r.source + r.dest)
            })
            .unwrap_or(acc)
    })
}

fn find_min(low: u64, high: u64, maps: &[Vec<Range>]) -> u64 {
    let len = high - low;
    let low_location = get_location(low, maps);
    if len == 1 {
        return low_location;
    }
    let high_location = get_location(high, maps);
    if high_location.saturating_sub(low_location) == len {
        // linear with no gaps
        low_location
    } else {
        let mid = (low + high) / 2;
        find_min(low, mid, maps).min(find_min(mid, high, maps))
    }
}

fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<Vec<Range>>)> {
    let (input, seeds) = delimited(
        tag("seeds: "),
        separated_list1(space1, complete::u64),
        line_ending,
    )(input)?;
    let (input, _) = take_till(|c: char| c == 's')(input)?;
    let (input, maps) = separated_list1(pair(line_ending, line_ending), parse_map)(input)?;
    Ok((input, (seeds, maps)))
}

fn parse_map(input: &str) -> IResult<&str, Vec<Range>> {
    let (input, _) = terminated(take_until("\n"), line_ending)(input)?;
    separated_list1(
        line_ending,
        map(
            tuple((complete::u64, space1, complete::u64, space1, complete::u64)),
            |(dest, _, source, _, len)| Range { dest, source, len },
        ),
    )(input)
}

#[derive(Copy, Clone, Debug)]
struct Range {
    dest: u64,
    source: u64,
    len: u64,
}

#[cfg(test)]
mod day5 {

    use super::*;

    const INPUT: &'static str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "35");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "46");
    }
}
