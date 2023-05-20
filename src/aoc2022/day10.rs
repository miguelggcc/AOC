use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::{all_consuming, map},
    sequence::preceded,
    Finish, IResult,
};

pub fn part1(input: &str) -> i32 {
    let lines = input
        .lines()
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1);
    let mut x = vec![1];
    for l in lines {
        match l {
            Command::Noop => x.push(*x.last().unwrap()),
            Command::Addx(a) => {
                x.push(*x.last().unwrap());
                x.push(x.last().unwrap() + a);
            }
        }
    }
    x.iter()
        .skip(20 - 1)
        .step_by(40)
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (i as i32 * 40 + 20) * x)
}

pub fn part2(input: &str) -> String {
    let lines = input
        .lines()
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1);
    let mut x_vec = vec![1];
    for l in lines {
        match l {
            Command::Noop => x_vec.push(*x_vec.last().unwrap()),
            Command::Addx(a) => {
                x_vec.push(*x_vec.last().unwrap());
                x_vec.push(x_vec.last().unwrap() + a);
            }
        }
    }
    x_vec.pop();
    x_vec
        .chunks(40)
        .map(|xl| {
            let mut line = xl
                .iter()
                .enumerate()
                .map(|(i, x)| if (i as i32 - x).abs() < 2 { '#' } else { '.' })
                .collect::<String>();
            line.push('\n');
            line
        })
        .collect()
}

fn parse_line(input: &str) -> IResult<&str, Command> {
    alt((
        map(tag("noop"), |_| Command::Noop),
        map(preceded(tag("addx "), complete::i32), |a: i32| {
            Command::Addx(a)
        }),
    ))(input)
}

#[derive(Debug)]
enum Command {
    Noop,
    Addx(i32),
}

#[cfg(test)]
mod tests {

    use super::part1;
    use super::part2;
    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 13140);
    }
    #[test]
    fn part_2() {
        assert_eq!(
            part2(INPUT),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
            .to_string()
        );
    }
}
