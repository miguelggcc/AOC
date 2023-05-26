use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

pub fn part1(input: &str) -> usize {
    let (mut points, folds) = parse(input);
    fold(&mut points, folds[0].clone())
    .len()
}

pub fn part2(input: &str) -> String {
    let (mut points, folds) = parse(input);

    for f in folds {
        points = fold(&mut points, f);
    }
    let (xmax, ymax) = points
        .iter()
        .fold((0, 0), |(xmax, ymax), &(x, y)| (x.max(xmax), y.max(ymax)));
    let mut out = String::with_capacity(((1 + xmax) * ymax) as usize);
    for y in 0..=ymax {
        for x in 0..=xmax {
            out.push(if points.contains(&(x, y)) { '#' } else { '.' });
        }
        out.push('\n');
    }
    out
}

fn fold(points: &mut HashSet<(u16, u16)>, f: Fold) -> HashSet<(u16, u16)> {
    match f {
        Fold::X(x) => {
            let (mut left, mut right): (HashSet<_>, _) = points.drain().partition(|p| p.0 <= x);
            right.drain().for_each(|p| {
                left.insert((2 * x - p.0, p.1));
            });
            left
        }
        Fold::Y(y) => {
            let (mut up, mut down): (HashSet<_>, _) = points.drain().partition(|p| p.1 <= y);
            down.drain().for_each(|p| {
                up.insert((p.0, 2 * y - p.1));
            });
            up
        }
    }
}

#[derive(Clone)]
enum Fold {
    X(u16),
    Y(u16),
}

fn parse(input: &str) -> (HashSet<(u16, u16)>, Vec<Fold>) {
    let mut lines = input.lines();
    let points: HashSet<_> = (&mut lines)
        .map_while(|l| all_consuming(parse_point)(l).finish().ok().map(|(_, l)| l))
        .collect();

    let folds: Vec<_> = lines
        .map(|l| all_consuming(parse_fold)(l).finish().unwrap().1)
        .collect();
    (points, folds)
}

fn parse_point(input: &str) -> IResult<&str, (u16, u16)> {
    separated_pair(complete::u16, complete::char(','), complete::u16)(input)
}

fn parse_fold(input: &str) -> IResult<&str, Fold> {
    preceded(
        tag("fold along "),
        alt((
            map(preceded(tag("x="), complete::u16), Fold::X),
            map(preceded(tag("y="), complete::u16), Fold::Y),
        )),
    )(input)
}

#[cfg(test)]
mod day13 {

    use super::*;

    const INPUT: &'static str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 17);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), "#####\n#...#\n#...#\n#...#\n#####\n");
    }
}
