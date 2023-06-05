use nom::{
    bytes::complete::tag,
    character::complete::i32,
    sequence::{preceded, separated_pair},
    IResult,
};

pub fn part1(input: &str) -> i32 {
    let ((x0, x1), (y0, y1)) = parse(input).ok().unwrap().1;
    let mut ymax = 0;
    for vx in ((2.0 * x0 as f32).sqrt()) as i32..=x1 {
        for vy in 0..-y0 {
            let mut pos = (0, 0);
            let mut v = (vx, vy);
            while pos.0 <= x1 && pos.1 >= y0 {
                if pos.0 >= x0 && pos.1 <= y1 {
                    ymax = ymax.max(vy * (vy + 1) / 2);
                    break;
                }
                pos = (pos.0 + v.0, pos.1 + v.1);
                v = (v.0 - v.0.signum(), v.1 - 1);
            }
        }
    }
    ymax
}

pub fn part2(input: &str) -> i32 {
    let ((x0, x1), (y0, y1)) = parse(input).ok().unwrap().1;
    let mut total = 0;
    for vx in ((2.0 * x0 as f32).sqrt()) as i32..=x1 {
        for vy in y0..-y0 {
            let mut pos = (0, 0);
            let mut v = (vx, vy);
            while pos.0 <= x1 && pos.1 >= y0 {
                if pos.0 >= x0 && pos.1 <= y1 {
                    total += 1;
                    break;
                }
                pos = (pos.0 + v.0, pos.1 + v.1);
                v = (v.0 - v.0.signum(), v.1 - 1);
            }
        }
    }
    total
}
type Range = ((i32, i32), (i32, i32));
fn parse(input: &str) -> IResult<&str, Range> {
    preceded(
        tag("target area: x="),
        separated_pair(
            separated_pair(i32, tag(".."), i32),
            tag(", y="),
            separated_pair(i32, tag(".."), i32),
        ),
    )(input)
}

#[cfg(test)]
mod day17 {

    use super::*;

    const INPUT: &'static str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 45);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 112);
    }
}
