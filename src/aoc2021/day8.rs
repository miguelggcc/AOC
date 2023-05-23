use std::collections::HashSet;

use nom::{
    character::complete,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    Finish, IResult,
};

pub fn part1(input: &str) -> usize {
    let data = input.lines().map(|line| parse(line).finish().unwrap().1);
    data.map(|(_, outputs)| {
        outputs
            .iter()
            .filter(|o| o.len() <= 4 || o.len() == 7)
            .count()
    })
    .sum()
}
pub fn part2(input: &str) -> u32 {
    let data = input.lines().map(|line| parse(line).finish().unwrap().1);
    let mut seg = [0; 10];
    data.map(|(signals, outputs)| {
        let digits: HashSet<_> = HashSet::from_iter(signals.iter().chain(&outputs));
        let (len6, digits): (Vec<_>, Vec<_>) = digits.into_iter().partition(|n| n.len() == 6);
        let mut common_len6 = common_list(len6.into_iter().map(to_num).collect::<Vec<_>>());
        seg[1] = to_num(digits.iter().find(|n| n.len() == 2).expect("no '1' found"));
        seg[7] = to_num(digits.iter().find(|n| n.len() == 3).expect("no '7' found"));
        seg[4] = to_num(digits.iter().find(|n| n.len() == 4).expect("no '4' found"));
        seg[8] = to_num(digits.iter().find(|n| n.len() == 7).expect("no '8' found"));

        let top = seg[7] / seg[1];
        let down_right = common(seg[1], common_len6);
        let up_right = seg[1] / down_right;
        common_len6 /= down_right;
        let up_left = common(seg[4], common_len6);
        let center = seg[4] / (seg[1] * up_left);
        let bottom = common_len6 / (top * up_left);
        seg[0] = seg[8] / center;
        seg[2] = seg[8] / (down_right * up_left);
        seg[3] = bottom * down_right * center * up_right * top;
        seg[5] = bottom * down_right * center * up_left * top;
        seg[6] = seg[8] / up_right;
        seg[9] = top * center * bottom * up_right * down_right * up_left;

        outputs.iter().fold(0, |acc, d| {
            acc * 10 + seg.iter().position(|&s| to_num(d) == s).unwrap() as u32
        })
    })
    .sum()
}

const PRIMES: [u32; 7] = [2, 3, 5, 7, 11, 13, 17];

fn to_num(n: &str) -> u32 {
    n.chars()
        .fold(1, |acc, c| acc * PRIMES[(c as u8 - b'a') as usize])
}

fn common(a: u32, b: u32) -> u32 {
    PRIMES
        .into_iter()
        .find(|x| a % x == 0 && b % x == 0)
        .expect("no common factors")
}

fn common_list(list: Vec<u32>) -> u32 {
    PRIMES.into_iter().fold(1, |acc, x| {
        acc * if list.iter().all(|a| a % x == 0) {
            x
        } else {
            1
        }
    })
}

fn parse(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    separated_pair(
        separated_list1(complete::space1, complete::alpha1),
        delimited(complete::space0, complete::char('|'), complete::space0),
        separated_list1(complete::space1, complete::alpha1),
    )(input)
}

#[cfg(test)]
mod day8 {

    use super::*;

    const INPUT: &'static str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 26);
    }
    #[test]
    #[ignore]
    fn ex_part_2() {
        assert_eq!(part2("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"), 5353);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 61229);
    }
}
