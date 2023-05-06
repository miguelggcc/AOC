use std::time::Instant;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    character::complete::u32,
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
    Finish, IResult,
};

pub fn day5(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't find input file");
    let time = Instant::now();
    //Part 1
    println!("Final top is {}", do_day5_part1(&input));
    //Part 2
    //println!("Final top for part 2 is {}", do_day5_part2(&input));
    println!("{:?}", time.elapsed());
}

fn do_day5_part1(input: &str) -> u32 {
    /*let moves: Vec<Monkey> = input.lines()
    .map(|line| all_consuming(parse_line)(line).finish().unwrap().1)
    .collect();*/
    0
}

struct Monkey {
    id: String,
    op: Operation,
}

enum Operation {
    Sum(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    N(i32),
}

/*fn parse_line(input: &str) -> IResult<&str, Monkey> {

}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
   let sum_parser = separated_pair()
}*/

fn do_day5_part2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::do_day5_part1;
    use super::do_day5_part2;

    #[test]
    fn part_1() {
        let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

        //assert_eq!(do_day5_part1(input), "CMZ");
        //assert_eq!(do_day5_part2(input), "MCD");
    }
}
