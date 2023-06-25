use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let reactions = parse(input);
    let mut leftovers = HashMap::new();
    n_of_ores("FUEL", 1, &reactions, &mut leftovers)
}

pub fn part2(input: &str) -> usize {
    let reactions = parse(input);
    let mut leftovers: HashMap<_, _> = HashMap::new();

    binary_search(0, 100_000_000, |n| {
        leftovers.clear();
        n_of_ores("FUEL", n, &reactions, &mut leftovers).cmp(&1_000_000_000_000)
    })
}

fn n_of_ores<'a>(
    parent: &'a str,
    acc: usize,
    reactions: &'a Reactions,
    leftovers: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some((np, reacts)) = reactions.get(parent) {
        let needed = match leftovers.entry(parent) {
            Entry::Occupied(mut o) => {
                let diff = acc.saturating_sub(*o.get());
                *o.get_mut() = o.get_mut().saturating_sub(acc);
                diff
            }
            Entry::Vacant(_) => acc,
        };
        if needed > 0 {
            let n = needed / np + (needed % np).min(1);
            let produced = n * np;
            if produced > needed {
                *leftovers.entry(parent).or_insert(0) += produced - needed;
            }
            return reacts
                .iter()
                .map(|(nc, r)| n_of_ores(r, *nc * n, reactions, leftovers))
                .sum();
        } else {
            return 0;
        }
    }
    acc
}

fn binary_search(mut start: usize, mut end: usize, mut f: impl FnMut(usize) -> Ordering) -> usize {
    while start <= end {
        let mid = (end + start) / 2;
        match f(mid) {
            Ordering::Greater => end = mid - 1,
            Ordering::Less => start = mid + 1,
            Ordering::Equal => return mid,
        }
    }
    return start - 1;
}

type Reactions<'a> = HashMap<&'a str, (usize, Vec<(usize, &'a str)>)>;
fn parse(input: &str) -> Reactions {
    let mut reactions = HashMap::new();
    for l in input.lines() {
        let (lhs, rhs) = l.split_once(" => ").unwrap();
        let (n, id) = parse_pair(rhs);
        let reacts: Vec<_> = lhs.split(", ").map(|p| parse_pair(p)).collect();
        reactions.insert(id, (n, reacts));
    }
    reactions
}

fn parse_pair(p: &str) -> (usize, &str) {
    let (n, id) = p.split_once(' ').unwrap();
    (n.parse::<usize>().unwrap(), id)
}

#[cfg(test)]
mod day14 {

    use super::*;

    const INPUT: &'static str = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 13312);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 82892753);
    }
}
