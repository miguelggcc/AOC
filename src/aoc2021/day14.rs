use std::collections::HashMap;

pub fn part1(input: &str) -> u64 {
    do_steps(input, 10)
}

pub fn part2(input: &str) -> u64 {
    do_steps(input, 40)
}

fn do_steps(input: &str, n_of_steps: usize) -> u64 {
    let mut lines = input.lines();
    let template = lines.next().unwrap().as_bytes();
    assert!(lines.next().unwrap().is_empty());
    let steps: Vec<_> = lines
        .map(|l| {
            let (pair, middle) = l.split_once(" -> ").unwrap();
            let (left, right) = pair.split_at(1);
            let new_pairs = [[left, middle].join(""), [middle, right].join("")];
            (pair, middle, new_pairs)
        })
        .collect();

    let mut pairs = HashMap::with_capacity(26 * 26);
    let mut letters = vec![0; (1 + b'Z' - b'A') as usize];

    template
        .iter()
        .for_each(|c| letters[(c - b'A') as usize] += 1);

    template.windows(2).for_each(|bytes| {
        *pairs
            .entry(String::from_utf8(bytes.to_vec()).unwrap())
            .or_insert(0) += 1;
    });
    let mut old_pairs;
    for _ in 0..n_of_steps {
        pairs.retain(|_, c| *c > 0);
        old_pairs = pairs.clone();
        for (old_pair, middle, new_pairs) in steps.iter() {
            if let Some(count) = old_pairs.get(*old_pair) {
                *pairs.get_mut(*old_pair).unwrap() -= count;
                letters[(middle.as_bytes()[0] - b'A') as usize] += count;
                new_pairs.iter().for_each(|k| {
                    *pairs.entry(k.clone()).or_insert(0) += *count;
                });
            }
        }
    }
    let (min, max) = letters
        .into_iter()
        .filter(|&c| c > 0)
        .fold((u64::MAX, 0), |(min, max), c| (min.min(c), max.max(c)));
    max - min
}

#[cfg(test)]
mod day14 {

    use super::*;

    const INPUT: &'static str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 1588);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 2188189693529);
    }
}
