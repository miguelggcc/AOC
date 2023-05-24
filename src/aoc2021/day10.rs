pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|line| {
            let mut stack = vec![];
            for c in line.trim().chars() {
                match c {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    _ => {
                        if Some(c) != stack.pop() {
                            return match c {
                                ')' => Some(3),
                                ']' => Some(57),
                                '}' => Some(1197),
                                _ => Some(25137),
                            };
                        }
                    }
                };
            }
            None
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let mut scores: Vec<_> = input
        .lines()
        .flat_map(|line| {
            let mut stack = vec![];
            for c in line.trim().chars() {
                match c {
                    '(' => stack.push(1),
                    '[' => stack.push(2),
                    '{' => stack.push(3),
                    '<' => stack.push(4),
                    _ => {
                        if c != CLOSERS[stack.pop().unwrap() - 1] {
                            return None;
                        }
                    }
                };
            }
            Some(stack)
        })
        .map(|stack| stack.iter().rev().fold(0, |acc, points| acc * 5 + points))
        .collect();
    scores.sort();
    scores[scores.len() / 2] as u64
}
const CLOSERS: [char; 4] = [')', ']', '}', '>'];

#[cfg(test)]
mod day10 {

    use super::*;

    const INPUT: &'static str = "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 26397);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 288957);
    }
}
