use std::time::Instant;

use nom::{branch::alt, IResult, combinator::{map, all_consuming}, bytes::complete::tag, character::complete, sequence::preceded, Finish};

pub fn day10(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't read input file");
    let time = Instant::now();
    //Part 1
    println!("Number of pos. visited: {}", do_day10_part1(&input));
    //Part 2
//println!("Part 2, number of pos. visited: {}", do_day10_part2(&input));
println!("{:?}", time.elapsed());
}

fn do_day10_part1(input: &str)->u32{
    let lines = input
    .lines()
    .map(|line| all_consuming(parse_line)(line).finish().unwrap().1);
    for l in lines{
        dbg!(l);
    }
0
}

fn parse_line(input: &str)->IResult<&str, Command>{
    alt((map(tag("noop"), |_: &str|Command::Noop), 
        map(preceded(tag("addx "), complete::i32), |a: i32|Command::Addx(a))))(input)
}

#[derive(Debug)]
enum Command{
    Noop,
    Addx(i32)
}

#[cfg(test)]
mod tests {

    use super::do_day10_part1;

    #[test]
    fn part_1() {
let input = "addx 15
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

assert_eq!(do_day10_part1(input), 13);
    }
}