use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::{
        complete::{self, alpha1},
        is_digit,
    },
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    IResult,
};
use std::{collections::HashMap, ops::Range};

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut lines = input.lines();
    let wfs = get_wfs(&mut lines);

    lines
        .map(|l| parse_part(l).unwrap().1)
        .filter_map(|part| {
            let mut id = "in";
            loop {
                if id == "A" {
                    return Some(part.into_iter().sum::<u32>());
                }
                if id == "R" {
                    return None;
                }
                id = wfs
                    .get(id)
                    .unwrap()
                    .iter()
                    .find_map(|wf| wf.calculate(&part))
                    .unwrap();
            }
        })
        .sum::<u32>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let wfs = get_wfs(&mut input.lines());
    get_ranges(core::array::from_fn(|_| 1..4001), "in", 0, &wfs)
}

type Ranges = [Range<u32>; 4];
fn get_ranges(ranges: Ranges, id: &str, i: usize, wfs: &HashMap<&str, Vec<Workflow>>) -> usize {
    if id == "A" {
        return ranges.into_iter().map(|r| r.len()).product();
    }
    if id == "R" {
        return 0;
    }
    let (new_id, new_ranges) = wfs.get(id).unwrap()[i].calculate_range(ranges.clone());
    if let Some((first_range, second_range)) = new_ranges {
        return get_ranges(first_range, id, i + 1, wfs) + get_ranges(second_range, new_id, 0, wfs);
    }
    get_ranges(ranges, new_id, 0, wfs)
}

fn get_wfs<'a>(lines: &mut std::str::Lines<'a>) -> HashMap<&'a str, Vec<Workflow<'a>>> {
    lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(parse_workflows)
        .collect()
}

enum Workflow<'a> {
    LessThan(usize, u32, &'a str),
    MoreThan(usize, u32, &'a str),
    Id(&'a str),
}

impl Workflow<'_> {
    fn calculate(&self, part: &[u32]) -> Option<&str> {
        match self {
            Workflow::LessThan(cat, value, new_id) => {
                if part[*cat] < *value {
                    return Some(new_id);
                }
                None
            }
            Workflow::MoreThan(cat, value, new_id) => {
                if part[*cat] > *value {
                    return Some(new_id);
                }
                None
            }
            Workflow::Id(new_id) => Some(new_id),
        }
    }
    fn calculate_range(&self, mut ranges: Ranges) -> (&str, Option<(Ranges, Ranges)>) {
        match self {
            Workflow::MoreThan(cat, value, new_id) => {
                let mut ranges1 = ranges.clone();
                ranges[*cat].end = *value + 1;
                ranges1[*cat].start = *value + 1;
                (new_id, Some((ranges, ranges1)))
            }
            Workflow::LessThan(cat, value, new_id) => {
                let mut ranges1 = ranges.clone();
                ranges[*cat].start = *value;
                ranges1[*cat].end = *value;
                (new_id, Some((ranges, ranges1)))
            }
            Workflow::Id(new_id) => (new_id, None),
        }
    }
}

fn parse_workflows(input: &str) -> (&str, Vec<Workflow>) {
    let (id, wfs) = input.split_once('{').unwrap();
    let list: IResult<_, _> = separated_list1(
        tag(","),
        alt((
            map(
                tuple((alpha1, tag(">"), complete::u32, tag(":"), alpha1)),
                |(c, _, n, _, id2)| Workflow::MoreThan(parse_category(c), n, id2),
            ),
            map(
                tuple((alpha1, tag("<"), complete::u32, tag(":"), alpha1)),
                |(c, _, n, _, id2)| Workflow::LessThan(parse_category(c), n, id2),
            ),
            map(terminated(alpha1, tag("}")), Workflow::Id),
        )),
    )(wfs);
    (id, list.unwrap().1)
}

fn parse_part(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(
        tag("{x="),
        separated_list1(take_till(|c| is_digit(c as u8)), complete::u32),
    )(input)
}

fn parse_category(c: &str) -> usize {
    match c {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        _ => 3,
    }
}

#[cfg(test)]
mod day19 {

    use super::*;

    const INPUT: &'static str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "19114");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "167409079868000");
    }
}
