use itertools::*;

pub fn part1(input: &str) -> String {
    let p = parse(input);
    execute(p, 1).into_iter().map(|n| n.to_string()).collect()
}

pub fn part2(input: &str) -> String {
    let p = parse(input);
    execute(p, 5).into_iter().map(|n| n.to_string()).collect()
}

fn parse(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

const DIGITS: [i32; 2] = [100, 1000];

fn execute(mut p: Vec<i32>, input: i32) -> Vec<i32> {
    let mut output = vec![];
    let mut i = 0;
    loop {
        let (par0, par1) = DIGITS
            .iter()
            .enumerate()
            .map(|(index, d)| {
                if (p[i] / d) % 10 == 1 {
                    i + index + 1
                } else {
                    *p.get(i + index + 1).unwrap_or(&0) as usize
                }
            })
            .collect_tuple()
            .unwrap();

        let opcode = p[i] % 100;
        let o = *p.get(i + 3).unwrap_or(&0) as usize;

        i += match opcode {
            1 => {
                p[o] = p[par0 as usize] + p[par1 as usize];
                4
            }
            2 => {
                p[o] = p[par0 as usize] * p[par1 as usize];
                4
            }
            3 => {
                p[par0 as usize] = input;
                2
            }
            4 => {
                output.push(p[par0 as usize]);
                2
            }
            5 => {
                if p[par0 as usize] != 0 {
                    p[par1 as usize] as usize - i
                } else {
                    3
                }
            }
            6 => {
                if p[par0 as usize] == 0 {
                    p[par1 as usize] as usize - i
                } else {
                    3
                }
            }
            7 => {
                p[o] = i32::from(p[par0 as usize] < p[par1 as usize]);
                4
            }
            8 => {
                p[o] = i32::from(p[par0 as usize] == p[par1 as usize]);
                4
            }
            99 => return output,
            e => panic!("uknown command {e}"),
        };
        assert!(i < p.len());
    }
}
