use nom::multi::separated_list1;

pub fn part1(input: &str) -> u64 {
    for line in input.lines() {
        let mut var = [0; 4];
        let n = 345345345;
        let (inst, rhs) = line.split_once(" ").unwrap();
        match inst {
            "inp" => var[0] = n,
            "add" => {
                let (a, b) = rhs.split_once(" ").unwrap();
                var[get_index(a)] += get_var(b, &var);
            }
            "mul" => {
                let (a, b) = rhs.split_once(" ").unwrap();
                var[get_index(a)] *= get_var(b, &var);
            }
            "div" => {
                let (a, b) = rhs.split_once(" ").unwrap();
                var[get_index(a)] /= get_var(b, &var);
            }
            "mod" => {
                let (a, b) = rhs.split_once(" ").unwrap();
                var[get_index(a)] %= get_var(b, &var);
            }
            "eql" => {
                let (a, b) = rhs.split_once(" ").unwrap();
                var[get_index(a)] = u64::from(var[get_index(a)] == get_var(b, &var));
            }
            e => panic!("Unknown instruction {e}"),
        }
    }
    0
}

pub fn part2(_input: &str) -> String {
    String::from("Not implemented")
}

fn get_var(input: &str, var: &[u64]) -> u64 {
    if let Ok(v) = input.parse::<u64>() {
        return v;
    }
    var[get_index(input)]
}

fn get_index(n: &str) -> usize {
    (n.chars().next().unwrap() as u8 - b'w') as usize
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
        assert_eq!(part1(INPUT), 0);
    }
    #[test]
    #[ignore]
    fn part_2() {
        assert_eq!(part2(INPUT), "");
    }
}
