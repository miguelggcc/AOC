use nom::{
    bytes::complete::{tag, take_until},
    character::complete,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};

pub fn part1(input: &str) -> u64 {
    let inst = parse(input).finish().unwrap().1;
    let mut zstack = vec![];
    let mut digits = [9; 14];
    for (i, v) in inst.iter().enumerate() {
        if v.1 > 9 && v.0 == 1 {
            zstack.push((v.2, i))
        } else {
            let (offset, other_i) = zstack.pop().unwrap();
            let delta = offset + v.1;
            if delta > 0 {
                digits[other_i] -= delta;
            } else {
                digits[i] += delta;
            }
        }
    }
    digits.into_iter().fold(0, |acc, d| acc * 10 + d as u64)
}

pub fn part2(input: &str) -> u64 {
    let inst = parse(input).finish().unwrap().1;
    let mut zstack = vec![];
    let mut digits = [1; 14];
    for (i, v) in inst.iter().enumerate() {
        if v.1 > 9 && v.0 == 1 {
            zstack.push((v.2, i))
        } else {
            let (offset, other_i) = zstack.pop().unwrap();
            let delta = offset + v.1;
            if delta > 0 {
                digits[i] += delta;
            } else {
                digits[other_i] -= delta;
            }
        }
    }
    digits.into_iter().fold(0, |acc, d| acc * 10 + d as u64)
}

fn parse(input: &str) -> IResult<&str, Vec<(i32, i32, i32)>> {
    separated_list1(tag("\r\nmul y x\r\nadd z y"), parse_block)(input)
}

fn parse_block(input: &str) -> IResult<&str, (i32, i32, i32)> {
    let (input, _) = terminated(take_until("div z "), tag("div z "))(input)?;
    let (input, (a, b)) = separated_pair(complete::i32, tag("\r\nadd x "), complete::i32)(input)?;
    let (input, _) = terminated(take_until("w\r\nadd y "), tag("w\r\nadd y "))(input)?;
    let (input, c) = complete::i32(input)?;
    Ok((input, (a, b, c)))
}

//no tests since there's no example case
