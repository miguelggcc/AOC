use nom::branch::alt;
use nom::combinator::{all_consuming, map};
use nom::sequence::pair;
use nom::{
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::{preceded, tuple},
};
use std::collections::VecDeque;
use std::time::Instant;

use nom::{Finish, IResult};

pub fn day11(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't read input file");
    let time = Instant::now();
    //Part 1
    println!("Monkey business for 20 rounds: {}", do_day11_part1(&input));
    //Part 2
    println!("Monkey business for 10k rounds: {}", do_day11_part2(&input));
    println!("{:?}", time.elapsed());
}

fn do_day11_part1(input: &str) -> u64 {
    let mut monkeys = all_consuming(separated_list1(
        alt((tag("\n\n"), tag("\r\n\r\n"))),
        parse_monkey,
    ))(input)
    .finish()
    .unwrap()
    .1;
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let monkey = monkeys.get_mut(i).unwrap();
                monkey.times_inspected += 1;
                let mut item = monkey.operation.operate(item);
                item /= 3;
                let throw_to = if item % monkey.test_divisible == 0 {
                    monkey.throw_to_if_true
                } else {
                    monkey.throw_to_if_false
                };
                monkeys.get_mut(throw_to).unwrap().items.push_back(item);
            }
        }
    }
    monkeys
        .iter()
        .fold([0; 2], |mut max, monkey| {
            let times = monkey.times_inspected;
            if times > max[0] {
                max[1] = max[0];
                max[0] = times;
            } else if times > max[1] {
                max[1] = times;
            }
            max
        })
        .iter()
        .product()
}

fn do_day11_part2(input: &str) -> u64 {
    let mut monkeys = all_consuming(separated_list1(
        alt((tag("\n\n"), tag("\r\n\r\n"))),
        parse_monkey,
    ))(input)
    .finish()
    .unwrap()
    .1;

    let divisor: u64 = monkeys.iter().map(|monkey| monkey.test_divisible).product();
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let monkey = monkeys.get_mut(i).unwrap();
                monkey.times_inspected += 1;
                let item = item % divisor;
                let item = monkey.operation.operate(item);
                let throw_to = if item % monkey.test_divisible == 0 {
                    monkey.throw_to_if_true
                } else {
                    monkey.throw_to_if_false
                };
                monkeys.get_mut(throw_to).unwrap().items.push_back(item);
            }
        }
    }
    monkeys
        .iter()
        .fold([0; 2], |mut max, monkey| {
            let times = monkey.times_inspected;
            if times > max[0] {
                max[1] = max[0];
                max[0] = times;
            } else if times > max[1] {
                max[1] = times;
            }
            max
        })
        .iter()
        .product()
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tuple((tag("Monkey "), complete::u64, tag(":")))(input)?;
    let (input, items) = preceded(
        pair(complete::multispace1, tag("Starting items: ")),
        separated_list1(tag(", "), complete::u64),
    )(input)?;
    let (input, operation) = preceded(
        pair(complete::multispace1, tag("Operation: new = ")),
        parse_operation,
    )(input)?;
    let (input, test_divisible) = preceded(
        pair(complete::multispace1, tag("Test: divisible by ")),
        complete::u64,
    )(input)?;
    let (input, throw_to_if_true) = preceded(
        pair(complete::multispace1, tag("If true: throw to monkey ")),
        complete::u64,
    )(input)?;
    let (input, throw_to_if_false) = preceded(
        pair(complete::multispace1, tag("If false: throw to monkey ")),
        complete::u64,
    )(input)?;
    Ok((
        input,
        Monkey {
            items: VecDeque::from(items),
            operation,
            test_divisible,
            throw_to_if_true: throw_to_if_true as usize,
            throw_to_if_false: throw_to_if_false as usize,
            times_inspected: 0,
        },
    ))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let parser_mul = map(preceded(tag("old * "), complete::u64), |m| {
        Operation::Multiply(m)
    });
    let parser_sum = map(preceded(tag("old + "), complete::u64), |s| {
        Operation::Sum(s)
    });
    let parser_square = map(tag("old * old"), |_| Operation::Square);
    alt((parser_mul, parser_sum, parser_square))(input)
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test_divisible: u64,
    throw_to_if_true: usize,
    throw_to_if_false: usize,
    times_inspected: u64,
}

#[derive(Debug)]
enum Operation {
    Sum(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn operate(&self, x: u64) -> u64 {
        match self {
            Self::Sum(s) => x + s,
            Self::Multiply(m) => x * m,
            Self::Square => x * x,
        }
    }
}
#[cfg(test)]
mod tests {

    use super::do_day11_part1;
    use super::do_day11_part2;

    #[test]
    fn part_1() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        assert_eq!(do_day11_part1(input), 10605);
        assert_eq!(do_day11_part2(input), 2713310158)
    }
}
