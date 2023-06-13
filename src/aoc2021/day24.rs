use nom::{
    bytes::complete::{tag, take_until},
    character::complete,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};

pub fn part1(input: &str) -> u64 {
    let inst = parse(input).finish().unwrap().1;
    get_nomad(inst, Mode::Max)
}

pub fn part2(input: &str) -> u64 {
    let inst = parse(input).finish().unwrap().1;
    get_nomad(inst, Mode::Min)
}

//This solution assumes that:
//1. Only div z A, add x B, add y C change between inp w blocks
//2. If A=1 then B is always >9, otherwise A=26 and B is <=9
//3. The two cases described in 2 always happen the same number of times

fn get_nomad(inst: Vec<(i8, i8, i8)>, mode: Mode) -> u64 {
    let mut zstack = vec![];
    let mut digits = [mode.clone() as i8; 14];
    for (i, v) in inst.iter().enumerate() {
        if v.1 > 9 && v.0 == 1 {
            zstack.push((v.2, i))
        } else {
            let (offset, other_i) = zstack.pop().unwrap();
            let delta = match mode {
                Mode::Max => -offset - v.1,
                Mode::Min => offset + v.1,
            };
            if delta > 0 {
                digits[i] += delta;
            } else {
                digits[other_i] -= delta;
            }
        }
    }
    digits
        .into_iter()
        .fold(0, |n, d| n * 10 + (d as i64).unsigned_abs())
}

#[derive(Clone)]
enum Mode {
    Max = -9,
    Min = 1,
}

fn parse(input: &str) -> IResult<&str, Vec<(i8, i8, i8)>> {
    separated_list1(tag("\r\nmul y x\r\nadd z y"), parse_block)(input)
}

fn parse_block(input: &str) -> IResult<&str, (i8, i8, i8)> {
    let (input, _) = terminated(take_until("div z "), tag("div z "))(input)?;
    let (input, (a, b)) = separated_pair(complete::i8, tag("\r\nadd x "), complete::i8)(input)?;
    let (input, _) = terminated(take_until("w\r\nadd y "), tag("w\r\nadd y "))(input)?;
    let (input, c) = complete::i8(input)?;
    Ok((input, (a, b, c)))
}

//No tests since there's no example case
