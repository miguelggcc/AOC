use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, char, space0},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

pub fn part1(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| all_consuming(parse_game)(l).unwrap().1)
        .filter(|game| {
            game.sets
                .iter()
                .all(|s| s.red <= 12 && s.green <= 13 && s.blue <= 14)
        })
        .map(|game| game.id)
        .sum::<u32>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| {
            all_consuming(parse_game)(l)
                .unwrap()
                .1
                .sets
                .iter()
                .fold([0, 0, 0], |acc, s| {
                    [acc[0].max(s.red), acc[1].max(s.green), acc[2].max(s.blue)]
                })
                .iter()
                .product::<u32>()
        })
        .sum::<u32>()
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = delimited(tag("Game "), complete::u32, tag(": "))(input)?;
    let (input, sets) = separated_list1(char(';'), parse_set)(input)?;
    Ok((input, Game { id, sets }))
}

fn parse_set(input: &str) -> IResult<&str, Set> {
    let mut set = Set::default();
    let (input, _) = separated_list1(
        char(','),
        alt((
            map(
                delimited(alt((space0, tag(", "))), complete::u32, tag(" red")),
                |r| set.red = r,
            ),
            map(
                delimited(alt((space0, tag(", "))), complete::u32, tag(" green")),
                |g| set.green = g,
            ),
            map(
                delimited(alt((space0, tag(", "))), complete::u32, tag(" blue")),
                |b| set.blue = b,
            ),
        )),
    )(input)?;
    Ok((input, set))
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

#[derive(Debug, Default)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

#[cfg(test)]
mod day2 {

    use super::*;

    const INPUT: &'static str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "8");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "2286");
    }
}
