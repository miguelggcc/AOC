use std::collections::HashMap;

pub fn part1(input: &str) -> u32 {
    let orbits = parse(input);
    get_orbits(0, "COM", &orbits)
}

pub fn part2(input: &str) -> u32 {
    let orbits = parse(input);
    orbital_transfers("COM", "SAN", "YOU", &orbits).unwrap().1
}

fn get_orbits(mut total: u32, parent: &str, orbits: &Orbits) -> u32 {
    if let Some(children) = orbits.get(parent) {
        total += children
            .iter()
            .map(|child| get_orbits(total + 1, child, orbits))
            .sum::<u32>();
    }
    total
}

fn orbital_transfers(parent: &str, san: &str, you: &str, orbits: &Orbits) -> Option<(bool, u32)> {
    if let Some(children) = orbits.get(parent) {
        if children.iter().any(|&c| c == you || c == san) {
            return Some((false, 1));
        }
        let v: Vec<_> = children
            .iter()
            .filter_map(|&c| orbital_transfers(c, san, you, orbits))
            .collect();

        if v.len() > 1 {
            return Some((true, v.iter().map(|&(_, d)| d).sum()));
        }

        if let Some(&(found, d)) = v.first() {
            return Some((found, d + u32::from(!found)));
        }
    }
    None
}

type Orbits<'a> = HashMap<&'a str, Vec<&'a str>>;
fn parse(input: &str) -> Orbits {
    let mut orbits = HashMap::new();
    for (parent, child) in input.lines().map(|l| l.split_once(')').unwrap()) {
        orbits.entry(parent).or_insert(vec![]).push(child);
    }
    orbits
}

#[cfg(test)]
mod day6 {

    use super::*;

    #[test]
    fn part_1() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        assert_eq!(part1(input), 42);
    }
    #[test]
    fn part_2() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
        assert_eq!(part2(input), 4);
    }
}
