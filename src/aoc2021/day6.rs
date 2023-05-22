pub fn part1(input: &str) -> u64 {
    lanternfishes(input, 80)
}

pub fn part2(input: &str) -> u64 {
    lanternfishes(input, 256)
}

fn lanternfishes(input: &str, days: u32) -> u64 {
    let mut ages = input.split(',').fold([0; 9], |mut a, i| {
        a[i.parse::<usize>().unwrap()] += 1;
        a
    });
    for _ in 0..days {
        ages.rotate_left(1);
        ages[6] += ages[8];
    }
    ages.iter().sum()
}

#[cfg(test)]
mod day6 {

    use super::*;

    const INPUT: &'static str = "3,4,3,1,2";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 5934);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 26984457539);
    }
}
