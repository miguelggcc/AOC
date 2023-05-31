use nom::{
    branch::alt,
    character::complete::{self, char},
    combinator::map,
    sequence::{delimited, separated_pair},
    Finish, IResult,
};

pub fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut root_pair = parse_pair(lines.next().unwrap()).finish().unwrap().1;
    while let Some(line) = lines.next() {
        root_pair = Elem::new(root_pair, parse_pair(line).finish().unwrap().1);
        root_pair.reduce();
    }
    root_pair.magnitude()
}

pub fn part2(input: &str) -> u32 {
    let numbers: Vec<_> = input
        .lines()
        .map(|l| parse_pair(l).finish().unwrap().1)
        .collect();
    let mut max = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                let mut pair = Elem::new(numbers[i].clone(), numbers[j].clone());
                pair.reduce();
                max = max.max(pair.magnitude())
            }
        }
    }
    max
}

#[derive(Clone)]
enum Elem {
    Pair(Box<(Elem, Elem)>),
    Value(u8),
}

impl Elem {
    fn new(l: Self, r: Self) -> Self {
        Self::Pair(Box::new((l, r)))
    }
    fn reduce(&mut self) {
        while self.explode(0).is_some() || self.split() {}
    }
    fn explode(&mut self, depth: u8) -> Option<(u8, u8)> {
        match self {
            Self::Pair(p) if depth == 4 => {
                if let (Elem::Value(v0), Elem::Value(v1)) = p.as_ref() {
                    let explode = Some((*v0, *v1));
                    *self = Elem::Value(0);
                    return explode;
                }
            }
            Self::Pair(p) => match (&mut p.0, &mut p.1) {
                (Self::Pair(_), Self::Pair(_)) => {
                    if let Some((v0, v1)) = p.0.explode(depth + 1) {
                        p.1.add_left(v1);
                        return Some((v0, 0));
                    }
                    if let Some((v0, v1)) = p.1.explode(depth + 1) {
                        p.0.add_right(v0);
                        return Some((0, v1));
                    }
                }
                (Self::Value(_), Self::Value(_)) => return None,
                (_, Self::Value(v)) => {
                    if let Some((v0, v1)) = p.0.explode(depth + 1) {
                        *v += v1;
                        return Some((v0, 0));
                    }
                }
                (Self::Value(v), _) => {
                    if let Some((v0, v1)) = p.1.explode(depth + 1) {
                        *v += v0;
                        return Some((0, v1));
                    }
                }
            },
            _ => unreachable!(),
        }
        None
    }

    fn add_left(&mut self, value: u8) {
        match self {
            Self::Pair(p) => p.0.add_left(value),
            Self::Value(v) => *v += value,
        }
    }
    fn add_right(&mut self, value: u8) {
        match self {
            Self::Pair(p) => p.1.add_right(value),
            Self::Value(v) => *v += value,
        }
    }
    fn split(&mut self) -> bool {
        match self {
            Self::Pair(p) => p.0.split() || p.1.split(),
            Self::Value(v) if *v > 9 => {
                *self = Self::new(Self::Value(*v / 2), Self::Value((*v + 1) / 2));
                true
            }
            _ => false,
        }
    }
    fn magnitude(&self) -> u32 {
        match self {
            Self::Pair(p) => 3 * p.0.magnitude() + 2 * p.1.magnitude(),
            Self::Value(v) => *v as u32,
        }
    }
}

fn parse_pair(input: &str) -> IResult<&str, Elem> {
    let parse_pair = map(
        delimited(
            char('['),
            separated_pair(parse_pair, char(','), parse_pair),
            char(']'),
        ),
        |(l, r)| Elem::new(l, r),
    );
    let parse_value = map(complete::u8, Elem::Value);
    alt((parse_pair, parse_value))(input)
}

#[cfg(test)]
mod day18 {

    use super::*;

    const INPUT: &'static str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 4140);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 3993);
    }
}
