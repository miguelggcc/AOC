use nom::{
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete,
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::{preceded, terminated},
    IResult,
};

pub fn part1(input: &str) -> u32 {
    let (numbers, mut boards) = parse(input).unwrap().1;
    for number in numbers {
        for b in boards.iter_mut() {
            if let Some(index) = b.iter().position(|n| n == &number) {
                b[index] = -number - 1;
                let x = index % 5;
                let y = index / 5;
                let check_row = b[y * 5..y * 5 + 5].iter().all(|n| n.is_negative());
                let check_column = b.iter().skip(x).step_by(5).all(|n| n.is_negative());
                if check_row || check_column {
                    return get_score(b, number);
                }
            }
        }
    }
    panic!("not a single bingo")
}

pub fn part2(input: &str) -> u32 {
    let (numbers, mut boards) = parse(input).unwrap().1;
    let mut len = boards.len();
    let mut last = 0;
    for number in numbers {
        boards.retain_mut(|b| {
            if let Some(index) = b.iter().position(|n| n == &number) {
                b[index] = -number - 1;
                let x = index % 5;
                let y = index / 5;
                let check_row = b[y * 5..y * 5 + 5].iter().all(|n| n.is_negative());
                let check_column = b.iter().skip(x).step_by(5).all(|n| n.is_negative());
                if check_row || check_column {
                    if len == 1 {
                        last = get_score(&b, number);
                    }
                    false
                } else {
                    true
                }
            } else {
                true
            }
        });
        len = boards.len();
    }
    last
}

fn get_score(b: &[i8], number: i8) -> u32 {
    b.into_iter()
        .filter(|n| n.is_positive())
        .map(|n| *n as u32)
        .sum::<u32>()
        * number as u32
}

fn parse(input: &str) -> IResult<&str, (Vec<i8>, Vec<Vec<i8>>)> {
    let (input, numbers) = terminated(
        separated_list1(complete::char(','), complete::i8),
        complete::multispace1,
    )(input)?;
    let (input, boards) = separated_list1(alt((tag("\n\n"), tag("\r\n\r\n"))), parse_board)(input)?;
    Ok((input, (numbers, boards)))
}

fn parse_board(input: &str) -> IResult<&str, Vec<i8>> {
    many1(preceded(
        permutation((opt(complete::line_ending), complete::space0)),
        complete::i8,
    ))(input)
}

#[cfg(test)]
mod day4 {

    use super::*;

    const INPUT: &'static str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 4512);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 1924);
    }
}
