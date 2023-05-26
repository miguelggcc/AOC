use nom::{
    character::complete, combinator::all_consuming, sequence::separated_pair, Finish, IResult,
};
use std::{collections::HashSet, str::Lines};

pub fn part1(input: &str) -> usize {
    let (mut points, mut folds) = parse(input);
    fold(&mut points, folds.next().unwrap()).len()
}

pub fn part2(input: &str) -> String {
    let (mut points, folds) = parse(input);

    for f in folds {
        points = fold(&mut points, f);
    }
    let (xmax, ymax) = points
        .iter()
        .fold((0, 0), |(xmax, ymax), &(x, y)| (x.max(xmax), y.max(ymax)));
    let mut out = String::with_capacity(((xmax + 1) * ymax) as usize);
    for y in 0..=ymax {
        for x in 0..=xmax {
            out.push(if points.contains(&(x, y)) { '#' } else { '.' });
        }
        out.push('\n');
    }
    out
}

fn fold(points: &mut HashSet<(u16, u16)>, f: &str) -> HashSet<(u16, u16)> {
    let mut split = f.split('=');
    match (
        split.next().unwrap().chars().last().unwrap(),
        split.next().unwrap().parse::<u16>().unwrap(),
    ) {
        ('x', x) => {
            let (mut left, mut right): (HashSet<_>, _) = points.drain().partition(|p| p.0 <= x);
            right.drain().for_each(|p| {
                left.insert((2 * x - p.0, p.1));
            });
            left
        }
        ('y', y) => {
            let (mut up, mut down): (HashSet<_>, _) = points.drain().partition(|p| p.1 <= y);
            down.drain().for_each(|p| {
                up.insert((p.0, 2 * y - p.1));
            });
            up
        }
        (e, _) => panic!("Unknown character {e}"),
    }
}

fn parse(input: &str) -> (HashSet<(u16, u16)>, Lines) {
    let mut lines = input.lines();
    let points: HashSet<_> = (&mut lines)
        .map_while(|l| all_consuming(parse_point)(l).finish().ok().map(|(_, l)| l))
        .collect();
    (points, lines)
}

fn parse_point(input: &str) -> IResult<&str, (u16, u16)> {
    separated_pair(complete::u16, complete::char(','), complete::u16)(input)
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
