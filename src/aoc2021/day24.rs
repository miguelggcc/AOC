use nom::multi::separated_list1;

pub fn part1(_input: &str) -> String {
    String::from("Not implemented")
}

pub fn part2(_input: &str) -> String {
    String::from("Not implemented")
}

enum Instructions {
    Inp(usize),
    AddVar(usize, usize),
    AddInt(usize, u32),
    MulVar(usize, usize),
    MulInt(usize, u32),
    DivVar(usize, usize),
    DivInt(usize, u32),
    ModVar(usize, usize),
    ModInt(usize, u32),
    EqlVar(usize, usize),
    EqlInt(usize, u32),
}

#[cfg(test)]
mod day24 {

    use super::*;

    const INPUT: &'static str = "inp w\nadd z w\nmod z 2\ndiv w 2
add y w\nmod y 2\ndiv w 2\nadd x w\nmod x 2\ndiv w 2\nmod w 2";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), "");
    }
    #[test]
    #[ignore]
    fn part_2() {
        assert_eq!(part2(INPUT), "");
    }
}
