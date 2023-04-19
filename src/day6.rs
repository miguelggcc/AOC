use std::time::Instant;

pub fn day6(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't find input file");
    //Part 1
    let time = Instant::now();
    println!("First marker after character {}", do_day6_part1(&input));
    println!("{:?}", time.elapsed());
    //Part 2
    println!(
        "Part2: First marker after character {}",
        do_day6_part2(&input)
    );
}

fn do_day6_part1(input: &str) -> u32 {
    let chars = input.chars().collect::<Vec<char>>();
    let size = 4;

    chars.windows(size).position(|w| !is_repeated(w)).unwrap() as u32
}

#[inline(always)]
fn is_repeated(c: &[char]) -> bool {
    c.iter()
        .enumerate()
        .any(|(i, &x)| c.iter().skip(i + 1).any(|&y| x == y))
}

fn do_day6_part2(input: &str) -> u32 {
    let chars = input.chars().collect::<Vec<char>>();
    let size = 14;

    chars.windows(size).position(|w| !is_repeated(w)).unwrap() as u32
}

/*#[cfg(test)]
mod tests {
    use super::do_day6_part1;
    use super::do_day6_part2;

    #[test]
    fn part_1() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";

        assert_eq!(do_day6_part1(input), 6);
    }
}*/
