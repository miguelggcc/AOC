use core::panic;

pub fn day3(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't find input file");
    //Part 1
    println!("Total is {}", do_day3_part1(&input));
    //Part 2
    println!("Total for part 2 is {}", do_day3_part2(&input));
}

fn do_day3_part1(input: &str) -> u32 {
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
        .sum()
}

fn do_day3_part2(input: &str) -> u32 {
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
        .sum()
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
    use super::do_day3_part1;

    #[test]
    fn part_1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(do_day3_part1(input), 157);
    }
}
