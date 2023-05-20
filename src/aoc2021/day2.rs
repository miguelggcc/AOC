use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete,
    combinator::{all_consuming, map},
    sequence::preceded,
    Finish, IResult,
};

pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| all_consuming(parse)(line).finish().unwrap().1)
        .fold([0, 0], |mut pos, c| {
            match c {
                Command::Forward(f) => pos[0] += f,
                Command::Down(d) => pos[1] += d,
                Command::Up(u) => pos[1] -= u,
            };
            pos
        })
        .iter()
        .product()
}

pub fn part2(_input: &str) -> String {
    String::from("Not implemented")
}

fn parse(input: &str) -> IResult<&str, Command> {
    alt((
        map(preceded(tag("forward "), complete::i32), Command::Forward),
        map(preceded(tag("down "), complete::i32), Command::Down),
        map(preceded(tag("up "), complete::i32), Command::Up),
    ))(input)
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[cfg(test)]
mod day2 {

    use super::*;

    const INPUT: &'static str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 150);
    }
    #[test]
    #[ignore]
    fn part_2() {
        assert_eq!(part2(INPUT), "");
    }
}
