pub fn part1(input: &str) -> u32 {
    let chars = input.chars().collect::<Vec<char>>();
    let size = 4;

    (chars.windows(size).position(|w| !is_repeated(w)).unwrap() + size) as u32
}

pub fn part2(input: &str) -> u32 {
    let chars = input.chars().collect::<Vec<char>>();
    let size = 14;

    (chars.windows(size).position(|w| !is_repeated(w)).unwrap() + size) as u32
}

#[inline(always)]
fn is_repeated(c: &[char]) -> bool {
    c.iter()
        .enumerate()
        .any(|(i, &x)| c.iter().skip(i + 1).any(|&y| x == y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

        assert_eq!(part1(input), 10);
    }
}
