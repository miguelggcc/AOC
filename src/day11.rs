use std::time::Instant;

pub fn day11(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't read input file");
    let time = Instant::now();
    //Part 1
    println!("Sum of signal strengths: {}", do_day11_part1(&input));
    //Part 2
    //println!("{}", do_day10_part2(&input));
    println!("{:?}", time.elapsed());
}

fn do_day11_part1(input: &str) -> i32 {
    5
}

fn parse_line(){

}

struct Monkey{
    items: Vec<u32>,
    operation: Operation,
    test: Operation,
    throw_to_if_true: usize,
    throw_to_if_false: usize,
}
enum Operation{
    Sum(u32),
    Multiply(u32),
    Square,
    Divisible(u32)
}
#[cfg(test)]
mod tests {

    use super::do_day11_part1;

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

  assert_eq!(do_day11_part1(input), 13);
    }
}