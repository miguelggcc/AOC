use nom::bytes::complete::tag;
use nom::sequence::separated_pair;
use nom::{
    branch::alt,
    character::complete::{self, alpha1},
    combinator::map,
    sequence::terminated,
    IResult,
};

pub fn part1(input: &str) -> impl std::fmt::Display {
    input.split(',').map(hash).sum::<usize>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut boxes = vec![vec![]; 256];
    for step_input in input.split(',') {
        let step = parse(step_input).unwrap().1;
        match step {
            Step::Equal(lens) => {
                let b = boxes.get_mut(hash(lens.id)).unwrap();
                if let Some(found_lens) = b.iter_mut().find(|l: &&mut Lens| l.id == lens.id) {
                    found_lens.focal_length = lens.focal_length;
                } else {
                    b.push(lens)
                }
            }
            Step::Dash(id) => {
                let b = boxes.get_mut(hash(id)).unwrap();
                if let Some(i) = b.iter().position(|lens| lens.id == id) {
                    b.remove(i);
                }
            }
        }
    }
    boxes
        .into_iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.into_iter()
                .enumerate()
                .map(move |(j, lens)| (i + 1) * (j + 1) * lens.focal_length)
        })
        .sum::<usize>()
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

fn parse(input: &str) -> IResult<&str, Step> {
    alt((
        map(terminated(alpha1, tag("-")), Step::Dash),
        map(
            separated_pair(alpha1, tag("="), complete::u32),
            |(id, f)| {
                Step::Equal(Lens {
                    id,
                    focal_length: f as usize,
                })
            },
        ),
    ))(input)
}

enum Step<'a> {
    Dash(&'a str),
    Equal(Lens<'a>),
}

#[derive(Clone)]
struct Lens<'a> {
    id: &'a str,
    focal_length: usize,
}
#[cfg(test)]
mod day15 {

    use super::*;

    const INPUT: &'static str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "1320");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "145");
    }
}
