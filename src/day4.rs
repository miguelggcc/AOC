pub fn day4(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't find input file");
    //Part 1
    println!("Total is {}", do_day4_part1(&input));
    //Part 2
    println!("Total for part 2 is {}", do_day4_part2(&input));
}

fn do_day4_part1(input: &str) -> u32 {
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

fn do_day4_part2(input: &str) -> u32 {
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
    use super::do_day4_part1;
    use super::do_day4_part2;

    #[test]
    fn part_1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(do_day4_part1(input), 2);
        assert_eq!(do_day4_part2(input), 4);
    }
}
