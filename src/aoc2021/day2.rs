pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .fold([0, 0], |mut pos, mut c| {
            match (c.next().unwrap(), c.next().unwrap().parse::<u32>().unwrap()) {
                ("forward", x) => pos[0] += x,
                ("down", y) => pos[1] += y,
                ("up", y) => pos[1] -= y,
                (c, _) => panic!("command {c} doesn't exist"),
            };
            pos
        })
        .iter()
        .product()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .fold(([0, 0], 0), |(mut pos, mut aim), mut c| {
            match (c.next().unwrap(), c.next().unwrap().parse::<u32>().unwrap()) {
                ("forward", x) => {
                    pos[0] += x;
                    pos[1] += aim * x;
                }
                ("down", a) => aim += a,
                ("up", a) => aim -= a,
                (c, _) => panic!("command {c} doesn't exist"),
            };
            (pos, aim)
        })
        .0
        .iter()
        .product()
}

#[cfg(test)]
mod day2 {

    use super::*;

    const INPUT: &'static str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 150);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 900);
    }
}
