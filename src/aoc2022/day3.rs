use core::panic;

pub fn part1(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            let (first_comp, second_comp) = l.split_at(l.len() / 2);
            let mut shared = '0';
            for c in first_comp.chars() {
                if second_comp.contains(c) {
                    shared = c;
                    break;
                }
            }
            letter_to_number(shared)
        })
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    lines
        .chunks(3)
        .map(|l| {
            let mut badge = '0';
            for c in l[0].chars() {
                if l[1].contains(c) && l[2].contains(c) {
                    badge = c;
                    break;
                }
            }
            letter_to_number(badge)
        })
        .sum::<u32>()
        .to_string()
}

fn letter_to_number(c: char) -> u32 {
    let c = c as u32;
    if (97..=122).contains(&c) {
        c - 96
    } else if (65..=90).contains(&c) {
        c - 65 + 27
    } else {
        panic!("shared character not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(part1(input), "157");
    }
}
