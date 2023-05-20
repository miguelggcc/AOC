use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1},
    combinator::{all_consuming, map},
    sequence::separated_pair,
    Finish, IResult,
};

pub fn part1(input: &str) -> i64 {
    let mut ids = HashMap::new();
    let monkeys: Vec<Monkey> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let monkey = all_consuming(parse_line)(line).finish().unwrap().1;
            ids.insert(monkey.id.to_owned(), i);
            monkey
        })
        .collect();

    calculate("root", &ids, &monkeys)
}

pub fn part2(input: &str) -> i64 {
    let mut ids = HashMap::new();
    let mut monkeys: Vec<Monkey> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let monkey = all_consuming(parse_line)(line).finish().unwrap().1;
            ids.insert(monkey.id.to_owned(), i);
            monkey
        })
        .collect();

    monkeys[*ids.get("root").unwrap()].op = match &monkeys[*ids.get("root").unwrap()].op {
        Operation::Sum(m1, m2)
        | Operation::Sub(m1, m2)
        | Operation::Mul(m1, m2)
        | Operation::Div(m1, m2) => Operation::Sub(m1.to_owned(), m2.to_owned()),
        _ => unreachable!(),
    }; //Replace root's operation with a substraction, because we want to solve a+b*j=c+d*j for j

    let out = tuple_calculate("root", &ids, &monkeys, "humn");
    (out.0 / out.1).round().abs() as i64 //Real (out.0) and imaginary (out.1) numbers have to be equal
}

fn calculate(monkey_id: &str, ids: &HashMap<String, usize>, monkeys: &[Monkey]) -> i64 {
    let monkey = &monkeys[*ids.get(monkey_id).unwrap()];
    match &monkey.op {
        Operation::Sum(m1, m2) => calculate(m1, ids, monkeys) + calculate(m2, ids, monkeys),
        Operation::Sub(m1, m2) => calculate(m1, ids, monkeys) - calculate(m2, ids, monkeys),
        Operation::Mul(m1, m2) => calculate(m1, ids, monkeys) * calculate(m2, ids, monkeys),
        Operation::Div(m1, m2) => calculate(m1, ids, monkeys) / calculate(m2, ids, monkeys),
        Operation::N(n) => *n,
    }
}

fn tuple_calculate(
    monkey_id: &str,
    ids: &HashMap<String, usize>,
    monkeys: &[Monkey],
    humn: &str,
) -> (f64, f64) {
    if monkey_id == humn {
        return (0.0, 1.0);
    }
    let monkey = &monkeys[*ids.get(monkey_id).unwrap()];
    match &monkey.op {
        Operation::Sum(m1, m2) => {
            let t1 = tuple_calculate(m1, ids, monkeys, humn);
            let t2 = tuple_calculate(m2, ids, monkeys, humn);
            (t1.0 + t2.0, t1.1 + t2.1)
        }
        Operation::Sub(m1, m2) => {
            let t1 = tuple_calculate(m1, ids, monkeys, humn);
            let t2 = tuple_calculate(m2, ids, monkeys, humn);
            (t1.0 - t2.0, t1.1 - t2.1)
        }
        Operation::Mul(m1, m2) => {
            let t1 = tuple_calculate(m1, ids, monkeys, humn);
            let t2 = tuple_calculate(m2, ids, monkeys, humn);
            (t1.0 * t2.0 - t1.1 * t2.1, t1.0 * t2.1 + t1.1 * t2.0)
        }
        Operation::Div(m1, m2) => {
            let t1 = tuple_calculate(m1, ids, monkeys, humn);
            let t2 = tuple_calculate(m2, ids, monkeys, humn);
            let den = t2.0 * t2.0 + t2.1 * t2.1;
            (
                (t1.0 * t2.0 + t1.1 * t2.1) / den,
                (-t1.0 * t2.1 + t1.1 * t2.0) / den,
            )
        }
        Operation::N(n) => (*n as f64, 0.0),
    }
}

#[derive(Debug)]
struct Monkey {
    id: String,
    op: Operation,
}

#[derive(Debug)]
enum Operation {
    Sum(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    N(i64),
}

fn parse_line(input: &str) -> IResult<&str, Monkey> {
    map(
        separated_pair(alpha1, tag(": "), parse_operation),
        |(id, op)| Monkey {
            id: id.to_string(),
            op,
        },
    )(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let sum_parser = separated_pair(alpha1, tag(" + "), alpha1);
    let sub_parser = separated_pair(alpha1, tag(" - "), alpha1);
    let mul_parser = separated_pair(alpha1, tag(" * "), alpha1);
    let div_parser = separated_pair(alpha1, tag(" / "), alpha1);
    let n_parser = complete::i64;

    alt((
        map(sum_parser, |(a, b): (&str, &str)| {
            Operation::Sum(a.to_string(), b.to_string())
        }),
        map(sub_parser, |(a, b): (&str, &str)| {
            Operation::Sub(a.to_string(), b.to_string())
        }),
        map(mul_parser, |(a, b): (&str, &str)| {
            Operation::Mul(a.to_string(), b.to_string())
        }),
        map(div_parser, |(a, b): (&str, &str)| {
            Operation::Div(a.to_string(), b.to_string())
        }),
        map(n_parser, Operation::N),
    ))(input)
}

#[cfg(test)]
mod tests {

    const INPUT: &'static str = "root: pppw + sjmn
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

    use super::part1;
    use super::part2;

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 152);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 301);
    }
}
