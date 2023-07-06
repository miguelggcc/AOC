use nom::{
    branch::alt, bytes::complete::tag, character::complete, combinator::map,
    multi::separated_list1, sequence::preceded, IResult,
};

pub fn part1(input: &str) -> i64 {
    let techniques = parse(input);
    let mut n = 2019;
    let len = 10_007;
    for t in techniques {
        n = match t {
            Tech::NewStack => len - 1 - n,
            Tech::Cut(c) => (n - c).rem_euclid(len),
            Tech::Increment(i) => (n * i) % len,
        };
    }
    n
}

pub fn part2(input: &str) -> i64 {
    let techniques = parse(input);
    let len = 119315717514047;
    let times = 101741582076661i64;
    let mut n = 2020;
    for t in techniques {
        n = match t {
            Tech::NewStack => len - 1 - n,
            Tech::Cut(c) => (n - c).rem_euclid(len),
            Tech::Increment(i) => (n * i) % len,
        };
    }
    n
}

enum Tech {
    NewStack,
    Cut(i64),
    Increment(i64),
}

fn parse(input: &str) -> Vec<Tech> {
    separated_list1(complete::line_ending, parse_technique)(input)
        .unwrap()
        .1
}

fn parse_technique(input: &str) -> IResult<&str, Tech> {
    let parse_ns = map(tag("deal into new stack"), |_| Tech::NewStack);
    let parse_cut = map(preceded(tag("cut "), complete::i64), Tech::Cut);
    let parse_inc = map(
        preceded(tag("deal with increment "), complete::i64),
        Tech::Increment,
    );
    alt((parse_ns, parse_cut, parse_inc))(input)
}

/*#[cfg(test)]
mod day22 {

    use super::*;

    const INPUT: &'static str = "";

    #[test]
    #[ignore]
    fn part_1() {
        assert_eq!(part1(INPUT), "");
    }
    #[test]
    #[ignore]
    fn part_2() {
        assert_eq!(part2(INPUT), "");
    }
}*/
