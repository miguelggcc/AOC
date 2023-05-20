pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let pairs: Vec<&str> = l.split(',').collect();
            let values_left: Vec<u32> = pairs[0]
                .split('-')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            let values_right: Vec<u32> = pairs[1]
                .split('-')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            u32::from(
                (values_left[0] >= values_right[0] && values_left[1] <= values_right[1])
                    || (values_right[0] >= values_left[0] && values_right[1] <= values_left[1]),
            )
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let pairs: Vec<&str> = l.split(',').collect();
            let values_left: Vec<u32> = pairs[0]
                .split('-')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            let values_right: Vec<u32> = pairs[1]
                .split('-')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            u32::from(
                (values_left[0] >= values_right[0] && values_left[0] <= values_right[1])
                    || (values_right[0] >= values_left[0] && values_right[0] <= values_left[1]),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_2() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(part1(input), 2);
        assert_eq!(part2(input), 4);
    }
}
