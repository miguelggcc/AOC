use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn part1(input: &str) -> u64 {
    let reactions = parse(input);
    let mut leftovers = HashMap::new();
    n_of_ores("FUEL", 1, &reactions, &mut leftovers)
}

pub fn part2(input: &str) -> u64 {
    let reactions = parse(input);
    let mut leftovers = HashMap::new();

    let mut fuel = 1;
    let target = 1_000_000_000_000;
    loop {
        leftovers.clear();
        let ore = n_of_ores("FUEL", fuel as u64 + 1, &reactions, &mut leftovers) as u128;
        if ore > target {
            return fuel as u64;
        } else {
            fuel = (fuel + 1).max((fuel + 1) * target / ore);
        }
    }
}

fn n_of_ores<'a>(
    parent: &'a str,
    needed: u64,
    reactions: &'a Reactions,
    leftovers: &mut HashMap<&'a str, u64>,
) -> u64 {
    if let Some((np, reacts)) = reactions.get(parent) {
        let quantity = match leftovers.entry(parent) {
            Entry::Occupied(mut o) => {
                let diff = needed.saturating_sub(*o.get());
                *o.get_mut() = o.get_mut().saturating_sub(needed);
                diff
            }
            Entry::Vacant(_) => needed,
        };
        if quantity == 0 {
            return 0;
        }

        let n = quantity / np + (quantity % np).min(1); //same as .ceil()
        *leftovers.entry(parent).or_insert(0) += n * np - quantity;

        return reacts
            .iter()
            .map(|(nc, r)| n_of_ores(r, *nc * n, reactions, leftovers))
            .sum();
    }
    needed
}

type Reactions<'a> = HashMap<&'a str, (u64, Vec<(u64, &'a str)>)>;
fn parse(input: &str) -> Reactions {
    let mut reactions = HashMap::new();
    for l in input.lines() {
        let (lhs, rhs) = l.split_once(" => ").unwrap();
        let (n, id) = parse_pair(rhs);
        let reacts: Vec<_> = lhs.split(", ").map(parse_pair).collect();
        reactions.insert(id, (n, reacts));
    }
    reactions
}

fn parse_pair(p: &str) -> (u64, &str) {
    let (n, id) = p.split_once(' ').unwrap();
    (n.parse::<u64>().unwrap(), id)
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
