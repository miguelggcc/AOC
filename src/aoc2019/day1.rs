pub fn part1(input: &str) -> u32 {
    input.lines().map(|l|l.parse::<u32>().unwrap()/3-2).sum()
}

pub fn part2(input: &str) -> u32 {
    input.lines().map(|l|{
        let mut m = l.parse::<i32>().unwrap()/3-2;
        let mut f = 0;
        while m>0{
            f+=m as u32;
            m = m/3-2;
        }
        f
}).sum()
}

#[cfg(test)]
mod day1 {

    use super::*;

    const INPUT: &'static str = "100756";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 33583);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 50346);
    }
}