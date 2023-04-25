use std::{cmp::Ordering, time::Instant};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::{all_consuming, map},
    multi::{separated_list0, separated_list1},
    sequence::delimited,
    Finish, IResult,
};

pub fn day13(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't read input file");
    let time = Instant::now();
    //Part 1
    println!(
        "Sum of pairs in the right order: {}",
        do_day13_part1(&input)
    );
    //Part 2
    println!("Part 2, decoder key: {}", do_day13_part2(&input));
    println!("{:?}", time.elapsed());
}

fn do_day13_part1(input: &str) -> u32 {
    let pairs = all_consuming(separated_list1(
        alt((tag("\n\n"), tag("\r\n\r\n"))),
        parse_pair,
    ))(input)
    .finish()
    .unwrap()
    .1;

    pairs
        .into_iter()
        .enumerate()
        .map(|(i, pair)| if pair.0 < pair.1 { i as u32 + 1 } else { 0 })
        .sum()
}

fn do_day13_part2(input: &str) -> u32 {
    let mut lists = all_consuming(separated_list1(
        alt((tag("\n\n"), tag("\r\n\r\n"), tag("\n"), tag("\r\n"))),
        parse_list,
    ))(input)
    .finish()
    .unwrap()
    .1;

    let list_two = List::L(vec![List::L(vec![List::I(2)])]);
    lists.push(list_two.clone());
    let list_six = List::L(vec![List::L(vec![List::I(6)])]);
    lists.push(list_six.clone());

    lists.sort_by(|l1, l2| l1.partial_cmp(l2).unwrap());

    (lists
        .iter()
        .position(|l| l == &list_two)
        .map(|p| p + 1)
        .expect("Couldn't find [[2]]")
        * lists
            .iter()
            .position(|l| *l == list_six)
            .map(|p| p + 1)
            .expect("Couldn't find [[6]]")) as u32
}

impl PartialOrd for List {
    fn partial_cmp(&self, right: &List) -> Option<Ordering> {
        match (self, right) {
            (List::I(i1), List::I(i2)) => {
                if i1 == i2 {
                    return None;
                }
                i1.partial_cmp(i2)
            }
            (l, r) => l.map_slice(|v1| {
                r.map_slice(|v2| {
                    for (l1, l2) in v1.iter().zip(v2) {
                        let b = l1.partial_cmp(l2);
                        if b.is_none() {
                            continue;
                        }
                        return b;
                    }
                    if v1.len() == v2.len() {
                        return None;
                    }
                    v1.len().partial_cmp(&v2.len())
                })
            }),
        }
    }
}

fn parse_pair(input: &str) -> IResult<&str, (List, List)> {
    let (input, list1) = (parse_list)(input)?;
    let (input, _) = alt((tag("\n"), tag("\r\n")))(input)?;
    let (input, list2) = (parse_list)(input)?;
    Ok((input, (list1, list2)))
}

fn parse_list(input: &str) -> IResult<&str, List> {
    let parse_list = map(
        delimited(
            complete::char('['),
            separated_list0(complete::char(','), parse_list),
            complete::char(']'),
        ),
        List::L,
    );
    let parse_integer = map(complete::u8, List::I);

    alt((parse_list, parse_integer))(input)
}

#[derive(Debug, PartialEq, Clone)]
enum List {
    L(Vec<List>),
    I(u8),
}

impl List {
    fn map_slice<T>(&self, f: impl FnOnce(&[List]) -> T) -> T {
        match self {
            Self::L(v) => f(&v[..]),
            Self::I(i) => f(&[Self::I(*i)]),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::do_day13_part1;
    use super::do_day13_part2;

    #[test]
    fn part_1() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(do_day13_part1(input), 13);
        assert_eq!(do_day13_part2(input), 140)
    }
}
