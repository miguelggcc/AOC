use std::collections::HashMap;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().cycle().enumerate();
    let mut nodes = HashMap::new();

    for line in lines.skip(1) {
        let (node, dirs) = line.split_once(" = (").unwrap();
        let directions = dirs[..dirs.len() - 1].split_once(", ").unwrap();
        nodes.insert(node, directions);
    }
    get_steps("AAA", |n| n == "ZZZ", &nodes, instructions).0
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().cycle().enumerate();
    let mut nodes = HashMap::new();

    for line in lines.skip(1) {
        let (node, dirs) = line.split_once(" = (").unwrap();
        let directions = dirs[..dirs.len() - 1].split_once(", ").unwrap();
        nodes.insert(node, directions);
    }
    //let mut node:Vec<&str> = nodes.keys().filter(|n|n.ends_with('A')).copied().collect();
    nodes
        .keys()
        .filter(|n| n.ends_with('A'))
        .copied()
        .fold(1, |acc, node| {
            let (i0, nodez) = get_steps(node, |n| n.ends_with('Z'), &nodes, instructions.clone());
            let repeating = get_steps(
                nodez,
                |n| n.ends_with('Z'),
                &nodes,
                instructions.clone().skip(i0),
            )
            .0 - i0;
            lcm(acc, repeating)
        })
}

fn get_steps<'a>(
    mut node: &'a str,
    end: impl Fn(&str) -> bool,
    nodes: &HashMap<&str, (&'a str, &'a str)>,
    instructions: impl Iterator<Item = (usize, char)>,
) -> (usize, &'a str) {
    for (i, ins) in instructions {
        node = match ins {
            'L' => nodes[node].0,
            _ => nodes[node].1,
        };
        if end(node) {
            return (i + 1, node);
        }
    }
    unreachable!()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod day8 {

    use super::*;

    const INPUT1: &'static str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &'static str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT1).to_string(), "6");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT2).to_string(), "6");
    }
}
